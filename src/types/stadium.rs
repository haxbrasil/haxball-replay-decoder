use serde::{Deserialize, Serialize};

use crate::types::{BackgroundType, CameraFollow};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vertex {
    pub pos: Point,
    pub b_coef: f64,
    pub c_mask: i32,
    pub c_group: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    pub flags: u8,
    pub v0: u8,
    pub v1: u8,
    pub bias: f64,
    pub curve: f64,
    pub color: i32,
    pub vis: bool,
    pub b_coef: f64,
    pub c_mask: i32,
    pub c_group: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plane {
    pub normal: Point,
    pub dist: f64,
    pub b_coef: f64,
    pub c_mask: i32,
    pub c_group: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    pub p0: Point,
    pub p1: Point,
    pub team_id: i8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Disc {
    pub pos: Point,
    pub speed: Point,
    pub gravity: Point,
    pub radius: f64,
    pub b_coef: f64,
    pub inv_mass: f64,
    pub damping: f64,
    pub color: u32,
    pub c_mask: i32,
    pub c_group: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Joint {
    pub d0: u8,
    pub d1: u8,
    pub min_length: f64,
    pub max_length: f64,
    pub strength: f64,
    pub color: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPhysics {
    pub b_coef: f64,
    pub inv_mass: f64,
    pub damping: f64,
    pub acceleration: f64,
    pub kicking_acceleration: f64,
    pub kicking_damping: f64,
    pub kick_strength: f64,
    pub gravity: Point,
    pub c_group: i32,
    pub radius: f64,
    pub kickback: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamColors {
    pub angle: u8,
    pub text: i32,
    pub inner: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stadium {
    pub default_stadium_id: u8,
    pub name: Option<String>,
    pub background_type: Option<BackgroundType>,
    pub background_width: Option<f64>,
    pub background_height: Option<f64>,
    pub background_kickoff_radius: Option<f64>,
    pub background_corner_radius: Option<f64>,
    pub background_goal_line: Option<f64>,
    pub background_color: Option<i32>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub spawn_distance: Option<f64>,
    pub player_physics: Option<PlayerPhysics>,
    pub max_view_width: Option<u16>,
    pub camera_follow: Option<CameraFollow>,
    pub can_be_stored: Option<bool>,
    pub full_kickoff_reset: Option<bool>,
    pub vertices: Vec<Vertex>,
    pub segments: Vec<Segment>,
    pub planes: Vec<Plane>,
    pub goals: Vec<Goal>,
    pub discs: Vec<Disc>,
    pub joints: Vec<Joint>,
    pub red_spawn_points: Vec<Point>,
    pub blue_spawn_points: Vec<Point>,
}

impl Stadium {
    pub fn is_custom(&self) -> bool {
        self.default_stadium_id == 255
    }
}
