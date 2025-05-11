#![no_std]

extern crate alloc;

struct Luffa {
    chainv: [u32; 40],
}

const fn mult2(a: &mut [u32; 8]) {
    let tmp = a[7];
    a[7] = a[6];
    a[6] = a[5];
    a[5] = a[4];
    a[4] = a[3] ^ tmp;
    a[3] = a[2] ^ tmp;
    a[2] = a[1];
    a[1] = a[0] ^ tmp;
    a[0] = tmp;
}

const fn mix_word(yk: &mut u32, yk4: &mut u32) {
    *yk4 ^= *yk;
    *yk = (*yk).rotate_left(2);
    *yk ^= *yk4;
    *yk4 = (*yk4).rotate_left(14);
    *yk4 ^= *yk;
    *yk = (*yk).rotate_left(10);
    *yk ^= *yk4;
    *yk4 = (*yk4).rotate_left(1);
}

const fn sub_crunmb(a0: &mut u32, a1: &mut u32, a2: &mut u32, a3: &mut u32) {
    let mut tmp = *a0;
    *a0 |= *a1;
    *a2 ^= *a3;
    *a1 = !*a1;
    *a0 ^= *a3;
    *a3 &= tmp;
    *a1 ^= *a3;
    *a3 ^= *a2;
    *a2 &= *a0;
    *a0 = !*a0;
    *a2 ^= *a1;
    *a1 |= *a3;
    tmp ^= *a1;
    *a3 ^= *a2;
    *a2 &= *a1;
    *a1 ^= *a0;
    *a0 = tmp;
}

const fn add_constant(a: &mut [u32; 4], cns: &[u32; 2]) {
    a[0] ^= cns[0];
    a[1] ^= cns[1];
}

const fn gen_constant(c: &mut [u32; 2], cns: &mut [u32; 2]) {
    if c[0] >> 31 == 1 {
        c[0] = (c[0] << 1) ^ (c[1] >> 31) ^ 0xc4d6496c;
        c[1] = (c[1] << 1) ^ 0x55c61c8d;
    } else {
        c[0] = (c[0] << 1) ^ (c[1] >> 31);
        c[1] <<= 1;
    }
    cns[0] = c[0];
    c[0] = c[1];
    c[1] = cns[0];
    if c[0] >> 31 == 1 {
        c[0] = (c[0] << 1) ^ (c[1] >> 31) ^ 0xc4d6496c;
        c[1] = (c[1] << 1) ^ 0x55c61c8d;
    } else {
        c[0] = (c[0] << 1) ^ (c[1] >> 31);
        c[1] <<= 1;
    }
    cns[1] = c[0];
    c[0] = c[1];
    c[1] = cns[1];
}

impl Luffa {}
