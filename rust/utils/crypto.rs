use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use base64::Engine;
use base64::{decode, encode};
use rand::{thread_rng, Rng};

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
// type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

pub fn aes_cbc_encrypt_url(data: &[u8], key: &[u8]) -> String {
    let mut rng = thread_rng();
    let iv: [u8; 16] = rng.gen();
    let iv = random_vec(16);
    let iv = &iv[..16];
    let ct_bytes = Aes128CbcEnc::new(key.into(), iv.into()).encrypt_padded_vec_mut::<Pkcs7>(data);
    encode(ct_bytes)
}

pub fn random_vec_alt(len: usize) -> Vec<u8> {
    let mut rng = thread_rng();
    (0..len).map(|_| rng.gen::<u8>()).collect()
}

pub fn random_vec(n: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let charset: Vec<u8> = "ABCDEFGHJKMNPQRSTWXYZabcdefhijkmnprstwxyz2345678"
        .bytes()
        .collect();

    let random_chars: Vec<u8> = (0..n)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx]
        })
        .collect();

    random_chars
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aes_cbc_encrypt_url_test() {
        let data = b"hello";
        let key = [48; 16];
        let iv = [48; 16];
        eprintln!("data: {:?}", data);
        eprintln!("key: {:?}", key);
        eprintln!("iv: {:?}", iv);
        let ct_bytes = Aes128CbcEnc::new(&key.into(), &iv.into()).encrypt_padded_vec_mut::<Pkcs7>(data);
        // eprintln!("ct_bytes: {:#?}", ct_bytes);
        eprintln!("aes_cbc_encrypt_url_test: {}", encode(ct_bytes));
    }
}
