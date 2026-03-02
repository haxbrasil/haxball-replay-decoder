use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OperationType {
    SendAnnouncement,
    SendChatIndicator,
    CheckConsistency,
    SendInput,
    SendChat,
    JoinRoom,
    KickBanPlayer,
    StartGame,
    StopGame,
    PauseResumeGame,
    SetGamePlayLimit,
    SetStadium,
    SetPlayerTeam,
    SetTeamsLock,
    SetPlayerAdmin,
    AutoTeams,
    SetPlayerSync,
    Ping,
    SetAvatar,
    SetTeamColors,
    ReorderPlayers,
    SetKickRateLimit,
    SetHeadlessAvatar,
    SetDiscProperties,
    CustomEvent,
    BinaryCustomEvent,
    SetPlayerIdentity,
}

impl OperationType {
    pub fn from_u8(value: u8) -> Option<Self> {
        let operation = match value {
            0 => Self::SendAnnouncement,
            1 => Self::SendChatIndicator,
            2 => Self::CheckConsistency,
            3 => Self::SendInput,
            4 => Self::SendChat,
            5 => Self::JoinRoom,
            6 => Self::KickBanPlayer,
            7 => Self::StartGame,
            8 => Self::StopGame,
            9 => Self::PauseResumeGame,
            10 => Self::SetGamePlayLimit,
            11 => Self::SetStadium,
            12 => Self::SetPlayerTeam,
            13 => Self::SetTeamsLock,
            14 => Self::SetPlayerAdmin,
            15 => Self::AutoTeams,
            16 => Self::SetPlayerSync,
            17 => Self::Ping,
            18 => Self::SetAvatar,
            19 => Self::SetTeamColors,
            20 => Self::ReorderPlayers,
            21 => Self::SetKickRateLimit,
            22 => Self::SetHeadlessAvatar,
            23 => Self::SetDiscProperties,
            24 => Self::CustomEvent,
            25 => Self::BinaryCustomEvent,
            26 => Self::SetPlayerIdentity,
            _ => return None,
        };
        Some(operation)
    }

    pub fn as_u8(self) -> u8 {
        match self {
            Self::SendAnnouncement => 0,
            Self::SendChatIndicator => 1,
            Self::CheckConsistency => 2,
            Self::SendInput => 3,
            Self::SendChat => 4,
            Self::JoinRoom => 5,
            Self::KickBanPlayer => 6,
            Self::StartGame => 7,
            Self::StopGame => 8,
            Self::PauseResumeGame => 9,
            Self::SetGamePlayLimit => 10,
            Self::SetStadium => 11,
            Self::SetPlayerTeam => 12,
            Self::SetTeamsLock => 13,
            Self::SetPlayerAdmin => 14,
            Self::AutoTeams => 15,
            Self::SetPlayerSync => 16,
            Self::Ping => 17,
            Self::SetAvatar => 18,
            Self::SetTeamColors => 19,
            Self::ReorderPlayers => 20,
            Self::SetKickRateLimit => 21,
            Self::SetHeadlessAvatar => 22,
            Self::SetDiscProperties => 23,
            Self::CustomEvent => 24,
            Self::BinaryCustomEvent => 25,
            Self::SetPlayerIdentity => 26,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BackgroundType {
    None,
    Grass,
    Hockey,
    Unknown(i32),
}

impl BackgroundType {
    pub fn from_i32(value: i32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Grass,
            2 => Self::Hockey,
            other => Self::Unknown(other),
        }
    }

    pub fn as_i32(self) -> i32 {
        match self {
            Self::None => 0,
            Self::Grass => 1,
            Self::Hockey => 2,
            Self::Unknown(value) => value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CameraFollow {
    None,
    Player,
    Unknown(u8),
}

impl CameraFollow {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Player,
            other => Self::Unknown(other),
        }
    }

    pub fn as_u8(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Player => 1,
            Self::Unknown(value) => value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GamePlayState {
    BeforeKickOff,
    Playing,
    AfterGoal,
    Ending,
    Unknown(i32),
}

impl GamePlayState {
    pub fn from_i32(value: i32) -> Self {
        match value {
            0 => Self::BeforeKickOff,
            1 => Self::Playing,
            2 => Self::AfterGoal,
            3 => Self::Ending,
            other => Self::Unknown(other),
        }
    }

    pub fn as_i32(self) -> i32 {
        match self {
            Self::BeforeKickOff => 0,
            Self::Playing => 1,
            Self::AfterGoal => 2,
            Self::Ending => 3,
            Self::Unknown(value) => value,
        }
    }
}
