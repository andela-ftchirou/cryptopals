pub fn mul(a: u8, b: u8) -> u8 {
    let mut p: u8 = 0;

    for i in 0..8 {
        if b & 0x01 != 0 {
            p ^= a;
        }

        b <<= 1;

        let carry: bool = a & 0x01 != 0;

        a >>= 1;
        
        if carry {
            a ^= 0x1b;
        }
    }

    p
}
