use std::io;

use thiserror::Error;

use crate::validate::ValidationReport;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("invalid replay signature: expected HBR2, got {found:?}")]
    InvalidMagic { found: [u8; 4] },

    #[error("truncated input while reading {context}")]
    UnexpectedEof { context: String },

    #[error("invalid varint while reading {context}")]
    InvalidVarInt { context: String },

    #[error("invalid UTF-8 while reading {context}: {source}")]
    InvalidUtf8 {
        context: String,
        #[source]
        source: std::str::Utf8Error,
    },

    #[error("invalid JSON while reading {context}: {source}")]
    InvalidJson {
        context: String,
        #[source]
        source: serde_json::Error,
    },

    #[error("raw deflate error while reading {context}: {source}")]
    Compression {
        context: String,
        #[source]
        source: io::Error,
    },

    #[error("compressed stream did not terminate cleanly while reading {context}")]
    IncompleteCompression { context: String },

    #[error("compressed stream contains trailing bytes while reading {context}")]
    TrailingCompressedData { context: String },

    #[error("unsupported replay version: {0}")]
    UnsupportedReplayVersion(u32),

    #[error("unsupported event type: {0}")]
    UnsupportedEventType(u8),

    #[error("cannot preserve unknown event type {event_type} without a payload boundary")]
    UnknownEventBoundaryUnsupported { event_type: u8 },

    #[error("integer overflow while computing {context}")]
    IntegerOverflow { context: String },

    #[error("trailing unread bytes while reading {context}: {remaining}")]
    TrailingBytes { context: String, remaining: usize },

    #[error("validation failed")]
    ValidationFailed(Box<ValidationReport>),
}
