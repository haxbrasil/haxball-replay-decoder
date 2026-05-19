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
        decode_haxball_string(bytes, context)
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
        Ok(Some(decode_haxball_string(bytes, context)?))
    }

    pub(crate) fn read_json(&mut self, context: &str) -> Result<serde_json::Value, DecodeError> {
        let string = self.read_string(context)?;
        serde_json::from_str(&string).map_err(|source| DecodeError::InvalidJson {
            context: context.to_string(),
            source,
        })
    }
}

fn decode_haxball_string(bytes: &[u8], context: &str) -> Result<String, DecodeError> {
    match std::str::from_utf8(bytes) {
        Ok(value) => Ok(value.to_string()),
        // HaxBall replays are produced by JavaScript code and can store emoji as
        // UTF-8-encoded UTF-16 surrogate pairs. Normalize that CESU-8 form here.
        Err(source) => decode_cesu8_string(bytes).map_err(|_| DecodeError::InvalidUtf8 {
            context: context.to_string(),
            source,
        }),
    }
}

fn decode_cesu8_string(bytes: &[u8]) -> Result<String, ()> {
    let mut output = String::new();
    let mut cursor = 0;

    while cursor < bytes.len() {
        let code_unit = read_cesu8_code_unit(bytes, &mut cursor)?;

        if (0xd800..=0xdbff).contains(&code_unit) {
            let low = read_cesu8_code_unit(bytes, &mut cursor)?;
            if !(0xdc00..=0xdfff).contains(&low) {
                return Err(());
            }

            let scalar = 0x10000 + (((code_unit - 0xd800) << 10) | (low - 0xdc00));
            output.push(char::from_u32(scalar).ok_or(())?);
        } else if (0xdc00..=0xdfff).contains(&code_unit) {
            return Err(());
        } else {
            output.push(char::from_u32(code_unit).ok_or(())?);
        }
    }

    Ok(output)
}

fn read_cesu8_code_unit(bytes: &[u8], cursor: &mut usize) -> Result<u32, ()> {
    let first = read_byte(bytes, cursor)?;

    if first < 0x80 {
        return Ok(first as u32);
    }

    if (0xc2..=0xdf).contains(&first) {
        let second = read_continuation_byte(bytes, cursor)?;
        return Ok((((first & 0x1f) as u32) << 6) | second as u32);
    }

    if (0xe0..=0xef).contains(&first) {
        let second = read_continuation_byte(bytes, cursor)?;
        let third = read_continuation_byte(bytes, cursor)?;

        if first == 0xe0 && second < 0x20 {
            return Err(());
        }

        return Ok((((first & 0x0f) as u32) << 12) | ((second as u32) << 6) | third as u32);
    }

    if (0xf0..=0xf4).contains(&first) {
        let second = read_continuation_byte(bytes, cursor)?;
        let third = read_continuation_byte(bytes, cursor)?;
        let fourth = read_continuation_byte(bytes, cursor)?;

        if first == 0xf0 && second < 0x10 {
            return Err(());
        }
        if first == 0xf4 && second > 0x0f {
            return Err(());
        }

        return Ok((((first & 0x07) as u32) << 18)
            | ((second as u32) << 12)
            | ((third as u32) << 6)
            | fourth as u32);
    }

    Err(())
}

fn read_byte(bytes: &[u8], cursor: &mut usize) -> Result<u8, ()> {
    let byte = bytes.get(*cursor).copied().ok_or(())?;
    *cursor += 1;
    Ok(byte)
}

fn read_continuation_byte(bytes: &[u8], cursor: &mut usize) -> Result<u8, ()> {
    let byte = read_byte(bytes, cursor)?;
    if (byte & 0xc0) != 0x80 {
        return Err(());
    }

    Ok(byte & 0x3f)
}

#[cfg(test)]
mod tests {
    use super::decode_cesu8_string;

    #[test]
    fn decodes_cesu8_surrogate_pairs() {
        assert_eq!(
            decode_cesu8_string(&[0xed, 0xa0, 0xbc, 0xed, 0xbf, 0x88]).unwrap(),
            "🏈"
        );
        assert_eq!(
            decode_cesu8_string(&[0xed, 0xa0, 0xbd, 0xed, 0xb8, 0x81]).unwrap(),
            "😁"
        );
    }

    #[test]
    fn rejects_unpaired_cesu8_surrogates() {
        assert!(decode_cesu8_string(&[0xed, 0xa0, 0xbc]).is_err());
        assert!(decode_cesu8_string(&[0xed, 0xbf, 0x88]).is_err());
    }
}
