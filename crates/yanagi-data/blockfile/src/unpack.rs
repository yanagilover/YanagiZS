use std::{
    cmp,
    io::{self, Cursor, Read},
};

use byteorder::{ReadBytesExt, LE};

use crate::util::{sub_18001A770, MhyObscureReadExt};

#[derive(Debug)]
#[expect(unused)]
struct CabFileInfo {
    pub path: String,
    pub flag: bool,
    pub offset: usize,
    pub size: usize,
}

#[derive(Debug)]
struct BlockInfo {
    pub compressed_size: usize,
    pub decompressed_size: usize,
}

impl CabFileInfo {
    pub fn read<R: MhyObscureReadExt>(r: &mut R) -> io::Result<Self> {
        Ok(Self {
            path: r.read_path_string()?,
            flag: r.read_bool()?,
            offset: r.read_obscure_i32()? as usize,
            size: r.read_obscure_u32()? as usize,
        })
    }
}

impl BlockInfo {
    pub fn read<R: MhyObscureReadExt>(r: &mut R) -> io::Result<Self> {
        Ok(Self {
            compressed_size: r.read_obscure_i32()? as usize,
            decompressed_size: r.read_obscure_u32()? as usize,
        })
    }
}

pub fn unpack_blk_raw<R: Read>(mut buf: R) -> io::Result<Box<[u8]>> {
    let v45 = buf.read_u64::<LE>()?;
    assert_eq!((v45 as u32) | 0x1000000, 0x3179686D); // "mhy1"

    let v6 = (v45 >> 32) as usize;
    let mut v8 = vec![0u8; v6];
    buf.read_exact(&mut v8)?;

    sub_18001A770(&mut v8, cmp::min(v6, 128), 0x1C);
    if v6 >= 0x27 {
        let data = &v8[48..];
        let decompressed_size = (data[1] as u32)
            | ((data[6] as u32) << 8)
            | ((data[3] as u32) << 16)
            | ((data[2] as u32) << 24);

        let data = lz4_flex::block::decompress(&data[7..], decompressed_size as usize).unwrap();
        let mut reader = Cursor::new(&data);

        let _ = (0..reader.read_obscure_i32()?)
            .map(|_| CabFileInfo::read(&mut reader))
            .collect::<Result<Vec<CabFileInfo>, _>>()?;

        let compressed_blocks = (0..reader.read_obscure_i32()?)
            .map(|_| BlockInfo::read(&mut reader))
            .collect::<Result<Vec<BlockInfo>, _>>()?;

        let mut decompressed_data = vec![
            0u8;
            compressed_blocks
                .iter()
                .fold(0usize, |len, b| len + b.decompressed_size)
        ];

        let mut idx = 0;
        for block in compressed_blocks {
            let mut compressed = vec![0u8; block.compressed_size];
            buf.read(&mut compressed)?;

            sub_18001A770(&mut compressed, cmp::min(128, block.compressed_size), 8);
            lz4_flex::block::decompress_into(
                &compressed[28..],
                &mut decompressed_data[idx..idx + block.decompressed_size],
            )
            .unwrap();

            idx += block.decompressed_size;
        }

        Ok(decompressed_data.into_boxed_slice())
    } else {
        Ok(Vec::with_capacity(0).into_boxed_slice())
    }
}
