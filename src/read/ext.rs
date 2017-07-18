use std::io;
use std::mem;

use error::DemoResult;

pub(crate) trait ReadExt {
    fn read_u8(&mut self) -> DemoResult<u8>;
    fn read_u16(&mut self) -> DemoResult<u16>;
    fn read_u32(&mut self) -> DemoResult<u32>;
    fn read_f32(&mut self) -> DemoResult<f32>;
    fn read_sized(&mut self) -> DemoResult<Vec<u8>>;
    fn read_cstring(&mut self) -> DemoResult<String>;
}

impl<R: io::Read> ReadExt for R {
    #[inline]
    fn read_u8(&mut self) -> DemoResult<u8> {
        let mut buf = [0];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    #[inline]
    fn read_u16(&mut self) -> DemoResult<u16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        let n: u16 = unsafe { mem::transmute(buf) };
        Ok(n.to_le())
    }

    #[inline]
    fn read_u32(&mut self) -> DemoResult<u32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        let n: u32 = unsafe { mem::transmute(buf) };
        Ok(n.to_le())
    }

    #[inline]
    fn read_f32(&mut self) -> DemoResult<f32> {
        unsafe { Ok(mem::transmute(self.read_u32()?)) }
    }

    fn read_sized(&mut self) -> DemoResult<Vec<u8>> {
        let size = self.read_u32()? as usize;
        let mut buf = Vec::with_capacity(size);
        unsafe {
            buf.set_len(size);
        }
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn read_cstring(&mut self) -> DemoResult<String> {
        let mut v = vec![];
        loop {
            let c = self.read_u8()?;
            if c != 0 {
                v.push(c)
            } else {
                break;
            }
        }
        Ok(String::from_utf8(v)?)
    }
}

#[cfg(test)]
mod tests {
    use std::{u8, u16, u32, f32};
    use std::io;

    use protobuf::CodedInputStream;

    use error::DemoError;
    use super::ReadExt;

    #[test]
    fn read_u8_zeroed() {
        let bytes = [0; 1];
        let mut stream = CodedInputStream::from_bytes(&bytes);
        assert_eq!(stream.read_u8().unwrap(), 0);
    }

    #[test]
    fn read_u8_max() {
        let bytes = [u8::MAX];
        let mut stream = CodedInputStream::from_bytes(&bytes);
        assert_eq!(stream.read_u8().unwrap(), u8::MAX);
    }

    #[test]
    fn read_u8_some() {
        let bytes = [1];
        let mut stream = CodedInputStream::from_bytes(&bytes);
        assert_eq!(stream.read_u8().unwrap(), 1);
    }

    #[test]
    fn read_u16_zeroed() {
        let bytes = [0; 2];
        let mut stream = CodedInputStream::from_bytes(&bytes);
        assert_eq!(stream.read_u16().unwrap(), 0);
    }

    #[test]
    fn read_u16_max() {
        let bytes = [u8::MAX; 2];
        let mut stream = CodedInputStream::from_bytes(&bytes);
        assert_eq!(stream.read_u16().unwrap(), u16::MAX);
    }

    #[test]
    fn read_u16_some() {
        let bytes = [1, 2];
        let mut stream = CodedInputStream::from_bytes(&bytes);
        assert_eq!(stream.read_u16().unwrap(), 513);
    }

    #[test]
    fn read_u32_zeroed() {
        let bytes = [0; 4];
        let mut stream = CodedInputStream::from_bytes(&bytes);
        assert_eq!(stream.read_u32().unwrap(), 0);
    }

    #[test]
    fn read_u32_max() {
        let bytes = [u8::MAX; 4];
        let mut stream = CodedInputStream::from_bytes(&bytes);
        assert_eq!(stream.read_u32().unwrap(), u32::MAX);
    }

    #[test]
    fn read_u32_some() {
        let bytes = [1, 2, 3, 4];
        let mut stream = CodedInputStream::from_bytes(&bytes);
        assert_eq!(stream.read_u32().unwrap(), 67305985);
    }

    #[test]
    fn read_f32_zeroed() {
        let bytes = [0; 4];
        let mut stream = CodedInputStream::from_bytes(&bytes);
        assert_eq!(stream.read_f32().unwrap(), 0f32);
    }

    #[test]
    fn read_f32_max() {
        let bytes = [0xff, 0xff, 0x7f, 0x7f];
        let mut stream = CodedInputStream::from_bytes(&bytes);
        assert_eq!(stream.read_f32().unwrap(), f32::MAX);
    }

    #[test]
    fn read_f32_epsilon() {
        let bytes = [0, 0, 0, 0x34];
        let mut stream = CodedInputStream::from_bytes(&bytes);
        assert_eq!(stream.read_f32().unwrap(), f32::EPSILON);
    }

    #[test]
    fn read_f32_min_positive() {
        let bytes = [0, 0, 0x80, 0];
        let mut stream = CodedInputStream::from_bytes(&bytes);
        assert_eq!(stream.read_f32().unwrap(), f32::MIN_POSITIVE);
    }

    #[test]
    fn read_f32_some() {
        let bytes = [0, 0x40, 0x9a, 0x44];
        let mut stream = CodedInputStream::from_bytes(&bytes);
        assert_eq!(stream.read_f32().unwrap(), 1234f32);
    }

    #[test]
    fn read_sized_eof() {
        let bytes = [
            5, 0, 0, 0, // 5 bytes
            1, 2, 3, 4,
        ];
        let mut stream = CodedInputStream::from_bytes(&bytes);
        match stream.read_sized() {
            Err(DemoError::Io(ref err)) if err.kind() == io::ErrorKind::UnexpectedEof => (),
            x => panic!("{:?}", x),
        }
    }

    #[test]
    fn read_sized_ok() {
        let bytes = [
            4, 0, 0, 0, // 4 bytes
            1, 2, 3, 4, // data
            5, 6, 7, 8, // rest
        ];
        let mut stream = CodedInputStream::from_bytes(&bytes);
        assert_eq!(stream.read_sized().unwrap(), &[1, 2, 3, 4]);
        assert_eq!(stream.pos(), 8);
    }

    #[test]
    fn read_cstring_ok() {
        let bytes = b"foo\0";
        let mut stream = CodedInputStream::from_bytes(&bytes[..]);
        assert_eq!(stream.read_cstring().unwrap(), format!("foo"));
    }

    #[test]
    fn read_cstring_eof() {
        let bytes = b"fooo";
        let mut stream = CodedInputStream::from_bytes(&bytes[..]);
        match stream.read_cstring() {
            Err(DemoError::Io(ref err)) if err.kind() == io::ErrorKind::UnexpectedEof => (),
            x => panic!("{:?}", x),
        }
    }
}
