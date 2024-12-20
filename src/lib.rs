mod utils;

use smcrypto::sm4;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-demo!");
}

pub const KEY: &str = "1234567812345678";
pub const IV: &str= "0000000000000000";


#[wasm_bindgen]
pub fn encrypt_to_base64(origin_str: &str) -> String {
    let sm4_cbc = sm4::CryptSM4CBC::new(KEY.as_bytes(), IV.as_bytes());
    let enc_cbc = sm4_cbc.encrypt_cbc_base64(origin_str.as_bytes());
    enc_cbc
}

#[wasm_bindgen]
pub fn decrypt_from_base64(encoded_str: &str) -> String {
    // 创建SM4 CBC解密器
    let sm4_cbc = sm4::CryptSM4CBC::new(KEY.as_bytes(), IV.as_bytes());

    // 进行解密
    let dec_cbc = sm4_cbc.decrypt_cbc_base64(&encoded_str);

    // 将解密后的字节数组转换为字符串
    match String::from_utf8(dec_cbc) {
        Ok(decoded_str) => decoded_str,
        Err(_) => "Failed to decode UTF-8 string".to_string(),
    }
}