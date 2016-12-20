extern crate bit_vec;
use bit_vec::BitVec;

fn main() {
    let mut args = std::env::args();
    args.next();
    let disk_size = args.next().expect("Need disk size parameter").parse::<usize>().unwrap();
    let input = args.next().expect("Need initial state parameter").to_string();

    let mut disk = BitVec::from_fn(input.len(), |i| {
        if input.as_bytes()[i] == b'1' { true } else { false }
    });

    while disk.len() < disk_size {
        dragon_curve_bitvec(&mut disk);
    }
    disk.truncate(disk_size);

    let cs = checksum(disk);

    println!("Checksum: {:?}", cs);
}

fn dragon_curve_bitvec(bv: &mut BitVec) {
    let len = bv.len();
    bv.reserve_exact(len + 1);
    bv.push(false);
    for i in (0..len).rev() {
        let bit = bv[i];
        bv.push(!bit)
    }
}

fn checksum(bv: mut BitVec) {
    for i in 0..bv.len() / 2 {
        let bit0 = bv[i*2];
        let bit1 = bv[i*2 + 1];
        let checksum_bit = bit0 == bit1;
        bv[i] = checksum_bit;
    };
    bv.truncate(bv.len() / 2);
    if result.len() & 1 == 0 {
        checksum(bv)
    } else {
        bv
    }
}