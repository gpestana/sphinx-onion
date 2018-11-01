# ronion specs

## Keys and Key Generation

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
- *um*=`0x756d`: used for error reporting. `//needed?`

