pub mod events;
pub mod primitives;
pub mod reader;
pub mod replay;
pub mod room;
pub mod stadium;

use std::io;

use flate2::{Decompress, FlushDecompress, Status};

use crate::error::DecodeError;

pub(crate) fn inflate_raw(input: &[u8], context: &'static str) -> Result<Vec<u8>, DecodeError> {
    let mut decompressor = Decompress::new(false);
    let mut remaining_input = input;
    let mut output = Vec::new();
    let mut chunk = [0_u8; 16 * 1024];

    loop {
        let input_before = decompressor.total_in();
        let output_before = decompressor.total_out();

        let status = decompressor
            .decompress(remaining_input, &mut chunk, FlushDecompress::None)
            .map_err(|err| DecodeError::Compression {
                context: context.to_string(),
                source: io::Error::new(io::ErrorKind::InvalidData, err),
            })?;

        let consumed_input = (decompressor.total_in() - input_before) as usize;
        let produced_output = (decompressor.total_out() - output_before) as usize;

        if produced_output > 0 {
            output.extend_from_slice(&chunk[..produced_output]);
        }

        if consumed_input > remaining_input.len() {
            return Err(DecodeError::Compression {
                context: context.to_string(),
                source: io::Error::new(io::ErrorKind::InvalidData, "invalid input accounting"),
            });
        }
        remaining_input = &remaining_input[consumed_input..];

        match status {
            Status::StreamEnd => {
                if !remaining_input.is_empty() {
                    return Err(DecodeError::TrailingCompressedData {
                        context: context.to_string(),
                    });
                }
                return Ok(output);
            }
            Status::Ok | Status::BufError => {
                if consumed_input == 0 && produced_output == 0 {
                    return Err(DecodeError::IncompleteCompression {
                        context: context.to_string(),
                    });
                }
            }
        }
    }
}
