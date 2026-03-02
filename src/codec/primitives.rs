use crate::codec::reader::ReplayReader;
use crate::error::DecodeError;
use crate::types::{Disc, Goal, Joint, Plane, PlayerPhysics, Point, Segment, TeamColors, Vertex};

pub(crate) fn parse_point(
    reader: &mut ReplayReader<'_>,
    context: &str,
) -> Result<Point, DecodeError> {
    Ok(Point {
        x: reader.read_f64(&format!("{context}.x"))?,
        y: reader.read_f64(&format!("{context}.y"))?,
    })
}

pub(crate) fn parse_vertex(
    reader: &mut ReplayReader<'_>,
    context: &str,
) -> Result<Vertex, DecodeError> {
    Ok(Vertex {
        pos: parse_point(reader, &format!("{context}.pos"))?,
        b_coef: reader.read_f64(&format!("{context}.bCoef"))?,
        c_mask: reader.read_i32(&format!("{context}.cMask"))?,
        c_group: reader.read_i32(&format!("{context}.cGroup"))?,
    })
}

pub(crate) fn parse_segment(
    reader: &mut ReplayReader<'_>,
    context: &str,
) -> Result<Segment, DecodeError> {
    let flags = reader.read_u8(&format!("{context}.flags"))?;
    let v0 = reader.read_u8(&format!("{context}.v0"))?;
    let v1 = reader.read_u8(&format!("{context}.v1"))?;

    let bias = if (flags & 1) != 0 {
        reader.read_f64(&format!("{context}.bias"))?
    } else {
        0.0
    };

    let curve = if (flags & 2) != 0 {
        reader.read_f64(&format!("{context}.curve"))?
    } else {
        f64::INFINITY
    };

    let color = if (flags & 4) != 0 {
        reader.read_i32(&format!("{context}.color"))?
    } else {
        0
    };

    let vis = (flags & 8) != 0;

    Ok(Segment {
        flags,
        v0,
        v1,
        bias,
        curve,
        color,
        vis,
        b_coef: reader.read_f64(&format!("{context}.bCoef"))?,
        c_mask: reader.read_i32(&format!("{context}.cMask"))?,
        c_group: reader.read_i32(&format!("{context}.cGroup"))?,
    })
}

pub(crate) fn parse_plane(
    reader: &mut ReplayReader<'_>,
    context: &str,
) -> Result<Plane, DecodeError> {
    Ok(Plane {
        normal: parse_point(reader, &format!("{context}.normal"))?,
        dist: reader.read_f64(&format!("{context}.dist"))?,
        b_coef: reader.read_f64(&format!("{context}.bCoef"))?,
        c_mask: reader.read_i32(&format!("{context}.cMask"))?,
        c_group: reader.read_i32(&format!("{context}.cGroup"))?,
    })
}

pub(crate) fn parse_goal(
    reader: &mut ReplayReader<'_>,
    context: &str,
) -> Result<Goal, DecodeError> {
    Ok(Goal {
        p0: parse_point(reader, &format!("{context}.p0"))?,
        p1: parse_point(reader, &format!("{context}.p1"))?,
        team_id: reader.read_i8(&format!("{context}.teamId"))?,
    })
}

pub(crate) fn parse_disc(
    reader: &mut ReplayReader<'_>,
    context: &str,
) -> Result<Disc, DecodeError> {
    Ok(Disc {
        pos: parse_point(reader, &format!("{context}.pos"))?,
        speed: parse_point(reader, &format!("{context}.speed"))?,
        gravity: parse_point(reader, &format!("{context}.gravity"))?,
        radius: reader.read_f64(&format!("{context}.radius"))?,
        b_coef: reader.read_f64(&format!("{context}.bCoef"))?,
        inv_mass: reader.read_f64(&format!("{context}.invMass"))?,
        damping: reader.read_f64(&format!("{context}.damping"))?,
        color: reader.read_u32(&format!("{context}.color"))?,
        c_mask: reader.read_i32(&format!("{context}.cMask"))?,
        c_group: reader.read_i32(&format!("{context}.cGroup"))?,
    })
}

pub(crate) fn parse_joint(
    reader: &mut ReplayReader<'_>,
    context: &str,
) -> Result<Joint, DecodeError> {
    Ok(Joint {
        d0: reader.read_u8(&format!("{context}.d0"))?,
        d1: reader.read_u8(&format!("{context}.d1"))?,
        min_length: reader.read_f64(&format!("{context}.minLength"))?,
        max_length: reader.read_f64(&format!("{context}.maxLength"))?,
        strength: reader.read_f64(&format!("{context}.strength"))?,
        color: reader.read_i32(&format!("{context}.color"))?,
    })
}

pub(crate) fn parse_player_physics(
    reader: &mut ReplayReader<'_>,
    context: &str,
) -> Result<PlayerPhysics, DecodeError> {
    Ok(PlayerPhysics {
        b_coef: reader.read_f64(&format!("{context}.bCoef"))?,
        inv_mass: reader.read_f64(&format!("{context}.invMass"))?,
        damping: reader.read_f64(&format!("{context}.damping"))?,
        acceleration: reader.read_f64(&format!("{context}.acceleration"))?,
        kicking_acceleration: reader.read_f64(&format!("{context}.kickingAcceleration"))?,
        kicking_damping: reader.read_f64(&format!("{context}.kickingDamping"))?,
        kick_strength: reader.read_f64(&format!("{context}.kickStrength"))?,
        gravity: parse_point(reader, &format!("{context}.gravity"))?,
        c_group: reader.read_i32(&format!("{context}.cGroup"))?,
        radius: reader.read_f64(&format!("{context}.radius"))?,
        kickback: reader.read_f64(&format!("{context}.kickback"))?,
    })
}

pub(crate) fn parse_team_colors(
    reader: &mut ReplayReader<'_>,
    context: &str,
) -> Result<TeamColors, DecodeError> {
    let angle = reader.read_u8(&format!("{context}.angle"))?;
    let text = reader.read_i32(&format!("{context}.text"))?;
    let color_count = reader.read_u8(&format!("{context}.innerCount"))? as usize;

    if color_count > 3 {
        return Err(DecodeError::InvalidVarInt {
            context: format!("{context}.innerCount"),
        });
    }

    let mut inner = Vec::with_capacity(color_count);
    for index in 0..color_count {
        inner.push(reader.read_i32(&format!("{context}.inner[{index}]"))?);
    }

    Ok(TeamColors { angle, text, inner })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_varint_boundaries() {
        let bytes = [0x00, 0x7f, 0x80, 0x01, 0xff, 0xff, 0xff, 0xff, 0x0f];
        let mut reader = ReplayReader::new(&bytes);

        assert_eq!(reader.read_var_u32("v0").unwrap(), 0);
        assert_eq!(reader.read_var_u32("v1").unwrap(), 127);
        assert_eq!(reader.read_var_u32("v2").unwrap(), 128);
        assert_eq!(reader.read_var_u32("v3").unwrap(), u32::MAX);
    }

    #[test]
    fn parse_nullable_string() {
        let bytes = [0x00, 0x04, b't', b'e', b's'];
        let mut reader = ReplayReader::new(&bytes);

        assert_eq!(reader.read_nullable_string("nullable").unwrap(), None);
        assert_eq!(
            reader.read_nullable_string("nullable").unwrap(),
            Some("tes".to_string())
        );
    }

    #[test]
    fn parse_team_colors_fixture() {
        let bytes = [12, 0, 0, 0, 1, 2, 0, 0, 0, 2, 0, 0, 0, 3];
        let mut reader = ReplayReader::new(&bytes);
        let colors = parse_team_colors(&mut reader, "team").unwrap();

        assert_eq!(colors.angle, 12);
        assert_eq!(colors.text, 1);
        assert_eq!(colors.inner, vec![2, 3]);
    }
}
