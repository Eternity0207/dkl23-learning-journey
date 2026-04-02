# DKLs23 — Threshold ECDSA in Three Rounds (My Understanding)

## 1. Problem

In traditional ECDSA, a single private key `sk` is used to sign messages:

s = k⁻¹ · (H(m) + r · sk) mod q

This creates a **single point of failure**:
- If `sk` is leaked → system is fully compromised

---

## 2. Solution: Threshold ECDSA

DKLs23 solves this by:
- Splitting the private key across multiple parties
- Requiring at least `t` out of `n` parties to sign

This removes the single point of failure.

---

## 3. Secret Sharing (Mathematical Foundation)

The private key is not stored directly.

Instead:
- A polynomial `f(x)` of degree `t-1` is created
- Secret is the constant term:

f(x) = sk + a₁x + a₂x² + ... + aₜ₋₁xᵗ⁻¹

Each party gets:
skᵢ = f(i)

### Key Properties:
- Any `t` shares → reconstruct `sk`
- Any `< t` shares → reveal nothing (information-theoretic security)

This is **Shamir Secret Sharing**, which I implemented separately.

---

## 4. Why Threshold ECDSA is Hard

The ECDSA equation:

s = k⁻¹ · (H(m) + r · sk)

Problem:
- `sk` and `k` are multiplied
- Both are secret and distributed across parties

👉 We need to compute:
sk · k  
WITHOUT revealing either

This is the core challenge.

---

## 5. Key Insight of DKLs23

Instead of computing `s` directly, the equation is rewritten.

The protocol computes:

- R = k · G  
- r = x-coordinate of R  
- u = r · φ  
- w = (H(m) + sk · r) · φ  

Final signature:

s = w / u

---

## 6. Why This Helps

Now the problem reduces to:
- Computing secure multiplications of secret-shared values

This is done using **VOLE (Vector Oblivious Linear Evaluation)**.

---

## 7. What is VOLE (Intuition)

VOLE allows two parties to compute:

a · b + c

Without revealing:
- `a` to the other party
- `b` to the other party

👉 It’s a secure multiplication primitive  
👉 Much faster than older methods (Paillier encryption)

---

## 8. Protocol Flow (3 Rounds)

### Round 1 — Commit
- Each party generates a nonce share `kᵢ`
- Commits to it (cannot change later)

---

### Round 2 — Compute
- Parties run VOLE to compute:
  - shares of `u`
  - shares of `w`
- Each party contributes to:
  R = k · G

---

### Round 3 — Combine
- Parties reveal shares of `u` and `w`
- Compute:

s = w / u

Final signature:
(r, s)

---

## 9. Important Concepts

### R and r

- k → secret nonce (never revealed)
- G → generator point
- R = k · G → elliptic curve point (public)
- r = x-coordinate of R

---

### Why R is safe

Even though R is public:
- Recovering k from R is hard (discrete log problem)

---

## 10. Security Guarantees

- **No single party knows full private key**
- **Malicious security** (parties can cheat but get caught)
- **Dishonest majority allowed**
- **UC security** (composable security model)

---

## 11. Why DKLs23 is Better

| Property | Older (GG18/GG20) | DKLs23 |
|--------|------------------|--------|
| Rounds | 6+               | 3      |
| Crypto | Paillier + ZK    | OT / VOLE |
| Speed  | Slow             | Fast   |

👉 Fewer rounds = lower latency

---

## 12. Connection to My Work

I implemented **Shamir Secret Sharing**, where:
- A secret is encoded as a polynomial
- Shares are points on that polynomial
- Reconstruction uses Lagrange interpolation

This directly helped me understand:
- How secrets are distributed
- Why t-of-n works
- How threshold cryptography is built

---

## 13. Key Takeaway

DKLs23 shows that:

- We can compute valid ECDSA signatures
- Without ever reconstructing the private key
- Using efficient primitives (VOLE)
- In just 3 rounds

---

## 14. One-Line Summary

DKLs23 is a protocol that enables multiple parties to collaboratively generate a valid ECDSA signature without revealing or reconstructing the private key, using efficient secure multiplications in just three communication rounds.