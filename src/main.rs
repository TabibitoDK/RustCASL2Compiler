
#[derive(Copy, Clone, Debug)]
struct Register {
    data: i16,
}





fn main() {
    let mut GR: [Register; 8] = [Register { data: 0 }; 8];
    let mut MEM: [i16; 65536] = [0; 8];

    let mut a = 10;
    let b = 42;
    CASL2_LD(&mut a, &b);
    println!("a = {}", a); // Output: a = 42
}



// LD Load opererater
// LD r1 r2
// CASL2_LD(&mut a, &b);
fn CASL2_LD<T: Copy>(a: &mut T, b: &T) {
    *a = *b;
}



// ST Store opererater
// ST r, adr [,x]
// CASL2_LD(&mut a, &b);
fn CASL2_ST<T: Copy>(a: &mut T, b: &T) {
    *a = *b;
}


