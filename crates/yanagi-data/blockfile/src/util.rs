use byteorder::{ByteOrder, LE};

use crate::magic::*;
use std::io::{Read, Result};

pub trait MhyObscureReadExt {
    fn read_obscure_u32(&mut self) -> Result<u32>;
    fn read_obscure_i32(&mut self) -> Result<i32>;
    fn read_path_string(&mut self) -> Result<String>;
    fn read_bool(&mut self) -> Result<bool>;
}

impl<R: Read> MhyObscureReadExt for R {
    fn read_obscure_u32(&mut self) -> Result<u32> {
        let mut buf = [0u8; 7];
        self.read_exact(&mut buf)?;

        Ok((buf[1] as u32)
            | ((buf[6] as u32) << 8)
            | ((buf[3] as u32) << 16)
            | ((buf[2] as u32) << 24))
    }

    fn read_obscure_i32(&mut self) -> Result<i32> {
        let mut buf = [0u8; 6];
        self.read_exact(&mut buf)?;

        Ok((buf[2] as i32)
            | ((buf[4] as i32) << 8)
            | ((buf[0] as i32) << 16)
            | ((buf[5] as i32) << 24))
    }

    fn read_path_string(&mut self) -> Result<String> {
        const WIN32_MAX_PATH: usize = 260;

        let mut buf = [0u8; WIN32_MAX_PATH + 1];
        self.read_exact(&mut buf)?;

        let str_len = buf.iter().position(|&c| c == 0).unwrap_or(WIN32_MAX_PATH);
        Ok(String::from_utf8(buf[..str_len].to_vec()).unwrap())
    }

    fn read_bool(&mut self) -> Result<bool> {
        use byteorder::ReadBytesExt;
        Ok(self.read_u8()? != 0)
    }
}

type Aes128Encryptor = ecb::Encryptor<aes::Aes128>;

// UnityPlayer: 0x1A770
#[allow(non_snake_case)]
pub fn sub_18001A770(a1: &mut Vec<u8>, len: usize, a4: usize) {
    use aes::cipher::{block_padding::NoPadding, BlockEncryptMut, KeyInit};

    let v5 = a4 + 15;
    shuffle128(&mut a1[4..20]);
    let v20 = v5 & 0xFFFFFFF0;
    assert_eq!(LE::read_u128(&a1[4..20]), XMMWORD_181D77E00);
    if len >= 0x24 {
        let v70 = v20;
        shuffle128(&mut a1[20..36]);

        let en = Aes128Encryptor::new(a1[0..16].into());
        en.encrypt_padded_mut::<NoPadding>(&mut a1[20..36], 16)
            .unwrap();

        (0..4).for_each(|i| a1[i] ^= a1[20 + i]);

        let v58 = a1[20..28].to_vec();
        let v73 = a1[28..36].to_vec();

        sub_18001A5E0(&mut a1[20 + v70..len], len - 20 - v70, &v58, 8, &v73);
    }
}

// UnityPlayer: 0x1A5E0
#[allow(non_snake_case)]
fn sub_18001A5E0(a1: &mut [u8], a2: usize, a3: &[u8], a4: usize, a5: &[u8]) {
    let mut v21 = UNK_181D77E50.to_vec();
    let mut v11 = 0u8;
    for v9 in 0..256 {
        let v10 = v21[v9];
        v11 = v11.wrapping_add(a3[v9 % a4].wrapping_add(v10));
        v21[v9] = v21[v11 as usize];
        v21[v11 as usize] = v10;
    }

    let mut v13 = 0u8;
    let mut v14 = 0u8;
    for v12 in 0..a2 {
        let v15 = v14.wrapping_add(1);
        v14 = v14.wrapping_add(1);
        let v16 = v21[v14 as usize];
        v13 = v13.wrapping_add(v16);
        v21[v14 as usize] = v21[v13 as usize];
        v21[v13 as usize] = v16;
        let v17 = v16.wrapping_add(v21[v14 as usize]);
        let v18 = a5[v15 as usize & 7] % 3;
        match v18 {
            2 => {
                a1[v12] = a1[v12].wrapping_add(v21[v17 as usize]);
            }
            1 => {
                a1[v12] = a1[v12].wrapping_sub(v21[v17 as usize]);
            }
            0 => {
                a1[v12] ^= v21[v17 as usize];
            }
            _ => unreachable!(),
        }
    }
}

// inlined 3 times in sub_18001A770
// 'decrypts' 16-byte chunks of data
fn shuffle128(a1: &mut [u8]) {
    assert_eq!(a1.len(), 16);
    for v6 in 0..3 {
        let v8 = 32 - 16 * v6;

        let mut temp = [0u8; 16];
        for i in 0..16 {
            temp[i] = a1[SHUFFLE_TABLE[v8 + i] as usize];
        }
        a1.copy_from_slice(&temp);

        let mut v16 = 0;
        for v17 in 0..16 {
            let v9 = a1[v17];
            let v18 = v17 & 7;
            let v19 = if v9 != 0 {
                BYTE_181D7BF80[(((BYTE_181D7C080[v9 as usize] as u16)
                    + (BYTE_181D7C080[BYTE_181D7C5B8[v18] as usize] as u16))
                    % 0xFF) as usize] as usize
                    | (v16 & 0x300)
            } else {
                v16 & 0x300
            };

            a1[v17] = BYTE_181D7C180[v19 as usize] ^ BYTE_181D7C5B0[v18 as usize];
            v16 += 256;
        }
    }
}
