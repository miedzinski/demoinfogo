use protobuf::CodedInputStream;

use error::DemoResult;
use read::{FromStream, ReadExt};

#[derive(Debug)]
pub struct ConsoleCommand {
    cmd: String,
}

impl ConsoleCommand {
    pub fn cmd(&self) -> &str {
        &self.cmd
    }
}

impl FromStream for ConsoleCommand {
    fn from_stream(stream: &mut CodedInputStream) -> DemoResult<ConsoleCommand> {
        let mut cmd = String::from_utf8(stream.read_sized()?)?;
        // XXX
        if let Some(idx) = cmd.find('\0') {
            cmd.truncate(idx);
        }
        Ok(ConsoleCommand { cmd })
    }
}

// https://developer.valvesoftware.com/wiki/Usercmd
// https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/shared/usercmd.cpp
#[derive(Debug)]
pub struct UserCommand {
    seq: u32,
    // XXX
    data: Vec<u8>,
}

impl FromStream for UserCommand {
    fn from_stream(stream: &mut CodedInputStream) -> DemoResult<UserCommand> {
        Ok(UserCommand {
            seq: stream.read_u32()?,
            data: stream.read_sized()?,
        })
    }
}

impl UserCommand {
    pub fn seq(&self) -> u32 {
        self.seq
    }
}

#[cfg(test)]
mod tests {
    use tests::read;
    use super::{ConsoleCommand, UserCommand};

    #[test]
    fn console_ok() {
        let cmd: ConsoleCommand = read("consolecmd.bin").unwrap();
        assert_eq!(cmd.cmd(), "lastinv");
    }

    #[test]
    fn user_ok() {
        let cmd: UserCommand = read("usercmd.bin").unwrap();
        assert_eq!(cmd.seq(), 1077);
        assert_eq!(cmd.data.len(), 18);
    }
}
