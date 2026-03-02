use serde::{Deserialize, Serialize};

use crate::types::{EventPayload, OperationType, RoomState};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalMarker {
    pub frame_no: u32,
    pub team_id: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayEvent {
    pub frame_no: u32,
    pub by_id: u16,
    pub payload: EventPayload,
}

impl ReplayEvent {
    pub fn operation_type(&self) -> Option<OperationType> {
        self.payload.operation_type()
    }

    pub fn event_type_u8(&self) -> u8 {
        self.payload.event_type_u8()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayData {
    pub room_data: RoomState,
    pub events: Vec<ReplayEvent>,
    pub goal_markers: Vec<GoalMarker>,
    pub total_frames: u32,
    pub version: u32,
}
