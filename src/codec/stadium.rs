use crate::codec::primitives::{
    parse_disc, parse_goal, parse_joint, parse_plane, parse_player_physics, parse_point,
    parse_segment, parse_vertex,
};
use crate::codec::reader::ReplayReader;
use crate::error::DecodeError;
use crate::types::{BackgroundType, CameraFollow, Stadium};

pub(crate) fn parse_stadium(reader: &mut ReplayReader<'_>) -> Result<Stadium, DecodeError> {
    let default_stadium_id = reader.read_u8("stadium.defaultStadiumId")?;

    let mut stadium = Stadium {
        default_stadium_id,
        name: default_stadium_name(default_stadium_id).map(ToString::to_string),
        background_type: None,
        background_width: None,
        background_height: None,
        background_kickoff_radius: None,
        background_corner_radius: None,
        background_goal_line: None,
        background_color: None,
        width: None,
        height: None,
        spawn_distance: None,
        player_physics: None,
        max_view_width: None,
        camera_follow: None,
        can_be_stored: None,
        full_kickoff_reset: None,
        vertices: Vec::new(),
        segments: Vec::new(),
        planes: Vec::new(),
        goals: Vec::new(),
        discs: Vec::new(),
        joints: Vec::new(),
        red_spawn_points: Vec::new(),
        blue_spawn_points: Vec::new(),
    };

    if default_stadium_id != 255 {
        return Ok(stadium);
    }

    stadium.name = reader.read_nullable_string("stadium.name")?;

    let bg_type = reader.read_i32("stadium.bgType")?;
    stadium.background_type = Some(BackgroundType::from_i32(bg_type));

    stadium.background_width = Some(reader.read_f64("stadium.bgWidth")?);
    stadium.background_height = Some(reader.read_f64("stadium.bgHeight")?);
    stadium.background_kickoff_radius = Some(reader.read_f64("stadium.bgKickOffRadius")?);
    stadium.background_corner_radius = Some(reader.read_f64("stadium.bgCornerRadius")?);
    stadium.background_goal_line = Some(reader.read_f64("stadium.bgGoalLine")?);
    stadium.background_color = Some(reader.read_i32("stadium.bgColor")?);

    stadium.width = Some(reader.read_f64("stadium.width")?);
    stadium.height = Some(reader.read_f64("stadium.height")?);
    stadium.spawn_distance = Some(reader.read_f64("stadium.spawnDistance")?);

    stadium.player_physics = Some(parse_player_physics(reader, "stadium.playerPhysics")?);
    stadium.max_view_width = Some(reader.read_u16("stadium.maxViewWidth")?);
    stadium.camera_follow = Some(CameraFollow::from_u8(
        reader.read_u8("stadium.cameraFollow")?,
    ));
    stadium.can_be_stored = Some(reader.read_u8("stadium.canBeStored")? != 0);
    stadium.full_kickoff_reset = Some(reader.read_u8("stadium.fullKickOffReset")? != 0);

    let vertex_count = reader.read_u8("stadium.vertices.length")? as usize;
    stadium.vertices.reserve(vertex_count);
    for index in 0..vertex_count {
        stadium
            .vertices
            .push(parse_vertex(reader, &format!("stadium.vertices[{index}]"))?);
    }

    let segment_count = reader.read_u8("stadium.segments.length")? as usize;
    stadium.segments.reserve(segment_count);
    for index in 0..segment_count {
        stadium.segments.push(parse_segment(
            reader,
            &format!("stadium.segments[{index}]"),
        )?);
    }

    let plane_count = reader.read_u8("stadium.planes.length")? as usize;
    stadium.planes.reserve(plane_count);
    for index in 0..plane_count {
        stadium
            .planes
            .push(parse_plane(reader, &format!("stadium.planes[{index}]"))?);
    }

    let goal_count = reader.read_u8("stadium.goals.length")? as usize;
    stadium.goals.reserve(goal_count);
    for index in 0..goal_count {
        stadium
            .goals
            .push(parse_goal(reader, &format!("stadium.goals[{index}]"))?);
    }

    let disc_count = reader.read_u8("stadium.discs.length")? as usize;
    stadium.discs.reserve(disc_count);
    for index in 0..disc_count {
        stadium
            .discs
            .push(parse_disc(reader, &format!("stadium.discs[{index}]"))?);
    }

    let joint_count = reader.read_u8("stadium.joints.length")? as usize;
    stadium.joints.reserve(joint_count);
    for index in 0..joint_count {
        stadium
            .joints
            .push(parse_joint(reader, &format!("stadium.joints[{index}]"))?);
    }

    let red_spawn_count = reader.read_u8("stadium.redSpawnPoints.length")? as usize;
    stadium.red_spawn_points.reserve(red_spawn_count);
    for index in 0..red_spawn_count {
        stadium.red_spawn_points.push(parse_point(
            reader,
            &format!("stadium.redSpawnPoints[{index}]"),
        )?);
    }

    let blue_spawn_count = reader.read_u8("stadium.blueSpawnPoints.length")? as usize;
    stadium.blue_spawn_points.reserve(blue_spawn_count);
    for index in 0..blue_spawn_count {
        stadium.blue_spawn_points.push(parse_point(
            reader,
            &format!("stadium.blueSpawnPoints[{index}]"),
        )?);
    }

    Ok(stadium)
}

fn default_stadium_name(default_stadium_id: u8) -> Option<&'static str> {
    let name = match default_stadium_id {
        0 => "Classic",
        1 => "Easy",
        2 => "Small",
        3 => "Big",
        4 => "Rounded",
        5 => "Hockey",
        6 => "Big Hockey",
        7 => "Big Easy",
        8 => "Big Rounded",
        9 => "Huge",
        _ => return None,
    };
    Some(name)
}
