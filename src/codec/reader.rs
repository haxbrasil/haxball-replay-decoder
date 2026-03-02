use crate::error::DecodeError;

pub(crate) struct ReplayReader<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> ReplayReader<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }

    pub(crate) fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.cursor)
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.remaining() == 0
    }

    pub(crate) fn ensure_eof(&self, context: &str) -> Result<(), DecodeError> {
        if self.is_eof() {
            Ok(())
        } else {
            Err(DecodeError::TrailingBytes {
                context: context.to_string(),
                remaining: self.remaining(),
            })
        }
    }

    pub(crate) fn read_u8(&mut self, context: &str) -> Result<u8, DecodeError> {
        if self.remaining() < 1 {
            return Err(DecodeError::UnexpectedEof {
                context: context.to_string(),
            });
        }
        let value = self.bytes[self.cursor];
        self.cursor += 1;
        Ok(value)
    }

    pub(crate) fn read_i8(&mut self, context: &str) -> Result<i8, DecodeError> {
        Ok(self.read_u8(context)? as i8)
    }

    pub(crate) fn read_i16(&mut self, context: &str) -> Result<i16, DecodeError> {
        let bytes = self.read_bytes(2, context)?;
        let value = i16::from_be_bytes([bytes[0], bytes[1]]);
        Ok(value)
    }

    pub(crate) fn read_u16(&mut self, context: &str) -> Result<u16, DecodeError> {
        let bytes = self.read_bytes(2, context)?;
        let value = u16::from_be_bytes([bytes[0], bytes[1]]);
        Ok(value)
    }

    pub(crate) fn read_i32(&mut self, context: &str) -> Result<i32, DecodeError> {
        let bytes = self.read_bytes(4, context)?;
        let value = i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        Ok(value)
    }

    pub(crate) fn read_u32(&mut self, context: &str) -> Result<u32, DecodeError> {
        let bytes = self.read_bytes(4, context)?;
        let value = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        Ok(value)
    }

    pub(crate) fn read_f32(&mut self, context: &str) -> Result<f32, DecodeError> {
        let bits = self.read_u32(context)?;
        Ok(f32::from_bits(bits))
    }

    pub(crate) fn read_f64(&mut self, context: &str) -> Result<f64, DecodeError> {
        let bytes = self.read_bytes(8, context)?;
        let bits = u64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]);
        Ok(f64::from_bits(bits))
    }

    pub(crate) fn read_var_u32(&mut self, context: &str) -> Result<u32, DecodeError> {
        let mut value: u32 = 0;
        let mut shift = 0;

        for byte_index in 0..5 {
            let byte = self.read_u8(context)?;
            let low = (byte & 0x7f) as u32;

            if byte_index == 4 && (byte & 0xf0) != 0 {
                return Err(DecodeError::InvalidVarInt {
                    context: context.to_string(),
                });
            }

            value |= low << shift;

            if (byte & 0x80) == 0 {
                return Ok(value);
            }

            shift += 7;
        }

        Err(DecodeError::InvalidVarInt {
            context: context.to_string(),
        })
    }

    pub(crate) fn read_bytes(
        &mut self,
        length: usize,
        context: &str,
    ) -> Result<&'a [u8], DecodeError> {
        if self.remaining() < length {
            return Err(DecodeError::UnexpectedEof {
                context: context.to_string(),
            });
        }
        let start = self.cursor;
        let end = start + length;
        self.cursor = end;
        Ok(&self.bytes[start..end])
    }

    pub(crate) fn read_bytes_vec(
        &mut self,
        length: usize,
        context: &str,
    ) -> Result<Vec<u8>, DecodeError> {
        Ok(self.read_bytes(length, context)?.to_vec())
    }

    pub(crate) fn read_remaining_vec(&mut self) -> Vec<u8> {
        let start = self.cursor;
        self.cursor = self.bytes.len();
        self.bytes[start..].to_vec()
    }

    pub(crate) fn read_string(&mut self, context: &str) -> Result<String, DecodeError> {
        let length = self.read_var_u32(context)? as usize;
        let bytes = self.read_bytes(length, context)?;
        let value = std::str::from_utf8(bytes).map_err(|source| DecodeError::InvalidUtf8 {
            context: context.to_string(),
            source,
        })?;
        Ok(value.to_string())
    }

    pub(crate) fn read_nullable_string(
        &mut self,
        context: &str,
    ) -> Result<Option<String>, DecodeError> {
        let length = self.read_var_u32(context)? as usize;
        if length == 0 {
            return Ok(None);
        }

        let bytes = self.read_bytes(length - 1, context)?;
        let value = std::str::from_utf8(bytes).map_err(|source| DecodeError::InvalidUtf8 {
            context: context.to_string(),
            source,
        })?;

        Ok(Some(value.to_string()))
    }

    pub(crate) fn read_json(&mut self, context: &str) -> Result<serde_json::Value, DecodeError> {
        let string = self.read_string(context)?;
        serde_json::from_str(&string).map_err(|source| DecodeError::InvalidJson {
            context: context.to_string(),
            source,
        })
    }
}
