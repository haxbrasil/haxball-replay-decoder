use crate::codec::primitives::{parse_disc, parse_team_colors};
use crate::codec::reader::ReplayReader;
use crate::codec::stadium::parse_stadium;
use crate::error::DecodeError;
use crate::types::{GamePlayState, GameState, Player, RoomState};

pub(crate) fn parse_room_state(reader: &mut ReplayReader<'_>) -> Result<RoomState, DecodeError> {
    let name = reader.read_nullable_string("roomState.name")?;
    let teams_locked = reader.read_u8("roomState.teamsLocked")? != 0;
    let score_limit = reader.read_i32("roomState.scoreLimit")?;
    let time_limit = reader.read_i32("roomState.timeLimit")?;
    let kick_rate_max = reader.read_i16("roomState.kickRateMax")?;
    let kick_rate_rate = reader.read_u8("roomState.kickRateRate")?;
    let kick_rate_min = reader.read_u8("roomState.kickRateMin")?;

    let stadium = parse_stadium(reader)?;

    let game_active = reader.read_u8("roomState.gameActive")? != 0;
    let game_state = if game_active {
        Some(parse_game_state(reader)?)
    } else {
        None
    };

    let player_count = reader.read_u8("roomState.players.length")? as usize;
    let mut players = Vec::with_capacity(player_count);
    for index in 0..player_count {
        players.push(parse_player(reader, index)?);
    }

    let red_team_colors = parse_team_colors(reader, "roomState.redTeamColors")?;
    let blue_team_colors = parse_team_colors(reader, "roomState.blueTeamColors")?;

    Ok(RoomState {
        name,
        teams_locked,
        score_limit,
        time_limit,
        kick_rate_max,
        kick_rate_rate,
        kick_rate_min,
        stadium,
        game_state,
        players,
        red_team_colors,
        blue_team_colors,
    })
}

fn parse_game_state(reader: &mut ReplayReader<'_>) -> Result<GameState, DecodeError> {
    let disc_count = reader.read_u8("gameState.discs.length")? as usize;
    let mut discs = Vec::with_capacity(disc_count);
    for index in 0..disc_count {
        discs.push(parse_disc(reader, &format!("gameState.discs[{index}]"))?);
    }

    let goal_tick_counter = reader.read_i32("gameState.goalTickCounter")?;
    let state = GamePlayState::from_i32(reader.read_i32("gameState.state")?);
    let red_score = reader.read_i32("gameState.redScore")?;
    let blue_score = reader.read_i32("gameState.blueScore")?;
    let time_elapsed = reader.read_f64("gameState.timeElapsed")?;
    let pause_game_tick_counter = reader.read_i32("gameState.pauseGameTickCounter")?;
    let goal_conceding_team = reader.read_u8("gameState.goalConcedingTeam")?;

    Ok(GameState {
        discs,
        goal_tick_counter,
        state,
        red_score,
        blue_score,
        time_elapsed,
        pause_game_tick_counter,
        goal_conceding_team,
    })
}

fn parse_player(reader: &mut ReplayReader<'_>, index: usize) -> Result<Player, DecodeError> {
    let base = format!("roomState.players[{index}]");

    Ok(Player {
        is_admin: reader.read_u8(&format!("{base}.isAdmin"))? != 0,
        avatar_number: reader.read_i32(&format!("{base}.avatarNumber"))?,
        avatar: reader.read_nullable_string(&format!("{base}.avatar"))?,
        headless_avatar: reader.read_nullable_string(&format!("{base}.headlessAvatar"))?,
        sync: reader.read_u8(&format!("{base}.sync"))? != 0,
        flag: reader.read_nullable_string(&format!("{base}.flag"))?,
        metadata: reader.read_i32(&format!("{base}.metadata"))?,
        name: reader.read_nullable_string(&format!("{base}.name"))?,
        input: reader.read_i32(&format!("{base}.input"))?,
        id: reader.read_var_u32(&format!("{base}.id"))?,
        is_kicking: reader.read_u8(&format!("{base}.isKicking"))? != 0,
        kick_rate_max_tick_counter: reader.read_i16(&format!("{base}.kickRateMaxTickCounter"))?,
        kick_rate_min_tick_counter: reader.read_u8(&format!("{base}.kickRateMinTickCounter"))?,
        team_id: reader.read_u8(&format!("{base}.teamId"))?,
        disc_index: reader.read_i16(&format!("{base}.discIndex"))?,
    })
}
