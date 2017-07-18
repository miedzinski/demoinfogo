mod kind;

use csgoproto::cstrike15_usermessages::{self, ECstrike15UserMessages};
use csgoproto::netmessages::{self, NET_Messages, SVC_Messages, CSVCMsg_UserMessage};
use protobuf::{CodedInputStream, parse_from_bytes, ProtobufEnum};

use error::{DemoError, DemoResult};
use read::FromStream;
pub use self::kind::MessageKind;

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Svc(SvcMessage),
    Net(NetMessage),
    User(UserMessage),
}

impl Message {
    pub fn is_svc(&self) -> bool {
        matches!(*self, Message::Svc(_))
    }

    pub fn is_net(&self) -> bool {
        matches!(*self, Message::Net(_))
    }

    pub fn is_user(&self) -> bool {
        matches!(*self, Message::User(_))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SvcMessage {
    ServerInfo(netmessages::CSVCMsg_ServerInfo),
    SendTable(netmessages::CSVCMsg_SendTable),
    ClassInfo(netmessages::CSVCMsg_ClassInfo),
    SetPause(netmessages::CSVCMsg_SetPause),
    CreateStringTable(netmessages::CSVCMsg_CreateStringTable),
    UpdateStringTable(netmessages::CSVCMsg_UpdateStringTable),
    VoiceInit(netmessages::CSVCMsg_VoiceInit),
    VoiceData(netmessages::CSVCMsg_VoiceData),
    Print(netmessages::CSVCMsg_Print),
    Sounds(netmessages::CSVCMsg_Sounds),
    SetView(netmessages::CSVCMsg_SetView),
    FixAngle(netmessages::CSVCMsg_FixAngle),
    CrosshairAngle(netmessages::CSVCMsg_CrosshairAngle),
    BspDecal(netmessages::CSVCMsg_BSPDecal),
    SplitScreen(netmessages::CSVCMsg_SplitScreen),
    EntityMessage(netmessages::CSVCMsg_EntityMsg),
    GameEvent(netmessages::CSVCMsg_GameEvent),
    PacketEntities(netmessages::CSVCMsg_PacketEntities),
    TempEntities(netmessages::CSVCMsg_TempEntities),
    Prefetch(netmessages::CSVCMsg_Prefetch),
    Menu(netmessages::CSVCMsg_Menu),
    GameEventList(netmessages::CSVCMsg_GameEventList),
    GetCvarValue(netmessages::CSVCMsg_GetCvarValue),
    PaintmapData(netmessages::CSVCMsg_PaintmapData),
    CmdKeyValues(netmessages::CSVCMsg_CmdKeyValues),
    EncryptedData(netmessages::CSVCMsg_EncryptedData),
    HltvReplay(netmessages::CSVCMsg_HltvReplay),
    BroadcastCommand(netmessages::CSVCMsg_Broadcast_Command),
}

#[derive(Debug, Clone, PartialEq)]
pub enum NetMessage {
    Nop(netmessages::CNETMsg_NOP),
    Disconnect(netmessages::CNETMsg_Disconnect),
    File(netmessages::CNETMsg_File),
    SplitScreenUser(netmessages::CNETMsg_SplitScreenUser),
    Tick(netmessages::CNETMsg_Tick),
    StringCmd(netmessages::CNETMsg_StringCmd),
    SetConVar(netmessages::CNETMsg_SetConVar),
    SignonState(netmessages::CNETMsg_SignonState),
    PlayerAvatarData(netmessages::CNETMsg_PlayerAvatarData),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserMessage {
    VGuiMenu(cstrike15_usermessages::CCSUsrMsg_VGUIMenu),
    Geiger(cstrike15_usermessages::CCSUsrMsg_Geiger),
    Train(cstrike15_usermessages::CCSUsrMsg_Train),
    HudText(cstrike15_usermessages::CCSUsrMsg_HudText),
    SayText(cstrike15_usermessages::CCSUsrMsg_SayText),
    SayText2(cstrike15_usermessages::CCSUsrMsg_SayText2),
    TextMsg(cstrike15_usermessages::CCSUsrMsg_TextMsg),
    HudMsg(cstrike15_usermessages::CCSUsrMsg_HudMsg),
    ResetHud(cstrike15_usermessages::CCSUsrMsg_ResetHud),
    GameTitle(cstrike15_usermessages::CCSUsrMsg_GameTitle),
    Shake(cstrike15_usermessages::CCSUsrMsg_Shake),
    Fade(cstrike15_usermessages::CCSUsrMsg_Fade),
    Rumble(cstrike15_usermessages::CCSUsrMsg_Rumble),
    CloseCaption(cstrike15_usermessages::CCSUsrMsg_CloseCaption),
    CloseCaptionDirect(cstrike15_usermessages::CCSUsrMsg_CloseCaptionDirect),
    SendAudio(cstrike15_usermessages::CCSUsrMsg_SendAudio),
    RawAudio(cstrike15_usermessages::CCSUsrMsg_RawAudio),
    VoiceMask(cstrike15_usermessages::CCSUsrMsg_VoiceMask),
    RequestState(cstrike15_usermessages::CCSUsrMsg_RequestState),
    Damage(cstrike15_usermessages::CCSUsrMsg_Damage),
    RadioText(cstrike15_usermessages::CCSUsrMsg_RadioText),
    HintText(cstrike15_usermessages::CCSUsrMsg_HintText),
    KeyHintText(cstrike15_usermessages::CCSUsrMsg_KeyHintText),
    ProcessSpottedEntityUpdate(cstrike15_usermessages::CCSUsrMsg_ProcessSpottedEntityUpdate),
    ReloadEffect(cstrike15_usermessages::CCSUsrMsg_ReloadEffect),
    AdjustMoney(cstrike15_usermessages::CCSUsrMsg_AdjustMoney),
    StopSpectatorMode(cstrike15_usermessages::CCSUsrMsg_StopSpectatorMode),
    KillCam(cstrike15_usermessages::CCSUsrMsg_KillCam),
    DesiredTimescale(cstrike15_usermessages::CCSUsrMsg_DesiredTimescale),
    CurrentTimescale(cstrike15_usermessages::CCSUsrMsg_CurrentTimescale),
    AchievementEvent(cstrike15_usermessages::CCSUsrMsg_AchievementEvent),
    MatchEndConditions(cstrike15_usermessages::CCSUsrMsg_MatchEndConditions),
    DisconnectToLobby(cstrike15_usermessages::CCSUsrMsg_DisconnectToLobby),
    PlayerStatsUpdate(cstrike15_usermessages::CCSUsrMsg_PlayerStatsUpdate),
    DisplayInventory(cstrike15_usermessages::CCSUsrMsg_DisplayInventory),
    WarmupHasEnded(cstrike15_usermessages::CCSUsrMsg_WarmupHasEnded),
    ClientInfo(cstrike15_usermessages::CCSUsrMsg_ClientInfo),
    XRankGet(cstrike15_usermessages::CCSUsrMsg_XRankGet),
    XRankUpd(cstrike15_usermessages::CCSUsrMsg_XRankUpd),
    CallVoteFailed(cstrike15_usermessages::CCSUsrMsg_CallVoteFailed),
    VoteStart(cstrike15_usermessages::CCSUsrMsg_VoteStart),
    VotePass(cstrike15_usermessages::CCSUsrMsg_VotePass),
    VoteFailed(cstrike15_usermessages::CCSUsrMsg_VoteFailed),
    VoteSetup(cstrike15_usermessages::CCSUsrMsg_VoteSetup),
    ServerRankRevealAll(cstrike15_usermessages::CCSUsrMsg_ServerRankRevealAll),
    SendLastKillerDamageToClient(cstrike15_usermessages::CCSUsrMsg_SendLastKillerDamageToClient),
    ServerRankUpdate(cstrike15_usermessages::CCSUsrMsg_ServerRankUpdate),
    ItemPickup(cstrike15_usermessages::CCSUsrMsg_ItemPickup),
    ShowMenu(cstrike15_usermessages::CCSUsrMsg_ShowMenu),
    BarTime(cstrike15_usermessages::CCSUsrMsg_BarTime),
    AmmoDenied(cstrike15_usermessages::CCSUsrMsg_AmmoDenied),
    MarkAchievement(cstrike15_usermessages::CCSUsrMsg_MarkAchievement),
    MatchStatsUpdate(cstrike15_usermessages::CCSUsrMsg_MatchStatsUpdate),
    ItemDrop(cstrike15_usermessages::CCSUsrMsg_ItemDrop),
    GlowPropTurnOff(cstrike15_usermessages::CCSUsrMsg_GlowPropTurnOff),
    SendPlayerItemDrops(cstrike15_usermessages::CCSUsrMsg_SendPlayerItemDrops),
    RoundBackupFilenames(cstrike15_usermessages::CCSUsrMsg_RoundBackupFilenames),
    SendPlayerItemFound(cstrike15_usermessages::CCSUsrMsg_SendPlayerItemFound),
    ReportHit(cstrike15_usermessages::CCSUsrMsg_ReportHit),
    XpUpdate(cstrike15_usermessages::CCSUsrMsg_XpUpdate),
    QuestProgress(cstrike15_usermessages::CCSUsrMsg_QuestProgress),
    ScoreLeaderboardData(cstrike15_usermessages::CCSUsrMsg_ScoreLeaderboardData),
    UpdateTeamMoney,
    Unknown(UnknownMessage),
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnknownMessage {
    msg_type: i32,
    msg_data: Vec<u8>,
}

impl UnknownMessage {
    pub fn msg_type(&self) -> i32 {
        self.msg_type
    }

    pub fn msg_data(&self) -> &[u8] {
        &self.msg_data
    }
}

impl FromStream for Message {
    fn from_stream(stream: &mut CodedInputStream) -> DemoResult<Message> {
        let msg_type = stream.read_raw_varint32()? as i32;

        if msg_type == SVC_Messages::svc_UserMessage.value() {
            Ok(Message::User(unpack_user_msg(stream.read_message()?)?))
        } else if let Some(msg_type) = SVC_Messages::from_i32(msg_type) {
            use self::SVC_Messages::*;
            use self::SvcMessage::*;
            Ok(Message::Svc(match msg_type {
                svc_ServerInfo => ServerInfo(stream.read_message()?),
                svc_SendTable => SendTable(stream.read_message()?),
                svc_ClassInfo => ClassInfo(stream.read_message()?),
                svc_SetPause => SetPause(stream.read_message()?),
                svc_CreateStringTable => CreateStringTable(stream.read_message()?),
                svc_UpdateStringTable => UpdateStringTable(stream.read_message()?),
                svc_VoiceInit => VoiceInit(stream.read_message()?),
                svc_VoiceData => VoiceData(stream.read_message()?),
                svc_Print => Print(stream.read_message()?),
                svc_Sounds => Sounds(stream.read_message()?),
                svc_SetView => SetView(stream.read_message()?),
                svc_FixAngle => FixAngle(stream.read_message()?),
                svc_CrosshairAngle => CrosshairAngle(stream.read_message()?),
                svc_BSPDecal => BspDecal(stream.read_message()?),
                svc_SplitScreen => SplitScreen(stream.read_message()?),
                svc_EntityMessage => EntityMessage(stream.read_message()?),
                svc_GameEvent => GameEvent(stream.read_message()?),
                svc_PacketEntities => PacketEntities(stream.read_message()?),
                svc_TempEntities => TempEntities(stream.read_message()?),
                svc_Prefetch => Prefetch(stream.read_message()?),
                svc_Menu => Menu(stream.read_message()?),
                svc_GameEventList => GameEventList(stream.read_message()?),
                svc_GetCvarValue => GetCvarValue(stream.read_message()?),
                svc_PaintmapData => PaintmapData(stream.read_message()?),
                svc_CmdKeyValues => CmdKeyValues(stream.read_message()?),
                svc_EncryptedData => EncryptedData(stream.read_message()?),
                svc_HltvReplay => HltvReplay(stream.read_message()?),
                svc_Broadcast_Command => BroadcastCommand(stream.read_message()?),
                svc_UserMessage => unreachable!(),
            }))
        } else if let Some(msg_type) = NET_Messages::from_i32(msg_type) {
            use self::NET_Messages::*;
            use self::NetMessage::*;
            Ok(Message::Net(match msg_type {
                net_NOP => Nop(stream.read_message()?),
                net_Disconnect => Disconnect(stream.read_message()?),
                net_File => File(stream.read_message()?),
                net_SplitScreenUser => SplitScreenUser(stream.read_message()?),
                net_Tick => Tick(stream.read_message()?),
                net_StringCmd => StringCmd(stream.read_message()?),
                net_SetConVar => SetConVar(stream.read_message()?),
                net_SignonState => SignonState(stream.read_message()?),
                net_PlayerAvatarData => PlayerAvatarData(stream.read_message()?),
            }))
        } else {
            Err(DemoError::UnknownMessage(msg_type))
        }
    }
}

fn unpack_user_msg(mut msg: CSVCMsg_UserMessage) -> DemoResult<UserMessage> {
    let msg_type = ECstrike15UserMessages::from_i32(msg.get_msg_type());

    if msg_type.is_none() {
        return Ok(UserMessage::Unknown(UnknownMessage {
            msg_type: msg.get_msg_type(),
            msg_data: msg.take_msg_data(),
        }));
    }

    let bytes = msg.get_msg_data();

    use self::ECstrike15UserMessages::*;
    use self::UserMessage::*;

    Ok(match msg_type.unwrap() {
        CS_UM_VGUIMenu => VGuiMenu(parse_from_bytes(bytes)?),
        CS_UM_Geiger => Geiger(parse_from_bytes(bytes)?),
        CS_UM_Train => Train(parse_from_bytes(bytes)?),
        CS_UM_HudText => HudText(parse_from_bytes(bytes)?),
        CS_UM_SayText => SayText(parse_from_bytes(bytes)?),
        CS_UM_SayText2 => SayText2(parse_from_bytes(bytes)?),
        CS_UM_TextMsg => TextMsg(parse_from_bytes(bytes)?),
        CS_UM_HudMsg => HudMsg(parse_from_bytes(bytes)?),
        CS_UM_ResetHud => ResetHud(parse_from_bytes(bytes)?),
        CS_UM_GameTitle => GameTitle(parse_from_bytes(bytes)?),
        CS_UM_Shake => Shake(parse_from_bytes(bytes)?),
        CS_UM_Fade => Fade(parse_from_bytes(bytes)?),
        CS_UM_Rumble => Rumble(parse_from_bytes(bytes)?),
        CS_UM_CloseCaption => CloseCaption(parse_from_bytes(bytes)?),
        CS_UM_CloseCaptionDirect => CloseCaptionDirect(parse_from_bytes(bytes)?),
        CS_UM_SendAudio => SendAudio(parse_from_bytes(bytes)?),
        CS_UM_RawAudio => RawAudio(parse_from_bytes(bytes)?),
        CS_UM_VoiceMask => VoiceMask(parse_from_bytes(bytes)?),
        CS_UM_RequestState => RequestState(parse_from_bytes(bytes)?),
        CS_UM_Damage => Damage(parse_from_bytes(bytes)?),
        CS_UM_RadioText => RadioText(parse_from_bytes(bytes)?),
        CS_UM_HintText => HintText(parse_from_bytes(bytes)?),
        CS_UM_KeyHintText => KeyHintText(parse_from_bytes(bytes)?),
        CS_UM_ProcessSpottedEntityUpdate => ProcessSpottedEntityUpdate(parse_from_bytes(bytes)?),
        CS_UM_ReloadEffect => ReloadEffect(parse_from_bytes(bytes)?),
        CS_UM_AdjustMoney => AdjustMoney(parse_from_bytes(bytes)?),
        CS_UM_StopSpectatorMode => StopSpectatorMode(parse_from_bytes(bytes)?),
        CS_UM_KillCam => KillCam(parse_from_bytes(bytes)?),
        CS_UM_DesiredTimescale => DesiredTimescale(parse_from_bytes(bytes)?),
        CS_UM_CurrentTimescale => CurrentTimescale(parse_from_bytes(bytes)?),
        CS_UM_AchievementEvent => AchievementEvent(parse_from_bytes(bytes)?),
        CS_UM_MatchEndConditions => MatchEndConditions(parse_from_bytes(bytes)?),
        CS_UM_DisconnectToLobby => DisconnectToLobby(parse_from_bytes(bytes)?),
        CS_UM_PlayerStatsUpdate => PlayerStatsUpdate(parse_from_bytes(bytes)?),
        CS_UM_DisplayInventory => DisplayInventory(parse_from_bytes(bytes)?),
        CS_UM_WarmupHasEnded => WarmupHasEnded(parse_from_bytes(bytes)?),
        CS_UM_ClientInfo => ClientInfo(parse_from_bytes(bytes)?),
        CS_UM_XRankGet => XRankGet(parse_from_bytes(bytes)?),
        CS_UM_XRankUpd => XRankUpd(parse_from_bytes(bytes)?),
        CS_UM_CallVoteFailed => CallVoteFailed(parse_from_bytes(bytes)?),
        CS_UM_VoteStart => VoteStart(parse_from_bytes(bytes)?),
        CS_UM_VotePass => VotePass(parse_from_bytes(bytes)?),
        CS_UM_VoteFailed => VoteFailed(parse_from_bytes(bytes)?),
        CS_UM_VoteSetup => VoteSetup(parse_from_bytes(bytes)?),
        CS_UM_ServerRankRevealAll => ServerRankRevealAll(parse_from_bytes(bytes)?),
        CS_UM_SendLastKillerDamageToClient => SendLastKillerDamageToClient(parse_from_bytes(bytes)?),
        CS_UM_ServerRankUpdate => ServerRankUpdate(parse_from_bytes(bytes)?),
        CS_UM_ItemPickup => ItemPickup(parse_from_bytes(bytes)?),
        CS_UM_ShowMenu => ShowMenu(parse_from_bytes(bytes)?),
        CS_UM_BarTime => BarTime(parse_from_bytes(bytes)?),
        CS_UM_AmmoDenied => AmmoDenied(parse_from_bytes(bytes)?),
        CS_UM_MarkAchievement => MarkAchievement(parse_from_bytes(bytes)?),
        CS_UM_MatchStatsUpdate => MatchStatsUpdate(parse_from_bytes(bytes)?),
        CS_UM_ItemDrop => ItemDrop(parse_from_bytes(bytes)?),
        CS_UM_GlowPropTurnOff => GlowPropTurnOff(parse_from_bytes(bytes)?),
        CS_UM_SendPlayerItemDrops => SendPlayerItemDrops(parse_from_bytes(bytes)?),
        CS_UM_RoundBackupFilenames => RoundBackupFilenames(parse_from_bytes(bytes)?),
        CS_UM_SendPlayerItemFound => SendPlayerItemFound(parse_from_bytes(bytes)?),
        CS_UM_ReportHit => ReportHit(parse_from_bytes(bytes)?),
        CS_UM_XpUpdate => XpUpdate(parse_from_bytes(bytes)?),
        CS_UM_QuestProgress => QuestProgress(parse_from_bytes(bytes)?),
        CS_UM_ScoreLeaderboardData => ScoreLeaderboardData(parse_from_bytes(bytes)?),
        CS_UM_UpdateTeamMoney => UpdateTeamMoney,
    })
}

#[cfg(test)]
mod tests {
    use csgoproto::netmessages::{NET_Messages, SVC_Messages, CSVCMsg_UserMessage};
    use csgoproto::cstrike15_usermessages::ECstrike15UserMessages;
    use protobuf::CodedInputStream;

    use error::DemoError;
    use read::FromStream;
    use tests::read;
    use super::{MessageKind, Message, NetMessage, SvcMessage, UserMessage};
    use super::unpack_user_msg;

    #[test]
    fn parses_svc() {
        let msg: Message = read("svc-print.bin").unwrap();
        let expected = "\n\
            Counter-Strike: Global Offensive\n\
            Map: de_dust2\n\
            Players: 1 (0 bots) / 20 humans\n\
            Build: 6771\n\
            Server Number: 1\n\n";
        assert_eq!(msg.kind(), MessageKind::Svc(SVC_Messages::svc_Print));
        match msg {
            Message::Svc(SvcMessage::Print(msg)) => {
                assert_eq!(msg.get_text(), expected);
            }
            x => panic!("{:?}", x),
        }
    }

    #[test]
    fn parses_net() {
        let msg: Message = read("net-stringcmd.bin").unwrap();
        assert_eq!(msg.kind(), MessageKind::Net(NET_Messages::net_StringCmd));
        match msg {
            Message::Net(NetMessage::StringCmd(msg)) => {
                assert_eq!(msg.get_command(), "dsp_player 0\n");
            }
            x => panic!("{:?}", x),
        }
    }

    #[test]
    fn parses_user() {
        let mut msg = CSVCMsg_UserMessage::new();
        msg.set_msg_type(1);
        msg.set_msg_data(vec![10, 8, 111, 118, 101, 114, 118, 105, 101, 119, 16, 0]);
        let unpacked = unpack_user_msg(msg).unwrap();
        assert_eq!(unpacked.kind(), MessageKind::User(ECstrike15UserMessages::CS_UM_VGUIMenu));
        match unpacked {
            UserMessage::VGuiMenu(msg) => {
                assert_eq!(msg.get_name(), "overview");
                assert!(!msg.get_show());
                assert!(msg.get_subkeys().is_empty());
            }
            x => panic!("{:?}", x),
        }
    }

    #[test]
    fn parses_svc_usermsg() {
        let msg: Message = read("svc-usermsg.bin").unwrap();
        assert_eq!(msg.kind(), MessageKind::User(ECstrike15UserMessages::CS_UM_TextMsg));
        match msg {
            Message::User(UserMessage::TextMsg(msg)) => {
                assert_eq!(msg.get_msg_dst(), 3);
                assert_eq!(
                    msg.get_params(),
                    ["#SFUI_Notice_Match_Will_Start_Chat", "", "", "", ""]
                );
            }
            x => panic!("{:?}", x),
        }
    }

    #[test]
    fn handles_unknown() {
        let bytes = [0xac, 0x02];
        let mut stream = CodedInputStream::from_bytes(&bytes);
        let msg = Message::from_stream(&mut stream);
        assert_matches!(msg, Err(DemoError::UnknownMessage(300)));
    }

    #[test]
    fn handles_unknown_user() {
        let mut msg = CSVCMsg_UserMessage::new();
        msg.set_msg_type(300);
        msg.set_msg_data(vec![1, 2, 3, 4]);
        let unpacked = unpack_user_msg(msg).unwrap();
        assert_eq!(unpacked.kind(), MessageKind::UnknownUser);
        match unpacked {
            UserMessage::Unknown(msg) => {
                assert_eq!(msg.msg_type(), 300);
                assert_eq!(msg.msg_data(), &[1, 2, 3, 4]);
            }
            x => panic!("{:?}", x),
        }
    }
}
