use ec_encrypt_scheme::{decrypt, encrypt};
use generic_ec::{
    curves::{Ed25519, Secp256k1},
    Curve, Point, Scalar,
};

fn sk_from_u64<E: Curve>(value: u64) -> Scalar<E> {
    Scalar::<E>::from_be_bytes_mod_order(value.to_be_bytes())
}

fn hex_decode_or_panic(input: &str) -> Vec<u8> {
    match hex::decode(input) {
        Ok(v) => v,
        Err(e) => panic!("invalid test hex: {e}"),
    }
}

#[test]
fn roundtrip_hello_world() {
    let sk = Scalar::<Ed25519>::random(&mut rand::thread_rng());
    let pk = Point::<Ed25519>::generator() * sk;

    let msg = b"hello world";
    let ct = encrypt(pk, msg);
    let pt = decrypt(sk, &ct);

    assert_eq!(pt, msg);
}

#[test]
fn roundtrip_empty_message() {
    let sk = Scalar::<Secp256k1>::random(&mut rand::thread_rng());
    let pk = Point::<Secp256k1>::generator() * sk;

    let msg = b"";
    let ct = encrypt(pk, msg);
    let pt = decrypt(sk, &ct);

    assert_eq!(pt, msg);
}

#[test]
fn roundtrip_large_message() {
    let sk = Scalar::<Ed25519>::random(&mut rand::thread_rng());
    let pk = Point::<Ed25519>::generator() * sk;

    let msg = vec![0xAB; 1000];
    let ct = encrypt(pk, &msg);
    let pt = decrypt(sk, &ct);

    assert_eq!(pt, msg);
}

#[test]
fn test_vectors_private_key_65537_basic() {
    let sk_ed = sk_from_u64::<Ed25519>(65537);

    let ct_ed = hex_decode_or_panic(
        "83789da3b47511d971be426996e29773dbf1fd0b5d4117dc3f6197ac3b390b16021c4d4dcacd69fa6ddfbd70272254a8c1d6caa1553718b4b592f518ca856030",
    );

    let msg_ed =
        hex_decode_or_panic("0000000000000000000000000000000000000000000000000000000000000000");

    assert_eq!(decrypt(sk_ed, &ct_ed), msg_ed);

    let sk_secp = sk_from_u64::<Secp256k1>(65537);

    let ct_secp = hex_decode_or_panic(
        "028ff73c6a81376adeb0a5b9d3e0a89de67ef1215174c1b53a953bc51a5849ad4940c21b932a166cb2b913778a30f500b4f1c09d48c2549560c9f5513a6cf395f1",
    );

    let msg_secp =
        hex_decode_or_panic("0000000000000000000000000000000000000000000000000000000000000000");

    assert_eq!(decrypt(sk_secp, &ct_secp), msg_secp);
}

#[test]
fn test_vector_ed25519_large_ff() {
    let sk = sk_from_u64::<Ed25519>(65537);

    let ct = hex_decode_or_panic(
        "63dddd19ca1aae622af6419925c1ccb6aa009255f08fc8f36ebc96aeffb0e575cc8408cbb3762fb4bbfdfb36f62cbc4e9dfaaab0882d62acc16f7d77e366af64cc8408cbb3762fb4bbfdfb36f62cbc4e9dfaaab0882d62acc16f7d77e366af64cc8408cbb3762fb4bbfdfb36f62cbc4e9dfaaab0882d62acc16f7d77e366af64cc8408cbb3762fb4bbfdfb36f62cbc4e9dfaaab0882d62acc16f7d77e366af64",
    );

    let msg = hex_decode_or_panic(
        "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
    );

    assert_eq!(decrypt(sk, &ct), msg);
}

#[test]
fn test_vector_secp256k1_large_ff() {
    let sk = sk_from_u64::<Secp256k1>(65537);

    let ct = hex_decode_or_panic(
        "022361daf6095c336b21f3ae6a9cb3a4389071e65f3dddc910783fd2805f80d0660ca42649522059373a5677b2391fe1c2dd718724bb984bb0b926e32c26123bf60ca42649522059373a5677b2391fe1c2dd718724bb984bb0b926e32c26123bf60ca42649522059373a5677b2391fe1c2dd718724bb984bb0b926e32c26123bf60ca42649522059373a5677b2391fe1c2dd718724bb984bb0b926e32c26123bf6",
    );

    let msg = hex_decode_or_panic(
        "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
    );

    assert_eq!(decrypt(sk, &ct), msg);
}

#[test]
fn invalid_ciphertext_does_not_panic() {
    let sk = Scalar::<Ed25519>::random(&mut rand::thread_rng());

    let out = decrypt(sk, &[1, 2, 3]);

    assert!(out.is_empty());
}