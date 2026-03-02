mod codec;
mod error;
mod options;
mod types;
mod validate;

pub use error::DecodeError;
pub use options::{DecodeOptions, ValidationProfile};
pub use types::*;
pub use validate::{ValidationIssue, ValidationReport, ValidationSeverity};

pub fn decode(bytes: &[u8]) -> Result<ReplayData, DecodeError> {
    decode_with_options(bytes, DecodeOptions::default())
}

pub fn decode_with_options(
    bytes: &[u8],
    options: DecodeOptions,
) -> Result<ReplayData, DecodeError> {
    let replay = codec::replay::parse_replay(bytes, options.allow_unknown_event_types)?;

    if replay.version != 3 {
        return Err(DecodeError::UnsupportedReplayVersion(replay.version));
    }

    let report = validate::rules::validate_replay_data(&replay, options.validation_profile);
    if !report.is_valid() {
        return Err(DecodeError::ValidationFailed(Box::new(report)));
    }

    Ok(replay)
}

pub fn validate(bytes: &[u8]) -> ValidationReport {
    validate_with_profile(bytes, ValidationProfile::Strict)
}

pub fn validate_with_profile(bytes: &[u8], profile: ValidationProfile) -> ValidationReport {
    match codec::replay::parse_replay(bytes, true) {
        Ok(replay) => validate::rules::validate_replay_data(&replay, profile),
        Err(error) => {
            let mut report = ValidationReport::new(profile);
            report.push(validate::rules::issue_from_decode_error(profile, &error));
            report
        }
    }
}
