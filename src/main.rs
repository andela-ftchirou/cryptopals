extern crate cryptopals;

use std::old_io;
use cryptopals::set_1::challenge_1::Base64;
use cryptopals::util::hexstring::HexString;

fn main() {
    let line = old_io::stdin()
        .read_line()
        .ok()
        .expect("failed to read hex string.");

    let hexstring = HexString::from_string(line);

    let base64 = Base64::from_bytes(&(hexstring.to_bytes()));

    println!("{}", base64.to_string());
    
}
