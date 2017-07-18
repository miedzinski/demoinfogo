use csgoproto::netmessages::{NET_Messages, SVC_Messages};
use csgoproto::cstrike15_usermessages::ECstrike15UserMessages;

use message::{Message, NetMessage, SvcMessage, UserMessage};

#[derive(Debug, Clone, PartialEq)]
pub enum MessageKind {
    Svc(SVC_Messages),
    Net(NET_Messages),
    User(ECstrike15UserMessages),
    UnknownUser,
}

impl Message {
    pub fn kind(&self) -> MessageKind {
        match *self {
            Message::Svc(ref msg) => {
                let kind = msg.kind();
                debug_assert_matches!(kind, MessageKind::Svc(_));
                kind
            }
            Message::Net(ref msg) => {
                let kind = msg.kind();
                debug_assert_matches!(kind, MessageKind::Net(_));
                kind
            }
            Message::User(ref msg) => {
                let kind = msg.kind();
                debug_assert_matches!(kind, MessageKind::User(_) | MessageKind::UnknownUser);
                kind
            }
        }
    }
}

impl SvcMessage {
    pub fn kind(&self) -> MessageKind {
        use self::SvcMessage::*;
        use self::SVC_Messages::*;
        use self::MessageKind::Svc;
        match *self {
            ServerInfo(_) => Svc(svc_ServerInfo),
            SendTable(_) => Svc(svc_SendTable),
            ClassInfo(_) => Svc(svc_ClassInfo),
            SetPause(_) => Svc(svc_SetPause),
            CreateStringTable(_) => Svc(svc_CreateStringTable),
            UpdateStringTable(_) => Svc(svc_UpdateStringTable),
            VoiceInit(_) => Svc(svc_VoiceInit),
            VoiceData(_) => Svc(svc_VoiceData),
            Print(_) => Svc(svc_Print),
            Sounds(_) => Svc(svc_Sounds),
            SetView(_) => Svc(svc_SetView),
            FixAngle(_) => Svc(svc_FixAngle),
            CrosshairAngle(_) => Svc(svc_CrosshairAngle),
            BspDecal(_) => Svc(svc_BSPDecal),
            SplitScreen(_) => Svc(svc_SplitScreen),
            EntityMessage(_) => Svc(svc_EntityMessage),
            GameEvent(_) => Svc(svc_GameEvent),
            PacketEntities(_) => Svc(svc_PacketEntities),
            TempEntities(_) => Svc(svc_TempEntities),
            Prefetch(_) => Svc(svc_Prefetch),
            Menu(_) => Svc(svc_Menu),
            GameEventList(_) => Svc(svc_GameEventList),
            GetCvarValue(_) => Svc(svc_GetCvarValue),
            PaintmapData(_) => Svc(svc_PaintmapData),
            CmdKeyValues(_) => Svc(svc_CmdKeyValues),
            EncryptedData(_) => Svc(svc_EncryptedData),
            HltvReplay(_) => Svc(svc_HltvReplay),
            BroadcastCommand(_) => Svc(svc_Broadcast_Command),
        }
    }
}

impl NetMessage {
    pub fn kind(&self) -> MessageKind {
        use self::NetMessage::*;
        use self::NET_Messages::*;
        use self::MessageKind::Net;
        match *self {
            Nop(_) => Net(net_NOP),
            Disconnect(_) => Net(net_Disconnect),
            File(_) => Net(net_File),
            SplitScreenUser(_) => Net(net_SplitScreenUser),
            Tick(_) => Net(net_Tick),
            StringCmd(_) => Net(net_StringCmd),
            SetConVar(_) => Net(net_SetConVar),
            SignonState(_) => Net(net_SignonState),
            PlayerAvatarData(_) => Net(net_PlayerAvatarData),
        }
    }
}

impl UserMessage {
    pub fn kind(&self) -> MessageKind {
        use self::UserMessage::*;
        use self::ECstrike15UserMessages::*;
        use self::MessageKind::User;
        match *self {
            VGuiMenu(_) => User(CS_UM_VGUIMenu),
            Geiger(_) => User(CS_UM_Geiger),
            Train(_) => User(CS_UM_Train),
            HudText(_) => User(CS_UM_HudText),
            SayText(_) => User(CS_UM_SayText),
            SayText2(_) => User(CS_UM_SayText2),
            TextMsg(_) => User(CS_UM_TextMsg),
            HudMsg(_) => User(CS_UM_HudMsg),
            ResetHud(_) => User(CS_UM_ResetHud),
            GameTitle(_) => User(CS_UM_GameTitle),
            Shake(_) => User(CS_UM_Shake),
            Fade(_) => User(CS_UM_Fade),
            Rumble(_) => User(CS_UM_Rumble),
            CloseCaption(_) => User(CS_UM_CloseCaption),
            CloseCaptionDirect(_) => User(CS_UM_CloseCaptionDirect),
            SendAudio(_) => User(CS_UM_SendAudio),
            RawAudio(_) => User(CS_UM_RawAudio),
            VoiceMask(_) => User(CS_UM_VoiceMask),
            RequestState(_) => User(CS_UM_RequestState),
            Damage(_) => User(CS_UM_Damage),
            RadioText(_) => User(CS_UM_RadioText),
            HintText(_) => User(CS_UM_HintText),
            KeyHintText(_) => User(CS_UM_KeyHintText),
            ProcessSpottedEntityUpdate(_) => User(CS_UM_ProcessSpottedEntityUpdate),
            ReloadEffect(_) => User(CS_UM_ReloadEffect),
            AdjustMoney(_) => User(CS_UM_AdjustMoney),
            StopSpectatorMode(_) => User(CS_UM_StopSpectatorMode),
            KillCam(_) => User(CS_UM_KillCam),
            DesiredTimescale(_) => User(CS_UM_DesiredTimescale),
            CurrentTimescale(_) => User(CS_UM_CurrentTimescale),
            AchievementEvent(_) => User(CS_UM_AchievementEvent),
            MatchEndConditions(_) => User(CS_UM_MatchEndConditions),
            DisconnectToLobby(_) => User(CS_UM_DisconnectToLobby),
            PlayerStatsUpdate(_) => User(CS_UM_PlayerStatsUpdate),
            DisplayInventory(_) => User(CS_UM_DisplayInventory),
            WarmupHasEnded(_) => User(CS_UM_WarmupHasEnded),
            ClientInfo(_) => User(CS_UM_ClientInfo),
            XRankGet(_) => User(CS_UM_XRankGet),
            XRankUpd(_) => User(CS_UM_XRankUpd),
            CallVoteFailed(_) => User(CS_UM_CallVoteFailed),
            VoteStart(_) => User(CS_UM_VoteStart),
            VotePass(_) => User(CS_UM_VotePass),
            VoteFailed(_) => User(CS_UM_VoteFailed),
            VoteSetup(_) => User(CS_UM_VoteSetup),
            ServerRankRevealAll(_) => User(CS_UM_ServerRankRevealAll),
            SendLastKillerDamageToClient(_) => User(CS_UM_SendLastKillerDamageToClient),
            ServerRankUpdate(_) => User(CS_UM_ServerRankUpdate),
            ItemPickup(_) => User(CS_UM_ItemPickup),
            ShowMenu(_) => User(CS_UM_ShowMenu),
            BarTime(_) => User(CS_UM_BarTime),
            AmmoDenied(_) => User(CS_UM_AmmoDenied),
            MarkAchievement(_) => User(CS_UM_MarkAchievement),
            MatchStatsUpdate(_) => User(CS_UM_MatchStatsUpdate),
            ItemDrop(_) => User(CS_UM_ItemDrop),
            GlowPropTurnOff(_) => User(CS_UM_GlowPropTurnOff),
            SendPlayerItemDrops(_) => User(CS_UM_SendPlayerItemDrops),
            RoundBackupFilenames(_) => User(CS_UM_RoundBackupFilenames),
            SendPlayerItemFound(_) => User(CS_UM_SendPlayerItemFound),
            ReportHit(_) => User(CS_UM_ReportHit),
            XpUpdate(_) => User(CS_UM_XpUpdate),
            QuestProgress(_) => User(CS_UM_QuestProgress),
            ScoreLeaderboardData(_) => User(CS_UM_ScoreLeaderboardData),
            UpdateTeamMoney => User(CS_UM_UpdateTeamMoney),
            Unknown(_) => MessageKind::UnknownUser,
        }
    }
}
