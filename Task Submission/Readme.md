# EC Encryption Scheme (Lockness Mentorship Challenge)

## Overview

This task assigns to make an elliptic-curve-based encryption scheme using the `generic-ec` crate.

The scheme follows a Diffie-Hellman style approach:

* Generate shared secret using elliptic curve multiplication
* Derive symmetric key using SHA-256
* Encrypt message using XOR with expanded key

---

## Algorithm

### Encryption

Given public key `pk` and message `M`:

1. Generate random scalar `eph`
2. Compute:

   * `R = G * eph`
   * `S = pk * eph`
3. Derive key:

   * `K = SHA256(encode(S))`
4. Expand key to match message length
5. Compute:

   * `C = M XOR K`
6. Output:

   * `encode(R) || C`

---

### Decryption

Given private key `sk` and ciphertext:

1. Split ciphertext into `R || C`
2. Compute:

   * `S = R * sk`
3. Derive key:

   * `K = SHA256(encode(S))`
4. Expand key
5. Recover message:

   * `M = C XOR K`

---

## Features

* Supports multiple curves:

  * Ed25519
  * Secp256k1
* No panicking operations
* Fully tested with:

  * roundtrip tests
  * edge cases
  * official test vectors

---

## Running Tests

```bash
cargo test
```

---

## Code Quality

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
```

---