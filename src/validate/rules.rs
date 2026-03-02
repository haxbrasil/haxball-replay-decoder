use crate::error::DecodeError;
use crate::options::ValidationProfile;
use crate::types::{
    BackgroundType, CameraFollow, EventPayload, GamePlayState, ReplayData, ReplayEvent,
};
use crate::validate::{ValidationIssue, ValidationReport, ValidationSeverity, issue};

pub(crate) fn validate_replay_data(
    replay: &ReplayData,
    profile: ValidationProfile,
) -> ValidationReport {
    let mut report = ValidationReport::new(profile);

    if replay.version != 3 {
        let severity = match profile {
            ValidationProfile::Strict => ValidationSeverity::Error,
            ValidationProfile::Structural => ValidationSeverity::Warning,
        };
        report.push(issue(
            "UNSUPPORTED_VERSION",
            severity,
            "version",
            format!(
                "only replay version 3 is supported in v1, got {}",
                replay.version
            ),
        ));
    }

    if profile == ValidationProfile::Structural {
        return report;
    }

    validate_goal_markers(replay, &mut report);
    validate_events(replay, &mut report);
    validate_room(replay, &mut report);

    report
}

pub(crate) fn issue_from_decode_error(
    profile: ValidationProfile,
    error: &DecodeError,
) -> ValidationIssue {
    issue(
        "DECODE_ERROR",
        ValidationSeverity::Error,
        "/",
        format!("{profile:?} validation failed during decoding: {error}"),
    )
}

fn validate_goal_markers(replay: &ReplayData, report: &mut ValidationReport) {
    let mut previous_frame = 0_u32;

    for (index, marker) in replay.goal_markers.iter().enumerate() {
        if marker.frame_no < previous_frame {
            report.push(issue(
                "GOAL_FRAME_NOT_MONOTONIC",
                ValidationSeverity::Error,
                format!("goalMarkers[{index}].frameNo"),
                "goal marker frame numbers must be monotonic non-decreasing",
            ));
        }

        if marker.frame_no > replay.total_frames {
            report.push(issue(
                "GOAL_FRAME_OUT_OF_BOUNDS",
                ValidationSeverity::Error,
                format!("goalMarkers[{index}].frameNo"),
                format!(
                    "goal marker frame {} exceeds total frames {}",
                    marker.frame_no, replay.total_frames
                ),
            ));
        }

        if !is_valid_team_u8(marker.team_id) {
            report.push(issue(
                "INVALID_TEAM_ID",
                ValidationSeverity::Error,
                format!("goalMarkers[{index}].teamId"),
                format!(
                    "team id {} is outside expected range [0, 2]",
                    marker.team_id
                ),
            ));
        }

        previous_frame = marker.frame_no;
    }
}

fn validate_events(replay: &ReplayData, report: &mut ValidationReport) {
    let mut previous_frame = 0_u32;

    for (index, event) in replay.events.iter().enumerate() {
        validate_event_frame(replay, report, index, event, &mut previous_frame);
        validate_event_payload(report, index, event);
    }
}

fn validate_event_frame(
    replay: &ReplayData,
    report: &mut ValidationReport,
    index: usize,
    event: &ReplayEvent,
    previous_frame: &mut u32,
) {
    if event.frame_no < *previous_frame {
        report.push(issue(
            "EVENT_FRAME_NOT_MONOTONIC",
            ValidationSeverity::Error,
            format!("events[{index}].frameNo"),
            "event frame numbers must be monotonic non-decreasing",
        ));
    }

    if event.frame_no > replay.total_frames {
        report.push(issue(
            "EVENT_FRAME_OUT_OF_BOUNDS",
            ValidationSeverity::Error,
            format!("events[{index}].frameNo"),
            format!(
                "event frame {} exceeds total frames {}",
                event.frame_no, replay.total_frames
            ),
        ));
    }

    *previous_frame = event.frame_no;
}

fn validate_event_payload(report: &mut ValidationReport, index: usize, event: &ReplayEvent) {
    match &event.payload {
        EventPayload::SetPlayerTeam(payload) => {
            if !is_valid_team_i8(payload.team_id) {
                report.push(issue(
                    "INVALID_TEAM_ID",
                    ValidationSeverity::Error,
                    format!("events[{index}].payload.teamId"),
                    format!(
                        "team id {} is outside expected range [0, 2]",
                        payload.team_id
                    ),
                ));
            }
        }
        EventPayload::SetTeamColors(payload) => {
            if !is_valid_team_i8(payload.team_id) {
                report.push(issue(
                    "INVALID_TEAM_ID",
                    ValidationSeverity::Error,
                    format!("events[{index}].payload.teamId"),
                    format!(
                        "team id {} is outside expected range [0, 2]",
                        payload.team_id
                    ),
                ));
            }
        }
        EventPayload::SetDiscProperties(payload) => {
            if (payload.flags & !0x1fff) != 0 {
                report.push(issue(
                    "INVALID_DISC_PROPERTIES_FLAGS",
                    ValidationSeverity::Error,
                    format!("events[{index}].payload.flags"),
                    format!(
                        "unexpected flag bits are set: 0x{:04x}",
                        payload.flags & !0x1fff
                    ),
                ));
            }
        }
        EventPayload::Unknown { event_type, .. } => {
            report.push(issue(
                "UNKNOWN_EVENT_TYPE",
                ValidationSeverity::Warning,
                format!("events[{index}].eventType"),
                format!("unknown event type {event_type} was preserved as raw payload"),
            ));
        }
        _ => {}
    }
}

fn validate_room(replay: &ReplayData, report: &mut ValidationReport) {
    let room = &replay.room_data;

    if let Some(state) = &room.game_state {
        if let GamePlayState::Unknown(value) = state.state {
            report.push(issue(
                "UNKNOWN_GAME_STATE",
                ValidationSeverity::Error,
                "roomData.gameState.state",
                format!("unknown game play state value {value}"),
            ));
        }

        if !is_valid_team_u8(state.goal_conceding_team) {
            report.push(issue(
                "INVALID_TEAM_ID",
                ValidationSeverity::Error,
                "roomData.gameState.goalConcedingTeam",
                format!(
                    "team id {} is outside expected range [0, 2]",
                    state.goal_conceding_team
                ),
            ));
        }

        for (index, player) in room.players.iter().enumerate() {
            if player.disc_index != -1 {
                if player.disc_index < -1 || player.disc_index as usize >= state.discs.len() {
                    report.push(issue(
                        "INVALID_PLAYER_DISC_INDEX",
                        ValidationSeverity::Error,
                        format!("roomData.players[{index}].discIndex"),
                        format!(
                            "disc index {} is invalid for game state with {} discs",
                            player.disc_index,
                            state.discs.len()
                        ),
                    ));
                }
            }
        }
    }

    for (index, player) in room.players.iter().enumerate() {
        if !is_valid_team_u8(player.team_id) {
            report.push(issue(
                "INVALID_TEAM_ID",
                ValidationSeverity::Error,
                format!("roomData.players[{index}].teamId"),
                format!(
                    "team id {} is outside expected range [0, 2]",
                    player.team_id
                ),
            ));
        }
    }

    if let Some(background_type) = room.stadium.background_type {
        if let BackgroundType::Unknown(value) = background_type {
            report.push(issue(
                "UNKNOWN_BACKGROUND_TYPE",
                ValidationSeverity::Error,
                "roomData.stadium.backgroundType",
                format!("unknown background type value {value}"),
            ));
        }
    }

    if let Some(camera_follow) = room.stadium.camera_follow {
        if let CameraFollow::Unknown(value) = camera_follow {
            report.push(issue(
                "UNKNOWN_CAMERA_FOLLOW",
                ValidationSeverity::Error,
                "roomData.stadium.cameraFollow",
                format!("unknown camera follow value {value}"),
            ));
        }
    }
}

fn is_valid_team_u8(value: u8) -> bool {
    value <= 2
}

fn is_valid_team_i8(value: i8) -> bool {
    (0..=2).contains(&value)
}
