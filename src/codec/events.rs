use crate::codec::inflate_raw;
use crate::codec::primitives::parse_team_colors;
use crate::codec::reader::ReplayReader;
use crate::codec::stadium::parse_stadium;
use crate::error::DecodeError;
use crate::types::{
    AutoTeamsEvent, BinaryCustomEvent, CheckConsistencyEvent, CustomEvent, EventPayload,
    JoinRoomEvent, KickBanPlayerEvent, PauseResumeGameEvent, PingEvent, ReorderPlayersEvent,
    SendAnnouncementEvent, SendChatEvent, SendChatIndicatorEvent, SendInputEvent, SetAvatarEvent,
    SetDiscPropertiesEvent, SetGamePlayLimitEvent, SetHeadlessAvatarEvent, SetKickRateLimitEvent,
    SetPlayerAdminEvent, SetPlayerIdentityEvent, SetPlayerSyncEvent, SetPlayerTeamEvent,
    SetStadiumEvent, SetTeamColorsEvent, SetTeamsLockEvent, StartGameEvent, StopGameEvent,
};

pub(crate) fn parse_event_payload(
    reader: &mut ReplayReader<'_>,
    event_type: u8,
    allow_unknown_event_types: bool,
) -> Result<EventPayload, DecodeError> {
    let payload = match event_type {
        0 => EventPayload::SendAnnouncement(SendAnnouncementEvent {
            msg: reader.read_string("events.sendAnnouncement.msg")?,
            color: reader.read_i32("events.sendAnnouncement.color")?,
            style: reader.read_u8("events.sendAnnouncement.style")?,
            sound: reader.read_u8("events.sendAnnouncement.sound")?,
        }),
        1 => EventPayload::SendChatIndicator(SendChatIndicatorEvent {
            value: reader.read_u8("events.sendChatIndicator.value")?,
        }),
        2 => {
            let data_len = reader.read_var_u32("events.checkConsistency.length")? as usize;
            EventPayload::CheckConsistency(CheckConsistencyEvent {
                data: reader.read_bytes_vec(data_len, "events.checkConsistency.data")?,
            })
        }
        3 => EventPayload::SendInput(SendInputEvent {
            input: reader.read_u32("events.sendInput.input")?,
        }),
        4 => EventPayload::SendChat(SendChatEvent {
            text: reader.read_string("events.sendChat.text")?,
        }),
        5 => EventPayload::JoinRoom(JoinRoomEvent {
            player_id: reader.read_i32("events.joinRoom.playerId")?,
            name: reader.read_nullable_string("events.joinRoom.name")?,
            flag: reader.read_nullable_string("events.joinRoom.flag")?,
            avatar: reader.read_nullable_string("events.joinRoom.avatar")?,
        }),
        6 => EventPayload::KickBanPlayer(KickBanPlayerEvent {
            player_id: reader.read_i32("events.kickBanPlayer.playerId")?,
            reason: reader.read_nullable_string("events.kickBanPlayer.reason")?,
            ban: reader.read_u8("events.kickBanPlayer.ban")? != 0,
        }),
        7 => EventPayload::StartGame(StartGameEvent {}),
        8 => EventPayload::StopGame(StopGameEvent {}),
        9 => EventPayload::PauseResumeGame(PauseResumeGameEvent {
            paused: reader.read_u8("events.pauseResumeGame.paused")? != 0,
        }),
        10 => EventPayload::SetGamePlayLimit(SetGamePlayLimitEvent {
            limit_type: reader.read_i32("events.setGamePlayLimit.type")?,
            new_value: reader.read_i32("events.setGamePlayLimit.newValue")?,
        }),
        11 => {
            let packed_len = reader.read_u16("events.setStadium.packedLength")? as usize;
            let packed = reader.read_bytes(packed_len, "events.setStadium.packed")?;
            let inflated = inflate_raw(packed, "events.setStadium.inflate")?;
            let mut stadium_reader = ReplayReader::new(&inflated);
            let stadium = parse_stadium(&mut stadium_reader)?;
            stadium_reader.ensure_eof("events.setStadium.inner")?;
            EventPayload::SetStadium(SetStadiumEvent { stadium })
        }
        12 => EventPayload::SetPlayerTeam(SetPlayerTeamEvent {
            player_id: reader.read_i32("events.setPlayerTeam.playerId")?,
            team_id: reader.read_i8("events.setPlayerTeam.teamId")?,
        }),
        13 => EventPayload::SetTeamsLock(SetTeamsLockEvent {
            new_value: reader.read_u8("events.setTeamsLock.value")? != 0,
        }),
        14 => EventPayload::SetPlayerAdmin(SetPlayerAdminEvent {
            player_id: reader.read_i32("events.setPlayerAdmin.playerId")?,
            value: reader.read_u8("events.setPlayerAdmin.value")? != 0,
        }),
        15 => EventPayload::AutoTeams(AutoTeamsEvent {}),
        16 => EventPayload::SetPlayerSync(SetPlayerSyncEvent {
            value: reader.read_u8("events.setPlayerSync.value")? != 0,
        }),
        17 => {
            let ping_count = reader.read_var_u32("events.ping.length")? as usize;
            let mut pings = Vec::with_capacity(ping_count);
            for index in 0..ping_count {
                pings.push(reader.read_var_u32(&format!("events.ping.pings[{index}]"))?);
            }
            EventPayload::Ping(PingEvent { pings })
        }
        18 => EventPayload::SetAvatar(SetAvatarEvent {
            value: reader.read_nullable_string("events.setAvatar.value")?,
        }),
        19 => EventPayload::SetTeamColors(SetTeamColorsEvent {
            team_id: reader.read_i8("events.setTeamColors.teamId")?,
            colors: parse_team_colors(reader, "events.setTeamColors.colors")?,
        }),
        20 => {
            let move_to_top = reader.read_u8("events.reorderPlayers.moveToTop")? != 0;
            let count = reader.read_u8("events.reorderPlayers.length")? as usize;
            let mut player_id_list = Vec::with_capacity(count);
            for index in 0..count {
                player_id_list
                    .push(reader.read_i32(&format!("events.reorderPlayers.playerIds[{index}]"))?);
            }
            EventPayload::ReorderPlayers(ReorderPlayersEvent {
                move_to_top,
                player_id_list,
            })
        }
        21 => EventPayload::SetKickRateLimit(SetKickRateLimitEvent {
            min: reader.read_i32("events.setKickRateLimit.min")?,
            rate: reader.read_i32("events.setKickRateLimit.rate")?,
            burst: reader.read_i32("events.setKickRateLimit.burst")?,
        }),
        22 => EventPayload::SetHeadlessAvatar(SetHeadlessAvatarEvent {
            value: reader.read_nullable_string("events.setHeadlessAvatar.value")?,
            player_id: reader.read_i32("events.setHeadlessAvatar.playerId")?,
        }),
        23 => EventPayload::SetDiscProperties(parse_set_disc_properties(reader)?),
        24 => EventPayload::CustomEvent(CustomEvent {
            event_type: reader.read_u32("events.customEvent.type")?,
            data: reader.read_json("events.customEvent.data")?,
        }),
        25 => {
            let packed_len = reader.read_u32("events.binaryCustomEvent.packedLength")? as usize;
            let packed = reader.read_bytes(packed_len, "events.binaryCustomEvent.packed")?;
            let inflated = inflate_raw(packed, "events.binaryCustomEvent.inflate")?;
            let mut inner = ReplayReader::new(&inflated);
            let event_type = inner.read_u32("events.binaryCustomEvent.type")?;
            let data = inner.read_remaining_vec();
            EventPayload::BinaryCustomEvent(BinaryCustomEvent { event_type, data })
        }
        26 => EventPayload::SetPlayerIdentity(SetPlayerIdentityEvent {
            id: reader.read_i32("events.setPlayerIdentity.id")?,
            data: reader.read_json("events.setPlayerIdentity.data")?,
        }),
        _ if allow_unknown_event_types => {
            let raw_payload = reader.read_remaining_vec();
            EventPayload::Unknown {
                event_type,
                raw_payload,
            }
        }
        _ => return Err(DecodeError::UnsupportedEventType(event_type)),
    };

    Ok(payload)
}

fn parse_set_disc_properties(
    reader: &mut ReplayReader<'_>,
) -> Result<SetDiscPropertiesEvent, DecodeError> {
    let id = reader.read_i32("events.setDiscProperties.id")?;
    let is_player = reader.read_u8("events.setDiscProperties.isPlayer")? != 0;
    let flags = reader.read_u16("events.setDiscProperties.flags")?;

    let mut data1 = [None; 10];
    for index in 0..10 {
        if (flags & (1 << index)) != 0 {
            data1[index] =
                Some(reader.read_f32(&format!("events.setDiscProperties.data1[{index}]"))?);
        }
    }

    let mut data2 = [None; 3];
    for index in 0..3 {
        let bit_index = 10 + index;
        if (flags & (1 << bit_index)) != 0 {
            data2[index] =
                Some(reader.read_i32(&format!("events.setDiscProperties.data2[{index}]"))?);
        }
    }

    Ok(SetDiscPropertiesEvent {
        id,
        is_player,
        flags,
        data1,
        data2,
    })
}
