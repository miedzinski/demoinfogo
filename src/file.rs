use std::io::{BufRead, Read};

use protobuf::CodedInputStream;

use cmd::{ConsoleCommand, UserCommand};
use error::{DemoError, DemoResult};
use frame::Frame;
use read::{FromStream, ReadExt};
use tables::{DataTables, StringTables};

const DEMO_MAGIC_LEN: usize = 8;
const DEMO_MAGIC: &[u8; DEMO_MAGIC_LEN] = b"HL2DEMO\0";
const DEMO_PROTOCOL: u32 = 4;
const GAME_DIRECTORY: &str = "csgo";
const HEADER_STR_LEN: usize = 260;

#[derive(Debug)]
struct Header {
    network_protocol: u32,
    server_name: String,
    client_name: String,
    map_name: String,
    playback_time: f32,
    playback_ticks: u32,
    playback_frames: u32,
    signon_length: u32,
}

impl FromStream for Header {
    fn from_stream(stream: &mut CodedInputStream) -> DemoResult<Header> {
        let mut magic = [0; DEMO_MAGIC_LEN];
        stream.read(&mut magic)?;

        if magic != *DEMO_MAGIC {
            return Err(DemoError::MagicMismatch(Box::new(magic)));
        }

        let demo_protocol = stream.read_u32()?;

        if demo_protocol != DEMO_PROTOCOL {
            return Err(DemoError::DemoProtocolMismatch(demo_protocol));
        }

        let network_protocol = stream.read_u32()?;
        let server_name = Self::read_string(stream)?;
        let client_name = Self::read_string(stream)?;
        let map_name = Self::read_string(stream)?;
        let game_directory = Self::read_string(stream)?;

        if game_directory != GAME_DIRECTORY {
            return Err(DemoError::GameDirectoryMismatch(game_directory));
        }

        Ok(Header {
            network_protocol,
            server_name,
            client_name,
            map_name,
            playback_time: stream.read_f32()?,
            playback_ticks: stream.read_u32()?,
            playback_frames: stream.read_u32()?,
            signon_length: stream.read_u32()?,
        })
    }
}

impl Header {
    fn read_string(stream: &mut CodedInputStream) -> DemoResult<String> {
        let mut v = Vec::with_capacity(HEADER_STR_LEN);
        unsafe {
            v.set_len(HEADER_STR_LEN);
        }
        stream.read(v.as_mut_slice())?;
        let mut s = String::from_utf8(v)?;
        if let Some(idx) = s.find('\0') {
            s.truncate(idx)
        }
        Ok(s)
    }
}

#[derive(Debug)]
pub enum Command {
    Signon(Frame),
    Packet(Frame),
    SyncTick,
    ConsoleCmd(ConsoleCommand),
    UserCmd(UserCommand),
    DataTables(DataTables),
    Stop,
    CustomData,
    StringTables(StringTables),
}

#[derive(Debug)]
pub struct Packet {
    tick: u32,
    player_slot: u8,
    cmd: Command,
}

impl FromStream for Packet {
    fn from_stream(stream: &mut CodedInputStream) -> DemoResult<Packet> {
        let cmd_type = stream.read_u8()?;
        let tick = stream.read_u32()?;
        let player_slot = stream.read_u8()?;

        let cmd = match cmd_type {
            1 => Command::Signon(Frame::from_stream(stream)?),
            2 => Command::Packet(Frame::from_stream(stream)?),
            3 => Command::SyncTick,
            4 => Command::ConsoleCmd(ConsoleCommand::from_stream(stream)?),
            5 => Command::UserCmd(UserCommand::from_stream(stream)?),
            6 => Command::DataTables(DataTables::from_stream(stream)?),
            7 => Command::Stop,
            8 => Command::CustomData,
            9 => Command::StringTables(StringTables::from_stream(stream)?),
            cmd => {
                return Err(DemoError::UnknownCommand(cmd));
            }
        };

        Ok(Packet {
            tick,
            player_slot,
            cmd,
        })
    }
}

impl Packet {
    pub fn tick(&self) -> u32 {
        self.tick
    }

    pub fn player_slot(&self) -> u8 {
        self.player_slot
    }

    pub fn cmd(&self) -> &Command {
        &self.cmd
    }

    pub fn is_signon(&self) -> bool {
        matches!(self.cmd, Command::Signon(_))
    }

    pub fn is_packet(&self) -> bool {
        matches!(self.cmd, Command::Packet(_))
    }

    pub fn is_synctick(&self) -> bool {
        matches!(self.cmd, Command::SyncTick)
    }

    pub fn is_consolecmd(&self) -> bool {
        matches!(self.cmd, Command::ConsoleCmd(_))
    }

    pub fn is_usercmd(&self) -> bool {
        matches!(self.cmd, Command::UserCmd(_))
    }

    pub fn is_datatables(&self) -> bool {
        matches!(self.cmd, Command::DataTables(_))
    }

    pub fn is_stop(&self) -> bool {
        matches!(self.cmd, Command::Stop)
    }

    pub fn is_customdata(&self) -> bool {
        matches!(self.cmd, Command::CustomData)
    }

    pub fn is_stringtables(&self) -> bool {
        matches!(self.cmd, Command::StringTables(_))
    }
}

pub struct Demo<'a> {
    stream: CodedInputStream<'a>,
    header: Header,
    finished: bool,
}

impl<'a> Demo<'a> {
    fn new(mut stream: CodedInputStream<'a>) -> DemoResult<Demo<'a>> {
        let header = Header::from_stream(&mut stream)?;
        Ok(Demo {
            stream,
            header,
            finished: false,
        })
    }

    pub fn from_reader(reader: &'a mut Read) -> DemoResult<Demo<'a>> {
        Self::new(CodedInputStream::new(reader))
    }

    pub fn from_buffered_reader(reader: &'a mut BufRead) -> DemoResult<Demo<'a>> {
        Self::new(CodedInputStream::from_buffered_reader(reader))
    }
}

impl<'a> Demo<'a> {
    pub fn network_protocol(&self) -> u32 {
        self.header.network_protocol
    }

    pub fn server_name(&self) -> &str {
        &self.header.server_name
    }

    pub fn client_name(&self) -> &str {
        &self.header.client_name
    }

    pub fn map_name(&self) -> &str {
        &self.header.map_name
    }

    pub fn playback_time(&self) -> f32 {
        self.header.playback_time
    }

    pub fn playback_ticks(&self) -> u32 {
        self.header.playback_ticks
    }

    pub fn playback_frames(&self) -> u32 {
        self.header.playback_frames
    }

    pub fn signon_length(&self) -> u32 {
        self.header.signon_length
    }

    pub fn tickrate(&self) -> f32 {
        self.header.playback_ticks as f32 / self.header.playback_time
    }
}

impl<'a> Iterator for Demo<'a> {
    type Item = DemoResult<Packet>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.finished {
            let packet = Packet::from_stream(&mut self.stream);
            if matches!(packet, Ok(Packet { cmd: Command::Stop, .. })) {
                self.finished = true;
            }
            Some(packet)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use error::{DemoError, DemoResult};
    use tests::{read, test_case};
    use super::{Demo, Header};

    #[test]
    fn header_ok() {
        let header: Header = read("header-ok.bin").unwrap();
        assert_eq!(header.network_protocol, 13585);
        assert_eq!(header.server_name, "localhost:27015");
        assert_eq!(header.client_name, "Player");
        assert_eq!(header.map_name, "de_dust2");
        assert_eq!(header.playback_time, 13.796875);
        assert_eq!(header.playback_ticks, 883);
        assert_eq!(header.playback_frames, 724);
        assert_eq!(header.signon_length, 452111);
    }

    #[test]
    fn header_magic_mismatch() {
        let res: DemoResult<Header> = read("header-magic-mismatch.bin");
        assert_matches!(res, Err(DemoError::MagicMismatch(ref magic)) if &**magic == b"HL3DEMO\0");
    }

    #[test]
    fn header_demo_protocol_mismatch() {
        let res: DemoResult<Header> = read("header-demo-proto-mismatch.bin");
        assert_matches!(res, Err(DemoError::DemoProtocolMismatch(42)));
    }

    #[test]
    fn header_game_directory_mismatch() {
        let res: DemoResult<Header> = read("header-gamedir-mismatch.bin");
        assert_matches!(res, Err(DemoError::GameDirectoryMismatch(ref dir)) if dir == "csgo2");
    }

    #[test]
    fn demo_tickrate() {
        let mut file = test_case("ok.dem");
        let demo = Demo::from_reader(&mut file).unwrap();
        assert_eq!(demo.tickrate(), 64f32);
    }

    #[test]
    fn iterates_to_end() {
        let mut file = test_case("ok.dem");
        let demo = Demo::from_reader(&mut file).unwrap();
        let packets: Vec<_> = demo.map(Result::unwrap).collect();
        assert_eq!(packets.len(), 1651);
        assert!(packets.first().unwrap().is_signon());
        assert!(packets.last().unwrap().is_stop());
    }
}
