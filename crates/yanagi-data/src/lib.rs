#[allow(dead_code, unused_imports)]
#[path = "../gen_flatbuffers/tables_generated.rs"]
mod data;
use std::io::Read;

pub use blockfile::ArchiveFile;
pub use data::*;

macro_rules! file_cfg {
    ($($name:ident;)*) => {
        ::paste::paste!{
            pub struct NapFileCfg {
                $(pub [<$name:snake>]: $name<'static>,)*
        }}

        impl NapFileCfg {
            ::paste::paste!{
                pub fn new(archive_file: &'static ArchiveFile) -> Self {
                    Self {
                        $(
                            [<$name:snake>]: {
                                ::flatbuffers::root::<$name>(archive_file.open(::const_format::formatcp!("{}", ::xxhash_rust::const_xxh64::xxh64(stringify!([<$name:lower>]).as_bytes(), 0))).unwrap()).unwrap()
                            },
                        )*
                    }
                }
            }
        }
    };
}

file_cfg! {
    AvatarBaseTemplateTb;
    WeaponTemplateTb;
}

pub fn read_archive_file<R: Read>(buf: R) -> std::io::Result<ArchiveFile> {
    let raw_data = blockfile::unpack_blk_raw(buf)?;
    ArchiveFile::from_raw(raw_data)
}
