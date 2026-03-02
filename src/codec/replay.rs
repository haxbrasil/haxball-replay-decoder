use crate::codec::events::parse_event_payload;
use crate::codec::inflate_raw;
use crate::codec::reader::ReplayReader;
use crate::codec::room::parse_room_state;
use crate::error::DecodeError;
use crate::types::{GoalMarker, ReplayData, ReplayEvent};

pub(crate) fn parse_replay(
    bytes: &[u8],
    allow_unknown_event_types: bool,
) -> Result<ReplayData, DecodeError> {
    if bytes.len() < 12 {
        return Err(DecodeError::UnexpectedEof {
            context: "replay header".to_string(),
        });
    }

    let magic = [bytes[0], bytes[1], bytes[2], bytes[3]];
    if magic != *b"HBR2" {
        return Err(DecodeError::InvalidMagic { found: magic });
    }

    let version = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    let total_frames = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);

    let packed = &bytes[12..];
    let inflated = inflate_raw(packed, "replay.packedContents")?;

    let mut reader = ReplayReader::new(&inflated);

    let goal_count = reader.read_u16("replay.goalMarkers.length")? as usize;
    let mut goal_markers = Vec::with_capacity(goal_count);
    let mut goal_frame = 0_u32;
    for index in 0..goal_count {
        let delta = reader.read_var_u32(&format!("replay.goalMarkers[{index}].delta"))?;
        goal_frame = goal_frame
            .checked_add(delta)
            .ok_or_else(|| DecodeError::IntegerOverflow {
                context: format!("replay.goalMarkers[{index}].frameNo"),
            })?;
        goal_markers.push(GoalMarker {
            frame_no: goal_frame,
            team_id: reader.read_u8(&format!("replay.goalMarkers[{index}].teamId"))?,
        });
    }

    let room_data = parse_room_state(&mut reader)?;

    let mut events = Vec::new();
    let mut event_frame = 0_u32;
    while !reader.is_eof() {
        let delta = reader.read_var_u32("replay.events.delta")?;
        event_frame =
            event_frame
                .checked_add(delta)
                .ok_or_else(|| DecodeError::IntegerOverflow {
                    context: "replay.events.frameNo".to_string(),
                })?;

        let by_id = reader.read_u16("replay.events.byId")?;
        let event_type = reader.read_u8("replay.events.eventType")?;
        let payload = parse_event_payload(&mut reader, event_type, allow_unknown_event_types)?;

        events.push(ReplayEvent {
            frame_no: event_frame,
            by_id,
            payload,
        });
    }

    Ok(ReplayData {
        room_data,
        events,
        goal_markers,
        total_frames,
        version,
    })
}
