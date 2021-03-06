# Specs and implementation details

## Overview of onion routing circuit message format

The protocol and specifications for constructing and consuming onion routing
packets are based on the [Sphinx message
format](https://www.freehaven.net/anonbib/cache/DBLP:conf/sp/DanezisG09.pdf) 
and the [Sphinx message format generalization for Onion
routing](http://cacr.uwaterloo.ca/techreports/2009/cacr2009-33.pdf). 

### Notations

- `k`: security parameter that bounds adversary attacks. `k` = 128 in this
  implementation;
- `v`: circuit length. Circuit construction is independent of `v`;
- `r`: max circuit length;
- `node_id`: node ID of `k` bits size;
- `μ`: message authentication code (MAC);
- `ρ`: pseudo-random generator (PRG);
- `Hμ`: MAC hash function;
- `Hρ`: PRG hash function;
- `Hb`: hash function for the blinding factor computation;
- `x_n`: public key of the relay node `n`;
- `s`: session key;
- `β`: packet encrypted by the PRG;

All hashes are modeled as random Oracles.

### Packet construction

The packet is constructed by the node who wishes to send a message through a
onion circuit with `v` onion nodes. The steps to construct a onion routing
package are:

1) Choose `v` onion relays and get their public keys `x_0`, ... `x_i`;

2) Compute random pseudonym `α_i`;

3) Computer session key `s_i`, where `s_i = α_i^x_i`;

4) wrap message in multiple layers of encryption usgin PRG `ρ` to generate
values `β_i`

5) calculate HMAC `γ_i` at each relay node;

The onion packet header is represent by the tuple `{α_i, β_i, γ_i`}.

### Packet consumption

Each onion relay nodes must first verify the packet received, derive the next
session key, decrypt the packet to extract the next packet and destination and
forward the packet:

1) Extract session key with its private key;

2) Extract the presudoanonym `α_i`;

3) Use `β_i` and `α_i` to verify the received HMAC `γ_i`;

4) Decrypt layer of encryption from `β_i`;

5) Compute pseudonym `α_i` for the next node relay; 

6) extract routing information and forward the packet and header to the next
onion relay.

## Implementation details for onion circuit packets

### Keys and key generation

The protocol uses three types of keys for encryption and verification. The keys
are derived from a shared secret.

The key generation takes a key type (`rho=0x72686F`, `mu=0x6d75` or `um=0x756d`)
and a 32-byte secret as inputs and returns a 32 byte key. Keys are generated
with HMAC-256 using the key type as key and shared secret as message input. The
returning HMAC is the respective key.

The key types are:

- *rho*=`0x72686F`: used as a key for generating the pseudo-random byte stream
  which is `XOR`ed to the `hops_data`. This key is generated using hop's shared 
secret;
- *mu*=`0x6d75`: used for HMCA generation;

## Header creation

*//todo*
