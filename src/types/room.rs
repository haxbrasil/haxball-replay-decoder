use serde::{Deserialize, Serialize};

use crate::types::{Disc, GamePlayState, Stadium, TeamColors};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub is_admin: bool,
    pub avatar_number: i32,
    pub avatar: Option<String>,
    pub headless_avatar: Option<String>,
    pub sync: bool,
    pub flag: Option<String>,
    pub metadata: i32,
    pub name: Option<String>,
    pub input: i32,
    pub id: u32,
    pub is_kicking: bool,
    pub kick_rate_max_tick_counter: i16,
    pub kick_rate_min_tick_counter: u8,
    pub team_id: u8,
    pub disc_index: i16,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    pub discs: Vec<Disc>,
    pub goal_tick_counter: i32,
    pub state: GamePlayState,
    pub red_score: i32,
    pub blue_score: i32,
    pub time_elapsed: f64,
    pub pause_game_tick_counter: i32,
    pub goal_conceding_team: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomState {
    pub name: Option<String>,
    pub teams_locked: bool,
    pub score_limit: i32,
    pub time_limit: i32,
    pub kick_rate_max: i16,
    pub kick_rate_rate: u8,
    pub kick_rate_min: u8,
    pub stadium: Stadium,
    pub game_state: Option<GameState>,
    pub players: Vec<Player>,
    pub red_team_colors: TeamColors,
    pub blue_team_colors: TeamColors,
}
