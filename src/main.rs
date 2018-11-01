mod crypto;
extern crate hmac;
extern crate sha2;

use crypto::keys::Keys;
use crypto::random::generate_pr;

fn main() {
    let s = String::from("secret");
    let ks = Keys::new(s);
    println!("{:?}\n{:?}", ks.rho, ks.mu);

    generate_pr();
}

