// client.rs: public interface to create sphinx header and packet
use super::constants::{HEADER_SIZE, NODE_ID_SIZE, PACKET_SIZE};
use super::error::{HeaderCreateError, PacketCreateError};
use super::structs::{Header, Packet};

// dummy enum while PublicKey is not figured out
pub enum PublicKey {}

// HopInfo contains all information about a hop
pub struct HopInfo {
    pub id: [u8; NODE_ID_SIZE],
    pub public_key: PublicKey,
}

// creates a new sphinx-onion header
//
// Arguments:
// - path: vector of hops which contains all information about the path. Last vector hop is
// the destination
//
// Outputs:
// - header or error
//
pub fn create_header(path: Vec<HopInfo>) -> Result<([u8; HEADER_SIZE], Header), HeaderCreateError> {
    // 1) derive the keys for each hop
    // 2)
    return Err(HeaderCreateError::ImpossibleError);
}

// creates a new sphinx-onion packet
//
// Arguments:
//
// Outputs:
//  - packet or error
//
pub fn create_packet() -> Result<([u8; PACKET_SIZE], Packet), PacketCreateError> {
    return Err(PacketCreateError::ImpossibleError);
}
