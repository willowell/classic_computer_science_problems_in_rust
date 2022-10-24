/// # Bit Vectors
///
use bitvec::prelude::*;

fn main() {
    let mut a = bitvec![0, 1];

    a.shrink_to_fit();

    let mut b = bitvec![u8, Lsb0;];

    b.push(true);
    b.push(false);

    let c = bits![0, 1];

    let d = BitVec::<u8>::from_slice(&[0, 1]);

    let e = BitVec::from_bitslice(c);

    let f = c.to_owned();

    const ADENINE: BitArr!(for 2, in u8, Lsb0) = bitarr![const u8, Lsb0; 0, 1];

    println!("{:?}", ADENINE);

    println!("{:?}", ADENINE.as_bitslice()[..2] == bits![0, 1]);

    println!("{:?}", a);

    println!("{:?}", b);

    println!("{:?}", c);

    println!("{:?}", d);

    println!("{:?}", e);

    println!("{:?}", f);
}
