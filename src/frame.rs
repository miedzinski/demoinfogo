use protobuf::CodedInputStream;

use error::{DemoError, DemoResult};
use message::Message;
use read::{FromStream, ReadExt};
use vectors::{QAngle, Vector};

const MAX_SPLITS: usize = 2;

bitflags! {
    pub struct SplitFlags: u32 {
        const USE_RESAMPLED_ORIGIN = 1 << 0;
        const USE_RESAMPLED_ANGLES = 1 << 1;
        const NO_INTERP = 1 << 2;
    }
}

impl FromStream for SplitFlags {
    fn from_stream(stream: &mut CodedInputStream) -> DemoResult<SplitFlags> {
        let bits = stream.read_u32()?;
        Self::from_bits(bits).ok_or_else(|| DemoError::UnknownSplitFlags(bits))
    }
}

#[derive(Debug)]
pub struct Split {
    flags: SplitFlags,
    original_view_origin: Vector,
    original_view_angle: QAngle,
    original_local_view_angle: QAngle,
    resampled_view_origin: Vector,
    resampled_view_angle: QAngle,
    resampled_local_view_angle: QAngle,
}

impl FromStream for Split {
    fn from_stream(stream: &mut CodedInputStream) -> DemoResult<Split> {
        Ok(Split {
            flags: SplitFlags::from_stream(stream)?,
            original_view_origin: Vector::from_stream(stream)?,
            original_view_angle: QAngle::from_stream(stream)?,
            original_local_view_angle: QAngle::from_stream(stream)?,
            resampled_view_origin: Vector::from_stream(stream)?,
            resampled_view_angle: QAngle::from_stream(stream)?,
            resampled_local_view_angle: QAngle::from_stream(stream)?,
        })
    }
}

impl Split {
    pub fn flags(&self) -> &SplitFlags {
        &self.flags
    }

    pub fn view_origin(&self) -> &Vector {
        if !self.flags.contains(USE_RESAMPLED_ORIGIN) {
            &self.original_view_origin
        } else {
            &self.resampled_view_origin
        }
    }

    pub fn view_angle(&self) -> &QAngle {
        if !self.flags.contains(USE_RESAMPLED_ANGLES) {
            &self.original_view_angle
        } else {
            &self.resampled_view_angle
        }
    }

    pub fn local_view_angle(&self) -> &QAngle {
        if !self.flags.contains(USE_RESAMPLED_ANGLES) {
            &self.original_local_view_angle
        } else {
            &self.resampled_local_view_angle
        }
    }

    pub fn original_view_origin(&self) -> &Vector {
        &self.original_view_origin
    }

    pub fn original_view_angle(&self) -> &QAngle {
        &self.original_view_angle
    }

    pub fn original_local_view_angle(&self) -> &QAngle {
        &self.original_local_view_angle
    }

    pub fn resampled_view_origin(&self) -> &Vector {
        &self.resampled_view_origin
    }

    pub fn resampled_view_angle(&self) -> &QAngle {
        &self.resampled_view_angle
    }

    pub fn resampled_local_view_angle(&self) -> &QAngle {
        &self.resampled_local_view_angle
    }
}

// https://github.com/ValveSoftware/source-sdk-2013/blob/0d8dceea4310fde5706b3ce1c70609d72a38efdf/mp/src/public/demofile/demoformat.h#L73-L157
#[derive(Debug)]
pub struct Frame {
    splits: [Split; MAX_SPLITS],
    seq_in: u32,
    seq_out: u32,
    messages: Vec<Message>,
}

impl FromStream for Frame {
    fn from_stream(stream: &mut CodedInputStream) -> DemoResult<Frame> {
        Ok(Self {
            splits: [Split::from_stream(stream)?, Split::from_stream(stream)?],
            seq_in: stream.read_u32()?,
            seq_out: stream.read_u32()?,
            messages: Self::read_messages(stream)?,
        })
    }
}

impl Frame {
    pub fn splits(&self) -> &[Split; MAX_SPLITS] {
        &self.splits
    }

    pub fn seq_in(&self) -> u32 {
        self.seq_in
    }

    pub fn seq_out(&self) -> u32 {
        self.seq_out
    }

    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn read_messages(stream: &mut CodedInputStream) -> DemoResult<Vec<Message>> {
        let size = stream.read_u32()? as u64;
        let mut bytes_read = 0;
        let mut messages = vec![];

        while bytes_read < size {
            let start_pos = stream.pos();
            let msg = Message::from_stream(stream)?;
            bytes_read += stream.pos() - start_pos;
            messages.push(msg);
        }

        Ok(messages)
    }
}

#[cfg(test)]
mod tests {
    use csgoproto::netmessages::{NET_Messages, SVC_Messages};
    use protobuf::CodedInputStream;

    use error::DemoError;
    use message::{Message, MessageKind};
    use read::FromStream;
    use tests::read;
    use super::{Frame, Split, SplitFlags, USE_RESAMPLED_ORIGIN, USE_RESAMPLED_ANGLES};

    #[test]
    fn splitflags_unknown() {
        let bytes = [::std::u8::MAX; 4];
        let mut stream = CodedInputStream::from_bytes(&bytes);
        let flags = SplitFlags::from_stream(&mut stream);
        assert_matches!(flags, Err(DemoError::UnknownSplitFlags(::std::u32::MAX)));
    }

    #[test]
    fn splitflags_ok() {
        let bytes = [0b111, 0, 0, 0];
        let mut stream = CodedInputStream::from_bytes(&bytes);
        let flags = SplitFlags::from_stream(&mut stream).unwrap();
        assert_eq!(flags, SplitFlags::all());
    }

    #[test]
    fn split_ok() {
        let split: Split = read("split-ok.bin").unwrap();

        assert_eq!(split.flags(), &SplitFlags::empty());

        assert_eq!(split.original_view_origin().x(), -1371.797f32);
        assert_eq!(split.original_view_origin().y(), -487.60672f32);
        assert_eq!(split.original_view_origin().z(), 131.49947f32);

        assert_eq!(split.original_view_angle().x(), 4.232799f32);
        assert_eq!(split.original_view_angle().y(), 129.23859f32);
        assert_eq!(split.original_view_angle().z(), 0f32);

        assert_eq!(split.original_local_view_angle().x(), 4.232799f32);
        assert_eq!(split.original_local_view_angle().y(), 129.23859f32);
        assert_eq!(split.original_local_view_angle().z(), 0f32);

        assert_eq!(split.resampled_view_origin().x(), 0f32);
        assert_eq!(split.resampled_view_origin().y(), 0f32);
        assert_eq!(split.resampled_view_origin().z(), 0f32);

        assert_eq!(split.resampled_view_angle().x(), 0f32);
        assert_eq!(split.resampled_view_angle().y(), 0f32);
        assert_eq!(split.resampled_view_angle().z(), 0f32);

        assert_eq!(split.resampled_local_view_angle().x(), 0f32);
        assert_eq!(split.resampled_local_view_angle().y(), 0f32);
        assert_eq!(split.resampled_local_view_angle().z(), 0f32);
    }

    #[test]
    fn split_use_resampled_origin() {
        let split: Split = read("split-resampled.bin").unwrap();

        assert!(split.flags().contains(USE_RESAMPLED_ORIGIN));

        assert_eq!(split.resampled_view_origin().x(), -1371.797f32);
        assert_eq!(split.resampled_view_origin().y(), -487.60672f32);
        assert_eq!(split.resampled_view_origin().z(), 131.49947f32);

        assert_eq!(split.view_origin(), split.resampled_view_origin());
    }

    #[test]
    fn split_use_resampled_angles() {
        let split: Split = read("split-resampled.bin").unwrap();

        assert!(split.flags().contains(USE_RESAMPLED_ANGLES));

        assert_eq!(split.resampled_view_angle().x(), 4.232799f32);
        assert_eq!(split.resampled_view_angle().y(), 129.23859f32);
        assert_eq!(split.resampled_view_angle().z(), 0f32);

        assert_eq!(split.resampled_local_view_angle().x(), 4.232799f32);
        assert_eq!(split.resampled_local_view_angle().y(), 129.23859f32);
        assert_eq!(split.resampled_local_view_angle().z(), 0f32);

        assert_eq!(split.view_angle(), split.resampled_view_angle());
        assert_eq!(split.local_view_angle(), split.resampled_local_view_angle());
    }

    #[test]
    fn frame_ok() {
        let frame: Frame = read("frame-ok.bin").unwrap();

        assert_eq!(frame.seq_in(), 912);
        assert_eq!(frame.seq_out(), 1107);

        let kinds: Vec<_> = frame.messages()
            .iter()
            .map(Message::kind)
            .collect();
        let expected = [
            MessageKind::Net(NET_Messages::net_Tick),
            MessageKind::Svc(SVC_Messages::svc_PacketEntities),
            MessageKind::Svc(SVC_Messages::svc_Sounds),
        ];

        assert_eq!(kinds, expected);
    }
}
