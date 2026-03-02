use std::io::{Read, Write};

use flate2::{Compression, read::DeflateDecoder, write::DeflateEncoder};
use haxball_replay_decoder::{DecodeError, decode};

const FIRST_EVENT_OFFSET: usize = 11_465;

fn fixture_bytes() -> Vec<u8> {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/recording-01.hbr2");
    std::fs::read(path).expect("fixture replay file should exist")
}

fn inflate_payload(bytes: &[u8]) -> Vec<u8> {
    let mut decoder = DeflateDecoder::new(&bytes[12..]);
    let mut output = Vec::new();
    decoder
        .read_to_end(&mut output)
        .expect("fixture payload should be valid raw deflate");
    output
}

fn rebuild_with_inflated_payload(header: &[u8], inflated: &[u8]) -> Vec<u8> {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(inflated)
        .expect("deflate encoding should work");
    let compressed = encoder.finish().expect("deflate finish should work");

    let mut rebuilt = Vec::with_capacity(12 + compressed.len());
    rebuilt.extend_from_slice(header);
    rebuilt.extend_from_slice(&compressed);
    rebuilt
}

fn mutate_inflated<F>(bytes: &[u8], mutator: F) -> Vec<u8>
where
    F: FnOnce(&mut Vec<u8>),
{
    let header = &bytes[..12];
    let mut inflated = inflate_payload(bytes);
    mutator(&mut inflated);
    rebuild_with_inflated_payload(header, &inflated)
}

#[test]
fn rejects_bad_magic() {
    let mut bytes = fixture_bytes();
    bytes[0] = b'X';

    let error = decode(&bytes).expect_err("bad magic should fail");
    assert!(matches!(error, DecodeError::InvalidMagic { .. }));
}

#[test]
fn rejects_unsupported_version() {
    let mut bytes = fixture_bytes();
    bytes[4..8].copy_from_slice(&4_u32.to_be_bytes());

    let error = decode(&bytes).expect_err("unsupported version should fail");
    assert!(matches!(error, DecodeError::UnsupportedReplayVersion(4)));
}

#[test]
fn rejects_truncated_file() {
    let mut bytes = fixture_bytes();
    bytes.truncate(28);

    assert!(decode(&bytes).is_err(), "truncated replay should fail");
}

#[test]
fn rejects_invalid_deflate_payload() {
    let mut bytes = fixture_bytes();
    bytes[12] = 0xff;

    let error = decode(&bytes).expect_err("invalid deflate payload should fail");
    assert!(matches!(
        error,
        DecodeError::Compression { .. }
            | DecodeError::IncompleteCompression { .. }
            | DecodeError::TrailingCompressedData { .. }
    ));
}

#[test]
fn rejects_invalid_varint_or_string_encoding() {
    let bytes = fixture_bytes();
    let bytes = mutate_inflated(&bytes, |inflated| {
        // roomState.name nullable string length varint
        inflated[2] = 0xff;
    });

    assert!(decode(&bytes).is_err(), "invalid string/varint should fail");
}

#[test]
fn rejects_truncated_nested_payload() {
    let bytes = fixture_bytes();
    let bytes = mutate_inflated(&bytes, |inflated| {
        // mutate first event from Ping(17) to SetStadium(11), then claim huge nested payload.
        assert_eq!(inflated[FIRST_EVENT_OFFSET + 3], 17);
        inflated[FIRST_EVENT_OFFSET + 3] = 11;
        inflated[FIRST_EVENT_OFFSET + 4] = 0xff;
        inflated[FIRST_EVENT_OFFSET + 5] = 0xff;
    });

    let error = decode(&bytes).expect_err("truncated nested payload should fail");
    assert!(matches!(error, DecodeError::UnexpectedEof { .. }));
}

#[test]
fn rejects_non_monotonic_or_overflowed_frame_deltas() {
    let bytes = fixture_bytes();
    let bytes = mutate_inflated(&bytes, |inflated| {
        // Replace first event frame delta(3) with u32::MAX to force overflow on next event.
        assert_eq!(inflated[FIRST_EVENT_OFFSET], 0x03);
        inflated.splice(
            FIRST_EVENT_OFFSET..(FIRST_EVENT_OFFSET + 1),
            [0xff, 0xff, 0xff, 0xff, 0x0f],
        );
    });

    let error = decode(&bytes).expect_err("overflowed frame deltas should fail");
    assert!(matches!(
        error,
        DecodeError::IntegerOverflow { .. } | DecodeError::ValidationFailed(_)
    ));
}
