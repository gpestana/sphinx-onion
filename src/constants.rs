// constants.rs

// Maximum number of hops a packet can traverse
pub const MAX_HOPS: usize = 4;

// Node ID size of 32 bytes
pub const NODE_ID_SIZE: usize = 32;

pub const HEADER_SIZE: usize = 10; // TODO: change
pub const PAYLOAD_SIZE: usize = 11; // TODO: change
pub const PACKET_SIZE: usize = HEADER_SIZE + PAYLOAD_SIZE;
