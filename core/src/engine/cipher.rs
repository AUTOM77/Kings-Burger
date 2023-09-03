use ring::{aead, error};

// const KEY: &[u8] = b"Q0U1Q0FGRjgzRDQ2NDcxMzkwOThFNjI1RjU1NjA4MDg=";
const NONCE: &[u8] = b"511520511520";

// #[warn(dead_code)]
// pub fn encode(raw: &str, key: &[u8]) -> String{
//     let mut cipher_vec = String::from(raw).as_bytes().to_vec();

//     let mut _o0_key: aead::SealingKey<OneNonceSequence> = make_key(
//         &aead::CHACHA20_POLY1305,
//         key,
//         aead::Nonce::try_assume_unique_for_key(&NONCE).unwrap(),
//     );

//     _o0_key.seal_in_place_append_tag(aead::Aad::empty(), &mut cipher_vec).unwrap();

//     let res:String = cipher_vec.iter()
//         .map(|c| format!("-{}", c))
//         .collect();
//     format!("0o://{}", &res[1..res.len()])
// }

pub fn decode(raw: &str, key: &[u8]) -> String{
    let mut cipher_vec: Vec<u8> = String::from(raw)
        .replace("0o://", "")
        .split("-")
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    let mut _o0_key: aead::OpeningKey<OneNonceSequence> = make_key(
        &aead::CHACHA20_POLY1305,
        key,
        aead::Nonce::try_assume_unique_for_key(&NONCE).unwrap(),
    );

    let decrypted = _o0_key.open_in_place(aead::Aad::empty(), &mut cipher_vec).unwrap();

    String::from_utf8(decrypted.to_vec()).unwrap()
}

fn make_key<K: aead::BoundKey<OneNonceSequence>>(
    algorithm: &'static aead::Algorithm,
    key: &[u8],
    nonce: aead::Nonce,
) -> K {
    let key = aead::UnboundKey::new(algorithm, key).unwrap();
    let nonce_sequence = OneNonceSequence::new(nonce);
    K::new(key, nonce_sequence)
}

struct OneNonceSequence(Option<aead::Nonce>);

impl OneNonceSequence {
    fn new(nonce: aead::Nonce) -> Self {
        Self(Some(nonce))
    }
}

impl aead::NonceSequence for OneNonceSequence {
    fn advance(&mut self) -> Result<aead::Nonce, error::Unspecified> {
        self.0.take().ok_or(error::Unspecified)
    }
}
