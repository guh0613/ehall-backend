use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use base64::Engine;
use base64::{decode, encode};
use rand::{thread_rng, Rng};

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

pub fn aes_cbc_encrypt_url(data: &[u8], key: &[u8; 16]) -> Vec<u8> {
    let mut rng = thread_rng();
    let iv: [u8; 16] = rng.gen();
    let ct_bytes = Aes128CbcEnc::new(key.into(), &iv.into()).encrypt_padded_vec_mut::<Pkcs7>(data);
    encode(ct_bytes).into()
}

#[cfg(test)]
mod test {
    use super::aes_cbc_encrypt_url;

    #[test]
    fn aes_cbc_encrypt_url_test() {
        let key = [0x42; 16];
        let plaintext = *b"hello world! this is my plaintext.";
        println!("{:?}", aes_cbc_encrypt_url(&plaintext, &key));
    }
}
