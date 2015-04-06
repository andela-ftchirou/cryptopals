pub fn mul(x: u8, y: u8) -> u8 {
    let mut a: u16 = x as u16;
    let mut b: u16 = y as u16;
    let mut p: u16 = 0;

    for i in 0..8 {
        if b & 0x01 != 0 {
            p ^= a;
        }

        let carry: bool = a & 0x80 != 0;

        a <<= 1;
        
        if carry {
            a ^= 0x1b;
        }

        b >>= 1;
    }

    p as u8
}

pub fn xtimes(a: u8) -> u8 {
    let mut p: u16 = a as u16;
    p <<= 1;
    
    if p & (1 << 8) != 0 {
        p ^= 0x11b;
    }

    p as u8
}
