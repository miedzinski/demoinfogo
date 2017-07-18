mod bits;
mod ext;

pub(crate) use self::ext::ReadExt;
pub(crate) use self::bits::BitReader;

pub(crate) trait FromStream: Sized {
    fn from_stream(stream: &mut ::protobuf::CodedInputStream) -> ::error::DemoResult<Self>;
}
