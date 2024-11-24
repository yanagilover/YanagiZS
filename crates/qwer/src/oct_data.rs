use byteorder::{ReadBytesExt, WriteBytesExt};
use paste::paste;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io::{Read, Result, Write};

pub trait OctData: Send + Sync {
    fn marshal_to<W: Write>(&self, w: &mut W, serializer_tag: u16) -> Result<()>;
    fn unmarshal_from<R: Read>(r: &mut R, serializer_tag: u16) -> Result<Self>
    where
        Self: Sized;
}

impl OctData for bool {
    fn marshal_to<W: Write>(&self, w: &mut W, _: u16) -> Result<()> {
        w.write_u8(u8::from(*self))
    }

    fn unmarshal_from<R: Read>(r: &mut R, _: u16) -> Result<Self> {
        Ok(r.read_u8()? != 0)
    }
}

impl OctData for u8 {
    fn marshal_to<W: Write>(&self, w: &mut W, _: u16) -> Result<()> {
        w.write_u8(*self)
    }

    fn unmarshal_from<R: Read>(r: &mut R, _: u16) -> Result<Self> {
        r.read_u8()
    }
}

impl OctData for i8 {
    fn marshal_to<W: Write>(&self, w: &mut W, _: u16) -> Result<()> {
        w.write_i8(*self)
    }

    fn unmarshal_from<R: Read>(r: &mut R, _: u16) -> Result<Self> {
        r.read_i8()
    }
}

macro_rules! impl_primitive {
    ($($t:ty,)*) => {
        $(
            impl OctData for $t {
                fn marshal_to<W: Write>(&self, w: &mut W, _: u16) -> Result<()> {
                    paste!(w.[<write_ $t>]::<byteorder::LittleEndian>(*self))
                }

                fn unmarshal_from<R: Read>(r: &mut R, _: u16) -> Result<Self> {
                    paste!(r.[<read_ $t>]::<byteorder::LittleEndian>())
                }
            }
        )*
    };
}

impl_primitive! {
    u16, i16, u32, i32, u64, i64,
}

// floats are a bit special, the bits are casted to an integer and then treated as an integer

impl OctData for f32 {
    fn marshal_to<W: Write>(&self, w: &mut W, _: u16) -> Result<()> {
        w.write_u32::<byteorder::LittleEndian>(self.to_bits())
    }

    fn unmarshal_from<R: Read>(r: &mut R, _: u16) -> Result<Self> {
        Ok(Self::from_bits(r.read_u32::<byteorder::LittleEndian>()?))
    }
}

impl OctData for f64 {
    fn marshal_to<W: Write>(&self, w: &mut W, _: u16) -> Result<()> {
        w.write_u64::<byteorder::LittleEndian>(self.to_bits())
    }

    fn unmarshal_from<R: Read>(r: &mut R, _: u16) -> Result<Self> {
        Ok(Self::from_bits(r.read_u64::<byteorder::LittleEndian>()?))
    }
}

impl<T> OctData for Vec<T>
where
    T: OctData,
{
    fn marshal_to<W: Write>(&self, w: &mut W, bt_property_tag: u16) -> Result<()> {
        if self.is_empty() {
            (0i32).marshal_to(w, bt_property_tag)?;
            return Ok(());
        }
        (self.len() as i32).marshal_to(w, bt_property_tag)?;
        for item in self {
            item.marshal_to(w, bt_property_tag)?;
        }
        Ok(())
    }

    fn unmarshal_from<R: Read>(r: &mut R, bt_property_tag: u16) -> Result<Self> {
        let len = i32::unmarshal_from(r, bt_property_tag)?;
        if len < 0 {
            let len = -len;
            let mut vec = Self::with_capacity(len as usize);
            for _ in 0..len {
                bool::unmarshal_from(r, bt_property_tag)?;
                vec.push(T::unmarshal_from(r, bt_property_tag)?);
            }
            Ok(vec)
        } else {
            let mut vec = Self::with_capacity(len as usize);
            for _ in 0..len {
                vec.push(T::unmarshal_from(r, bt_property_tag)?);
            }
            Ok(vec)
        }
    }
}

impl<K, V> OctData for HashMap<K, V>
where
    K: OctData + Eq + Hash + Ord,
    V: OctData,
{
    fn marshal_to<W: Write>(&self, w: &mut W, bt_property_tag: u16) -> Result<()> {
        (self.len() as i32).marshal_to(w, bt_property_tag)?;
        for (key, value) in self.iter() {
            key.marshal_to(w, bt_property_tag)?;
            value.marshal_to(w, bt_property_tag)?;
        }
        Ok(())
    }

    fn unmarshal_from<R: Read>(r: &mut R, bt_property_tag: u16) -> Result<Self> {
        let len = i32::unmarshal_from(r, bt_property_tag)?;
        if len == -1 {
            return Ok(Self::new());
        }

        let mut map = Self::with_capacity(len as usize);
        for _ in 0..len {
            map.insert(
                K::unmarshal_from(r, bt_property_tag)?,
                V::unmarshal_from(r, bt_property_tag)?,
            );
        }
        Ok(map)
    }
}

impl<T> OctData for Option<T>
where
    T: OctData,
{
    fn marshal_to<W: Write>(&self, w: &mut W, bt_property_tag: u16) -> Result<()> {
        if let Some(item) = self {
            true.marshal_to(w, bt_property_tag)?;
            item.marshal_to(w, bt_property_tag)?;
        } else {
            false.marshal_to(w, bt_property_tag)?;
        }

        Ok(())
    }

    fn unmarshal_from<R: Read>(r: &mut R, bt_property_tag: u16) -> Result<Self> {
        if bool::unmarshal_from(r, bt_property_tag)? {
            Ok(Some(T::unmarshal_from(r, bt_property_tag)?))
        } else {
            Ok(None)
        }
    }
}

impl OctData for String {
    fn marshal_to<W: Write>(&self, w: &mut W, bt_property_tag: u16) -> Result<()> {
        if self.is_empty() {
            (-1i32).marshal_to(w, bt_property_tag)?;
            return Ok(());
        }
        (self.len() as i32).marshal_to(w, bt_property_tag)?;
        w.write_all(self.as_bytes())
    }

    fn unmarshal_from<R: Read>(r: &mut R, bt_property_tag: u16) -> Result<Self> {
        let len = i32::unmarshal_from(r, bt_property_tag)?;
        if len == -1 {
            return Ok(Self::new());
        }
        let mut buf = vec![0; len as usize];
        r.read_exact(&mut buf)?;
        Ok(Self::from_utf8(buf).unwrap())
    }
}

impl<T> OctData for HashSet<T>
where
    T: OctData + Eq + Hash,
{
    fn marshal_to<W: Write>(&self, w: &mut W, bt_property_tag: u16) -> Result<()> {
        (self.len() as i32).marshal_to(w, bt_property_tag)?;
        for item in self {
            item.marshal_to(w, bt_property_tag)?;
        }
        Ok(())
    }

    fn unmarshal_from<R: Read>(r: &mut R, bt_property_tag: u16) -> Result<Self> {
        let len = i32::unmarshal_from(r, bt_property_tag)?;
        if len == -1 {
            return Ok(Self::new());
        }
        let mut set = Self::with_capacity(len as usize);
        for _ in 0..len {
            set.insert(T::unmarshal_from(r, bt_property_tag)?);
        }
        Ok(set)
    }
}
