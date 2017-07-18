use std::io;
use std::mem;
use std::u32;

use error::DemoResult;
use read::ReadExt;

const NBITS: usize = 32;

static MASKS: [u32; NBITS + 1] = [
    0,
    u32::MAX >> 31,
    u32::MAX >> 30,
    u32::MAX >> 29,
    u32::MAX >> 28,
    u32::MAX >> 27,
    u32::MAX >> 26,
    u32::MAX >> 25,
    u32::MAX >> 24,
    u32::MAX >> 23,
    u32::MAX >> 22,
    u32::MAX >> 21,
    u32::MAX >> 20,
    u32::MAX >> 19,
    u32::MAX >> 18,
    u32::MAX >> 17,
    u32::MAX >> 16,
    u32::MAX >> 15,
    u32::MAX >> 14,
    u32::MAX >> 13,
    u32::MAX >> 12,
    u32::MAX >> 11,
    u32::MAX >> 10,
    u32::MAX >> 9,
    u32::MAX >> 8,
    u32::MAX >> 7,
    u32::MAX >> 6,
    u32::MAX >> 5,
    u32::MAX >> 4,
    u32::MAX >> 3,
    u32::MAX >> 2,
    u32::MAX >> 1,
    u32::MAX,
];

pub(crate) struct BitReader<R: io::Read> {
    inner: R,
    bits: u32,
    available: usize,
}

impl<R: io:: Read> BitReader<R> {
    pub fn new(reader: R) -> BitReader<R> {
        BitReader {
            inner: reader,
            bits: 0,
            available: 0,
        }
    }

    fn ensure_bits(&mut self) -> io::Result<()> {
        let mut buf = [0; NBITS / 8];
        self.inner.read_exact(&mut buf)?;
        self.bits = unsafe { mem::transmute(buf) };
        self.available = NBITS;
        Ok(())
    }

    fn consume(&mut self, n: usize) {
        self.bits = match n {
            NBITS => 0,
            n => self.bits >> n,
        };
        self.available -= n;
    }

    pub fn read_bool(&mut self) -> DemoResult<bool> {
        if self.available == 0 {
            self.ensure_bits()?;
        }
        let ret = self.bits & 1 == 1;
        self.consume(1);
        Ok(ret)
    }

    fn read_nbits(&mut self, n: usize) -> DemoResult<u32> {
        debug_assert!(n <= NBITS);
        
        if self.available >= n {
            let ret = self.bits & MASKS[n];
            self.consume(n);
            Ok(ret)
        } else {
            let in_buf = self.bits;
            let consumed = self.available;
            let remaining = n - consumed;
            self.ensure_bits()?;
            let ret = in_buf | ((self.bits & MASKS[remaining]) << consumed);
            self.consume(remaining);
            Ok(ret.to_le())
        }
    }
}

impl<R: io::Read> ReadExt for BitReader<R> {
    fn read_u8(&mut self) -> DemoResult<u8> {
        self.read_nbits(8).map(|x| x as u8)
    }

    fn read_u16(&mut self) -> DemoResult<u16> {
        self.read_nbits(16).map(|x| x as u16)
    }

    fn read_u32(&mut self) -> DemoResult<u32> {
        self.read_nbits(32).map(|x| x as u32)
    }

    fn read_f32(&mut self) -> DemoResult<f32> {
        self.read_nbits(32).map(|x| unsafe { mem::transmute(x) })
    }

    fn read_sized(&mut self) -> DemoResult<Vec<u8>> {
        let size = self.read_u32()? as usize;
        let mut buf = Vec::with_capacity(size);
        if self.available % 8 == 0 {
            let nbytes_read = self.available / 8;
            while self.available > 0 {
                buf.push(self.read_u8()?);
            }
            unsafe {
                buf.set_len(size);
            }
            self.inner.read_exact(&mut buf[nbytes_read..])?;
        } else {
            for _ in 0..size {
                buf.push(self.read_u8()?);
            }
        }
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
    use std::io;
    use std::{u8, u16, u32, f32};

    use error::DemoError;
    use read::ReadExt;
    use super::{BitReader, NBITS};

    #[test]
    fn read_bool_from_dword() {
        let buf = io::Cursor::new([0b00110101; 4]);
        let mut reader = BitReader::new(buf);
        for _ in 0..4 {
            assert!(reader.read_bool().unwrap());
            assert!(!reader.read_bool().unwrap());
            assert!(reader.read_bool().unwrap());
            assert!(!reader.read_bool().unwrap());
            assert!(reader.read_bool().unwrap());
            assert!(reader.read_bool().unwrap());
            assert!(!reader.read_bool().unwrap());
            assert!(!reader.read_bool().unwrap());
        }
        assert!(reader.read_bool().is_err());
    }

    #[test]
    fn read_u8_zeroed() {
        let mut reader = BitReader::new(io::Cursor::new([0; 4]));
        assert_eq!(reader.read_u8().unwrap(), 0);
    }

    #[test]
    fn read_u8_max() {
        let mut reader = BitReader::new(io::Cursor::new([u8::MAX; 4]));
        assert_eq!(reader.read_u8().unwrap(), u8::MAX);
    }

    #[test]
    fn read_u8_some() {
        let mut reader = BitReader::new(io::Cursor::new([1u8; 4]));
        assert_eq!(reader.read_u8().unwrap(), 1);
    }

    #[test]
    fn read_u16_zeroed() {
        let mut reader = BitReader::new(io::Cursor::new([0; 4]));
        assert_eq!(reader.read_u16().unwrap(), 0);
    }

    #[test]
    fn read_u16_max() {
        let mut reader = BitReader::new(io::Cursor::new([u8::MAX; 4]));
        assert_eq!(reader.read_u16().unwrap(), u16::MAX);
    }

    #[test]
    fn read_u16_some() {
        let mut reader = BitReader::new(io::Cursor::new([1, 2, 3, 4]));
        assert_eq!(reader.read_u16().unwrap(), 513);
    }

    #[test]
    fn read_u32_zeroed() {
        let mut reader = BitReader::new(io::Cursor::new([0; 4]));
        assert_eq!(reader.read_u32().unwrap(), 0);
    }

    #[test]
    fn read_u32_max() {
        let mut reader = BitReader::new(io::Cursor::new([u8::MAX; 4]));
        assert_eq!(reader.read_u32().unwrap(), u32::MAX);
    }

    #[test]
    fn read_u32_some() {
        let mut reader = BitReader::new(io::Cursor::new([1, 2, 3, 4]));
        assert_eq!(reader.read_u32().unwrap(), 67305985);
    }

    #[test]
    fn read_f32_zeroed() {
        let mut reader = BitReader::new(io::Cursor::new([0; 4]));
        assert_eq!(reader.read_f32().unwrap(), 0f32);
    }

    #[test]
    fn read_f32_max() {
        let mut reader = BitReader::new(io::Cursor::new([0xff, 0xff, 0x7f, 0x7f]));
        assert_eq!(reader.read_f32().unwrap(), f32::MAX);
    }

    #[test]
    fn read_f32_epsilon() {
        let mut reader = BitReader::new(io::Cursor::new([0, 0, 0, 0x34]));
        assert_eq!(reader.read_f32().unwrap(), f32::EPSILON);
    }

    #[test]
    fn read_f32_min_positive() {
        let mut reader = BitReader::new(io::Cursor::new([0, 0, 0x80, 0]));
        assert_eq!(reader.read_f32().unwrap(), f32::MIN_POSITIVE);
    }

    #[test]
    fn read_f32_some() {
        let mut reader = BitReader::new(io::Cursor::new([0, 0x40, 0x9a, 0x44]));
        assert_eq!(reader.read_f32().unwrap(), 1234f32);
    }

    #[test]
    fn read_u8_misaligned() {
        let mut reader = BitReader::new(io::Cursor::new([129, 0, 0, 0]));
        assert!(reader.read_bool().unwrap());
        assert_eq!(reader.read_u8().unwrap(), 64);
        assert_eq!(reader.available, NBITS - 9);
    }

    #[test]
    fn read_u16_misaligned() {
        let mut reader = BitReader::new(io::Cursor::new([129, 64, 0, 0]));
        assert!(reader.read_bool().unwrap());
        assert_eq!(reader.read_u16().unwrap(), 8256);
        assert_eq!(reader.available, NBITS - 17);
    }

    #[test]
    fn read_sized_eof() {
        let bytes = [
            5, 0, 0, 0, // 5 bytes
            1, 2, 3, 4,
        ];
        let mut reader = BitReader::new(io::Cursor::new(bytes));
        match reader.read_sized() {
            Err(DemoError::Io(ref err)) if err.kind() == io::ErrorKind::UnexpectedEof => (),
            x => panic!("{:?}", x),
        }
    }

    #[test]
    fn read_sized_ok() {
        let bytes = [
            8, 0, 0, 0, // 8 bytes
            1, 2, 3, 4, // data
            5, 6, 7, 8,
            1, 2, 3, 4, // rest
        ];
        let mut reader = BitReader::new(io::Cursor::new(bytes));
        assert!(reader.ensure_bits().is_ok()); // make sure it bits in buffer are handled
        assert_eq!(reader.read_sized().unwrap(), &[1, 2, 3, 4, 5, 6, 7, 8]);
        assert!(reader.read_u32().is_ok());
    }

    #[test]
    fn read_sized_misaligned() {
        let bytes = [
            9, 0, 0, 0, 0, // true + 4
            63, 32, 16, 8, // data
            1, 2, 3, // junk
        ];
        let mut reader = BitReader::new(io::Cursor::new(bytes));
        assert!(reader.read_bool().unwrap());
        assert_eq!(reader.read_sized().unwrap(), &[128, 31, 16, 8]);
        assert_eq!(reader.available, NBITS - 1);
    }

    #[test]
    fn read_cstring_ok() {
        let bytes = b"foo\0";
        let mut reader = BitReader::new(io::Cursor::new(bytes));
        assert_eq!(reader.read_cstring().unwrap(), format!("foo"));
    }

    #[test]
    fn read_cstring_eof() {
        let bytes = b"fooo";
        let mut reader = BitReader::new(io::Cursor::new(bytes));
        match reader.read_cstring() {
            Err(DemoError::Io(ref err)) if err.kind() == io::ErrorKind::UnexpectedEof => (),
            x => panic!("{:?}", x),
        }
    }
}
