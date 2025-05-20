


// LD Load opererater
// LD r1 r2
// CASL2_LD(&mut a, &b);
pub fn casl2_ld<T: Copy>(a: &mut T, b: &T) {
    *a = *b;
}



// ST Store opererater
// ST r, adr [,x]
// CASL2_LD(&mut a, &b);
pub fn casl2_st<T: Copy>(a: &mut T, b: &T) {
    *a = *b;
}