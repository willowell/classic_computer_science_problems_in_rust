/// # Bit Vectors
/// 

use bitvec::prelude::*;

fn main() {
    let mut b = bitvec![u8, Lsb0;];

    let c = bits![0, 1];

    b.push(true);
    b.push(false);

    println!("{:?}", b);

    println!("{:?}", c);
}
