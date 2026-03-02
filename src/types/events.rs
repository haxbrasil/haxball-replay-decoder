use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::{OperationType, Stadium, TeamColors};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendAnnouncementEvent {
    pub msg: String,
    pub color: i32,
    pub style: u8,
    pub sound: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendChatIndicatorEvent {
    pub value: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckConsistencyEvent {
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendInputEvent {
    pub input: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendChatEvent {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinRoomEvent {
    pub player_id: i32,
    pub name: Option<String>,
    pub flag: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KickBanPlayerEvent {
    pub player_id: i32,
    pub reason: Option<String>,
    pub ban: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartGameEvent {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopGameEvent {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PauseResumeGameEvent {
    pub paused: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetGamePlayLimitEvent {
    pub limit_type: i32,
    pub new_value: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetStadiumEvent {
    pub stadium: Stadium,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPlayerTeamEvent {
    pub player_id: i32,
    pub team_id: i8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTeamsLockEvent {
    pub new_value: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPlayerAdminEvent {
    pub player_id: i32,
    pub value: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoTeamsEvent {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPlayerSyncEvent {
    pub value: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PingEvent {
    pub pings: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAvatarEvent {
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTeamColorsEvent {
    pub team_id: i8,
    pub colors: TeamColors,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReorderPlayersEvent {
    pub move_to_top: bool,
    pub player_id_list: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetKickRateLimitEvent {
    pub min: i32,
    pub rate: i32,
    pub burst: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetHeadlessAvatarEvent {
    pub value: Option<String>,
    pub player_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDiscPropertiesEvent {
    pub id: i32,
    pub is_player: bool,
    pub flags: u16,
    pub data1: [Option<f32>; 10],
    pub data2: [Option<i32>; 3],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomEvent {
    pub event_type: u32,
    pub data: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinaryCustomEvent {
    pub event_type: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPlayerIdentityEvent {
    pub id: i32,
    pub data: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind", content = "value")]
pub enum EventPayload {
    SendAnnouncement(SendAnnouncementEvent),
    SendChatIndicator(SendChatIndicatorEvent),
    CheckConsistency(CheckConsistencyEvent),
    SendInput(SendInputEvent),
    SendChat(SendChatEvent),
    JoinRoom(JoinRoomEvent),
    KickBanPlayer(KickBanPlayerEvent),
    StartGame(StartGameEvent),
    StopGame(StopGameEvent),
    PauseResumeGame(PauseResumeGameEvent),
    SetGamePlayLimit(SetGamePlayLimitEvent),
    SetStadium(SetStadiumEvent),
    SetPlayerTeam(SetPlayerTeamEvent),
    SetTeamsLock(SetTeamsLockEvent),
    SetPlayerAdmin(SetPlayerAdminEvent),
    AutoTeams(AutoTeamsEvent),
    SetPlayerSync(SetPlayerSyncEvent),
    Ping(PingEvent),
    SetAvatar(SetAvatarEvent),
    SetTeamColors(SetTeamColorsEvent),
    ReorderPlayers(ReorderPlayersEvent),
    SetKickRateLimit(SetKickRateLimitEvent),
    SetHeadlessAvatar(SetHeadlessAvatarEvent),
    SetDiscProperties(SetDiscPropertiesEvent),
    CustomEvent(CustomEvent),
    BinaryCustomEvent(BinaryCustomEvent),
    SetPlayerIdentity(SetPlayerIdentityEvent),
    Unknown {
        event_type: u8,
        raw_payload: Vec<u8>,
    },
}

impl EventPayload {
    pub fn operation_type(&self) -> Option<OperationType> {
        match self {
            Self::SendAnnouncement(_) => Some(OperationType::SendAnnouncement),
            Self::SendChatIndicator(_) => Some(OperationType::SendChatIndicator),
            Self::CheckConsistency(_) => Some(OperationType::CheckConsistency),
            Self::SendInput(_) => Some(OperationType::SendInput),
            Self::SendChat(_) => Some(OperationType::SendChat),
            Self::JoinRoom(_) => Some(OperationType::JoinRoom),
            Self::KickBanPlayer(_) => Some(OperationType::KickBanPlayer),
            Self::StartGame(_) => Some(OperationType::StartGame),
            Self::StopGame(_) => Some(OperationType::StopGame),
            Self::PauseResumeGame(_) => Some(OperationType::PauseResumeGame),
            Self::SetGamePlayLimit(_) => Some(OperationType::SetGamePlayLimit),
            Self::SetStadium(_) => Some(OperationType::SetStadium),
            Self::SetPlayerTeam(_) => Some(OperationType::SetPlayerTeam),
            Self::SetTeamsLock(_) => Some(OperationType::SetTeamsLock),
            Self::SetPlayerAdmin(_) => Some(OperationType::SetPlayerAdmin),
            Self::AutoTeams(_) => Some(OperationType::AutoTeams),
            Self::SetPlayerSync(_) => Some(OperationType::SetPlayerSync),
            Self::Ping(_) => Some(OperationType::Ping),
            Self::SetAvatar(_) => Some(OperationType::SetAvatar),
            Self::SetTeamColors(_) => Some(OperationType::SetTeamColors),
            Self::ReorderPlayers(_) => Some(OperationType::ReorderPlayers),
            Self::SetKickRateLimit(_) => Some(OperationType::SetKickRateLimit),
            Self::SetHeadlessAvatar(_) => Some(OperationType::SetHeadlessAvatar),
            Self::SetDiscProperties(_) => Some(OperationType::SetDiscProperties),
            Self::CustomEvent(_) => Some(OperationType::CustomEvent),
            Self::BinaryCustomEvent(_) => Some(OperationType::BinaryCustomEvent),
            Self::SetPlayerIdentity(_) => Some(OperationType::SetPlayerIdentity),
            Self::Unknown { .. } => None,
        }
    }

    pub fn event_type_u8(&self) -> u8 {
        match self {
            Self::Unknown { event_type, .. } => *event_type,
            _ => self
                .operation_type()
                .expect("known event payload has operation type")
                .as_u8(),
        }
    }
}
