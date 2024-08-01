use bitvec::{
    bits,
    order::Msb0,
    prelude::{BitVec, Lsb0},
    view::{AsBits, BitView},
};

use crate::daletl::{Argument, Body, IsNull, Tag};

use super::utils::*;
use super::DaletPackError;

pub fn encode(root: Vec<Tag>) -> Result<Vec<u8>, DaletPackError> {
    if root.len() > 2usize.pow(32) {
        return Err(DaletPackError::RootMaxSizeExceeded);
    }

    let mut bv: BitVec<u8, Msb0> = BitVec::new();

    for tag in root {
        write_tag(&mut bv, tag)?;
    }

    bv.set_uninitialized(false);
    Ok(bv.into_vec())
}

fn write_int(bv: &mut BitVec<u8, Msb0>, n: u8) {
    if n < 16 {
        write_4bit(bv, 0);
        write_4bit(bv, n);
    } else {
        write_4bit(bv, 1);
        bv.extend_from_raw_slice(&[n]);
    }
}

fn write_str(bv: &mut BitVec<u8, Msb0>, string: String) -> Result<(), DaletPackError> {
    let size = string.len();

    if size > 2usize.pow(32) {
        return Err(DaletPackError::StrMaxSizeExceeded);
    }

    if size <= 8 {
        write_4bit(bv, 2);
        write_3bit(bv, (size - 1) as u8);
    } else if size <= 16 {
        write_4bit(bv, 3);
        write_4bit(bv, (size - 1) as u8);
    } else if size <= 256 {
        write_4bit(bv, 4);
        bv.extend_from_raw_slice(&[(size - 1) as u8]);
    } else if size <= 65536 {
        write_4bit(bv, 5);
        bv.extend_from_bitslice(&((size - 1) as u16).view_bits::<Msb0>());
    } else {
        write_4bit(bv, 6);
        bv.extend_from_bitslice(&((size - 1) as u32).view_bits::<Msb0>());
    }

    bv.extend_from_bitslice(&string.as_bits::<Msb0>());

    Ok(())
}

fn write_array(bv: &mut BitVec<u8, Msb0>, arr: Vec<Tag>) -> Result<(), DaletPackError> {
    if arr.len() > 2usize.pow(32) {
        return Err(DaletPackError::ArrMaxSizeExceeded);
    }

    write_4bit(bv, 7);

    for tag in arr {
        write_tag(bv, tag)?;
    }

    bv.extend_from_bitslice(&bits![1, 0]);

    Ok(())
}

fn write_tag(bv: &mut BitVec<u8, Msb0>, tag: Tag) -> Result<(), DaletPackError> {
    if tag.body.is_null() && tag.argument.is_null() {
        write_4bit(bv, 15);
        write_tag_id(bv, tag.id as u8);
    } else if tag.argument.is_null() {
        write_4bit(bv, 13);
        write_tag_id(bv, tag.id as u8);
        write_tag_body(bv, tag.body)?;
    } else if tag.body.is_null() {
        write_4bit(bv, 14);
        write_tag_id(bv, tag.id as u8);
        write_tag_argument(bv, tag.argument)?;
    } else {
        write_4bit(bv, 15);
        write_tag_id(bv, tag.id as u8);
        write_tag_body(bv, tag.body)?;
        write_tag_argument(bv, tag.argument)?;
    }

    Ok(())
}

fn write_tag_id(bv: &mut BitVec<u8, Msb0>, n: u8) {
    bv.extend_from_bitslice(&n.view_bits::<Msb0>()[3..=7]);
}

fn write_tag_body(bv: &mut BitVec<u8, Msb0>, body: Body) -> Result<(), DaletPackError> {
    match body {
        Body::Text(s) => write_str(bv, s)?,
        Body::Tags(tags) => write_array(bv, tags)?,
        Body::Null => unreachable!("This function cannot be called with this value"),
    };

    Ok(())
}

fn write_tag_argument(bv: &mut BitVec<u8, Msb0>, argument: Argument) -> Result<(), DaletPackError> {
    match argument {
        Argument::Text(s) => write_str(bv, s)?,
        Argument::Number(n) => write_int(bv, n),
        Argument::Null => unreachable!("This function cannot be called with this value"),
    };

    Ok(())
}
