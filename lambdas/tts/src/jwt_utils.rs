use hmac::{Hmac, Mac};
use jwt::{VerifyWithKey, SignWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

fn verify_and_decode(token: String, key: String) -> BTreeMap<String, String> {
    let hmac_key: Hmac<Sha256> = Hmac::new_from_slice(key.as_bytes()).unwrap();
    //let token_str = "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJzb21lb25lIn0.5wwE1sBrs-vftww_BGIuTVDeHtc1Jsjo-fiHhDwR8m0";
    return token.verify_with_key(&hmac_key).unwrap();
}

fn sign(claims: BTreeMap<String, String>, key: String) -> String {
    let hmac_key: Hmac<Sha256> = Hmac::new_from_slice(key.as_bytes()).unwrap();
    //let mut claims = BTreeMap::new();
    //claims.insert("sub", "someone");

    return claims.sign_with_key(&hmac_key).unwrap();
}
