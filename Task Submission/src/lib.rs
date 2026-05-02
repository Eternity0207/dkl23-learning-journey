use generic_ec::{Curve, Point, Scalar};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};

fn expand(seed: &[u8], len: usize) -> Vec<u8> {
    if len == 0 {
        return Vec::new();
    }
    seed.iter().copied().cycle().take(len).collect()
}

fn xor(left: &[u8], right: &[u8]) -> Vec<u8> {
    left.iter().zip(right).map(|(a, b)| a ^ b).collect()
}

fn shared_key<E: Curve>(point: &Point<E>) -> [u8; 32] {
    let mut out = [0_u8; 32];
    let digest = Sha256::digest(point.to_bytes(true));
    out.copy_from_slice(&digest);
    out
}

pub fn encrypt<E: Curve>(pk: Point<E>, message: &[u8]) -> Vec<u8> {
    let eph = Scalar::<E>::random(&mut OsRng);
    let r = Point::<E>::generator() * eph;
    let s = pk * eph;
    let key = shared_key(&s);
    let mask = expand(&key, message.len());
    let mut out = r.to_bytes(true).as_ref().to_vec();
    out.extend(xor(message, &mask));
    out
}

pub fn decrypt<E: Curve>(sk: Scalar<E>, ciphertext: &[u8]) -> Vec<u8> {
    let r_len = Point::<E>::serialized_len(true);
    if ciphertext.len() < r_len {
        return Vec::new();
    }
    let (r_bytes, c) = ciphertext.split_at(r_len);
    let r = match Point::<E>::from_bytes(r_bytes) {
        Ok(p) => p,
        Err(_) => return Vec::new(),
    };
    let s = r * sk;
    let key = shared_key(&s);
    let mask = expand(&key, c.len());
    xor(c, &mask)
}
