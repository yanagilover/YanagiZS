mod magic;
mod unpack;
mod util;

use std::{
    collections::HashMap,
    io::{Cursor, Read, Seek},
};

use byteorder::{ReadBytesExt, BE, LE};
pub use unpack::unpack_blk_raw;

pub struct ArchiveFile {
    data: Box<[u8]>,
    file_map: HashMap<String, (usize, usize)>,
}

impl ArchiveFile {
    pub fn from_raw(data: Box<[u8]>) -> std::io::Result<Self> {
        let mut reader = Cursor::new(&data);

        // Skip unity slop
        let header_size = reader.read_u32::<BE>()?;
        reader.seek_relative(header_size as i64)?;

        while let Ok(b) = reader.read_u8() {
            if b != 0 {
                reader.seek_relative(-1)?;
                break;
            }
        }

        let mut file_map = HashMap::new();

        // read to end
        while reader.read_u8().is_ok() {
            reader.seek_relative(-1)?;

            let file_name = reader.read_string()?;
            let mut file_size = reader.read_u32::<LE>()? as usize;
            while file_size & 0xFF == 0 {
                let b = reader.read_u8()?;
                reader.seek_relative(-1)?;

                // Beginning of data
                if b == 0xC || b == 0x10 {
                    break;
                }

                reader.seek_relative(-3)?;
                file_size = reader.read_u32::<LE>()? as usize;
            }
            if file_name.ends_with(".bundle") {
                // when .bundle encoded, it ends with name of itself.
                let mut buf = vec![0u8; file_name.len()];
                loop {
                    reader.seek_relative(1)?;
                    reader.read_exact(&mut buf)?;
                    if file_name.as_bytes() == &buf {
                        reader.seek_relative(0x15)?;
                        break;
                    }

                    reader.seek_relative(-(file_name.len() as i64))?;
                }
            } else {
                let offset = reader.stream_position()?;
                reader.seek_relative(file_size as i64)?; // skip data

                let eof = loop {
                    match reader.read_u8() {
                        Ok(0) => (),
                        Ok(_) => break false,
                        Err(_) => break true,
                    }
                };

                //println!("got file: {file_name}, size: {file_size} bytes");
                file_map.insert(file_name, (offset as usize, file_size));

                if eof {
                    break;
                }

                reader.seek_relative(-1)?;
            }
        }

        Ok(Self { data, file_map })
    }

    pub fn open(&self, file_name: &str) -> Option<&[u8]> {
        self.file_map
            .get(file_name)
            .map(|&(start, size)| &self.data[start..start + size])
    }
}

pub trait ReadHelper {
    fn read_string(&mut self) -> std::io::Result<String>;
}

impl<R: Read> ReadHelper for R {
    fn read_string(&mut self) -> std::io::Result<String> {
        let str_len = self.read_u32::<LE>()? as usize;
        let mut buf = vec![0u8; str_len];
        self.read_exact(&mut buf)?;

        Ok(String::from_utf8_lossy(&buf).to_string())
    }
}
