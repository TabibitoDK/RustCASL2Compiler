mod instructionSet;

#[derive(Copy, Clone, Debug)]
struct Register {
    data: i16,
}





fn main() {
    let mut gr: [Register; 8] = [Register { data: 0 }; 8];
    let mut mem: [i16; 65536] = [0; 65536];

    let mut a = Register { data: 11 };
    let b = Register { data: 42 };
    instructionSet::casl2_ld(&mut a, &b);
    println!("a = {}", a.data); // Output: a = 42
}





