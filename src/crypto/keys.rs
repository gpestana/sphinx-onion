extern crate crypto;
extern crate hmac;
extern crate sha2;

use sha2::Sha256;
use hmac::{Hmac, Mac};

type HmacSha256 = Hmac<Sha256>;

const RHO: [u8; 3] = [0x72, 0x68, 0x6F];
const MU: [u8; 2] = [0x6d, 0x75];
const UM: [u8; 2] = [0x75, 0x6d];

#[derive(Debug)]
pub struct Keys {
    pub rho: Vec<u8>,
    pub mu: Vec<u8>,
    pub um: Vec<u8>,
}

impl Keys {
    pub fn new(s: String) -> Keys {
        Keys {
            rho: hmac(&RHO, &s),
            mu: hmac(&MU, &s),
            um: hmac(&UM, &s),
        }
    }
}

fn hmac(k: &[u8], s: &String) -> Vec<u8> {
    let mut mac = HmacSha256::new_varkey(k).expect("ok");
    mac.input(s.as_bytes());
    mac.result().code().as_slice().to_vec()
}
