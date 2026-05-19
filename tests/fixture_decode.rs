use std::collections::HashMap;

use haxball_replay_decoder::{EventPayload, decode, validate};

fn fixture_bytes() -> Vec<u8> {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/recording-01.hbr2");
    std::fs::read(path).expect("fixture replay file should exist")
}

fn emoji_avatar_fixture_bytes() -> Vec<u8> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/headless-avatar-emoji.hbr2");
    std::fs::read(path).expect("emoji avatar fixture replay file should exist")
}

fn headless_avatar_emoji_match_fixture_bytes() -> Vec<u8> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/headless-avatar-emoji-match.hbr2");
    std::fs::read(path).expect("headless avatar emoji match fixture replay file should exist")
}

#[test]
fn decodes_fixture_replay() {
    let bytes = fixture_bytes();
    let replay = decode(&bytes).expect("fixture replay should decode");

    assert_eq!(replay.version, 3);
    assert_eq!(replay.total_frames, 824);
    assert_eq!(replay.goal_markers.len(), 0);
    assert_eq!(replay.events.len(), 89);

    let mut counts: HashMap<&'static str, usize> = HashMap::new();
    for event in &replay.events {
        let key = match &event.payload {
            EventPayload::SendInput(_) => "SendInput",
            EventPayload::Ping(_) => "Ping",
            _ => "Other",
        };
        *counts.entry(key).or_insert(0) += 1;
    }

    assert_eq!(counts.get("SendInput").copied().unwrap_or_default(), 84);
    assert_eq!(counts.get("Ping").copied().unwrap_or_default(), 5);

    let room = &replay.room_data;
    assert_eq!(room.name.as_deref(), Some("gabinho's room"));
    assert_eq!(room.players.len(), 1);

    let stadium = &room.stadium;
    assert_eq!(stadium.default_stadium_id, 255);
    assert_eq!(stadium.name.as_deref(), Some("SBBHax.com [NOVO SITE]"));
    assert_eq!(stadium.vertices.len(), 55);
    assert_eq!(stadium.segments.len(), 25);
    assert_eq!(stadium.planes.len(), 6);
    assert_eq!(stadium.goals.len(), 2);
    assert_eq!(stadium.discs.len(), 39);
}

#[test]
fn fixture_is_valid_in_strict_profile() {
    let bytes = fixture_bytes();
    let report = validate(&bytes);

    assert!(
        report.is_valid(),
        "expected strict validation to pass, issues: {:?}",
        report.issues
    );
}

#[test]
fn decodes_headless_avatar_emoji() {
    let bytes = emoji_avatar_fixture_bytes();
    let replay = decode(&bytes).expect("emoji avatar fixture should decode");

    let avatar = replay.events.iter().find_map(|event| match &event.payload {
        EventPayload::SetHeadlessAvatar(event) => event.value.as_deref(),
        _ => None,
    });

    assert_eq!(avatar, Some("🏈"));
}

#[test]
fn headless_avatar_emoji_fixture_is_valid_in_strict_profile() {
    let bytes = emoji_avatar_fixture_bytes();
    let report = validate(&bytes);

    assert!(
        report.is_valid(),
        "expected strict validation to pass, issues: {:?}",
        report.issues
    );
}

#[test]
fn headless_avatar_emoji_match_fixture_is_valid_in_strict_profile() {
    let bytes = headless_avatar_emoji_match_fixture_bytes();
    let replay = decode(&bytes).expect("headless avatar emoji match fixture should decode");

    assert_eq!(replay.version, 3);
    assert_eq!(replay.total_frames, 2435);

    let report = validate(&bytes);
    assert!(
        report.is_valid(),
        "expected strict validation to pass, issues: {:?}",
        report.issues
    );
}
