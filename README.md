# DKLs23 Learning Journey

## About Me
I am a developer exploring threshold cryptography and MPC systems.

## What I’m Learning
- Threshold Signature Schemes (TSS)
- t-of-n security
- Basics of ECDSA
- Multi-Party Computation (MPC)

## DKLs23 (Initial Understanding)
DKLs23 is a threshold ECDSA protocol that allows multiple parties to jointly generate a signature without revealing their private key shares.

This improves security because no single party controls the full private key.

## Plan
- [ ] Read DKLs23 paper
- [ ] Implement Shamir Secret Sharing
- [ ] Explore Rust for MPC frameworks


## Implementation

### Shamir Secret Sharing
Implemented a basic t-of-n secret sharing scheme.

- Secret is split into multiple shares
- Minimum threshold required to reconstruct
- Used polynomial interpolation (Lagrange)
