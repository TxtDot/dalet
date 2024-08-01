use bitvec::{order::Msb0, prelude::BitVec, view::BitView};

pub fn write_3bit(bv: &mut BitVec<u8, Msb0>, n: u8) {
    bv.extend_from_bitslice(&n.view_bits::<Msb0>()[5..=7]);
}

pub fn write_4bit(bv: &mut BitVec<u8, Msb0>, n: u8) {
    bv.extend_from_bitslice(&n.view_bits::<Msb0>()[4..=7]);
}
