use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::marker::PhantomData;

use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;

use crate::ChecksumReader;
use crate::ChecksumWriter;
use crate::Decode;
use crate::Encode;
use crate::FixedSize;
use crate::OffsetSize;

/// Describe a segment with offset and size.
#[derive(Debug, Clone, Copy)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Segment<T = ()> {
    /// Offset of the segment.
    pub offset: u64,

    /// Size of the segment.
    pub size: u64,

    _p: PhantomData<T>,
}

impl<T> Segment<T> {
    /// Create a new segment.
    pub fn new(offset: u64, size: u64) -> Self {
        Self {
            offset,
            size,
            _p: Default::default(),
        }
    }
}

impl<T> OffsetSize for Segment<T> {
    fn offset(&self) -> u64 {
        self.offset
    }

    fn size(&self) -> u64 {
        self.size
    }
}

impl FixedSize for Segment {
    fn encoded_size() -> usize {
        // offset, size, checksum
        8 + 8 + 8
    }
}

impl Encode for Segment {
    fn encode<W: Write>(&self, mut w: W) -> Result<usize, Error> {
        let mut n = 0;

        let mut cw = ChecksumWriter::new(&mut w);

        cw.write_u64::<BigEndian>(self.offset)?;
        n += 8;

        cw.write_u64::<BigEndian>(self.size)?;
        n += 8;

        n += cw.write_checksum()?;

        Ok(n)
    }
}

impl Decode for Segment {
    fn decode<R: Read>(mut r: R) -> Result<Self, Error> {
        let mut cr = ChecksumReader::new(&mut r);

        let offset = cr.read_u64::<BigEndian>()?;
        let size = cr.read_u64::<BigEndian>()?;

        cr.verify_checksum(|| "Segment::decode()")?;

        Ok(Self {
            offset,
            size,
            _p: Default::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::segment::Segment;
    use crate::testing::test_codec;

    #[test]
    fn test_segment_codec() -> anyhow::Result<()> {
        let s = Segment {
            offset: 5,
            size: 10,
            _p: Default::default(),
        };

        let b = vec![
            0, 0, 0, 0, 0, 0, 0, 5, // offset
            0, 0, 0, 0, 0, 0, 0, 10, // size
            0, 0, 0, 0, 70, 249, 231, 4, // checksum
        ];

        test_codec(&b, &s)?;

        Ok(())
    }
}
