/// An AES128 CBC encryption type.
///
/// This is a specialised alias for a [`Encryptor`] from the `cbc` library.
///
/// [`Encryptor`]: cbc::Encryptor
pub type Aes128CbcEnc = Encryptor<Aes128>;

/// An AES128 CBC decryption type.
///
/// This is a specialised alias for a [`Decryptor`] from the `cbc` library.
///
/// [`Decryptor`]: cbc::Decryptor
pub type Aes128CbcDec = Decryptor<Aes128>;

/// Generate a 16-bit key.
pub fn GenKey16() -> String {
   let mut arr = [0u8; 16];
   thread_rng().try_fill(&mut arr[..]).expect("could not fill entropy array");
   return hex::encode(arr);
}

/// Generate a 32-bit hex string, useful as a secret key et cetera.
pub fn GenKey32() -> String {
   let mut arr = [0u8; 32];
   thread_rng().try_fill(&mut arr[..]).expect("could not fill entropy array");
   return hex::encode(arr);
}

/// Generate a 64-bit key from plain text.
pub fn GenKey64(pt: String) -> String {
   let mut buf = [0u8; 64];
   let key = [0x42; 64];
   let iv = [0x24; 64];

   thread_rng().try_fill(&mut buf).expect("could not fill from entropy");

   let pt_len = pt.len();
   let enc = Aes128CbcEnc::new_from_slices(&key, &iv)
      .unwrap()
      .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len)
      .unwrap();

   return hex::encode(enc);
}

/// Decrypt a 64-bit key using its cipher.
pub fn DecryptKey64(ct: String) -> String {
   let mut string = hex::decode(ct).unwrap();

   let key = [0x42; 64];
   let iv = [0x24; 64];
   let dec = Aes128CbcDec::new_from_slices(&key, &iv)
      .unwrap()
      .decrypt_padded_mut::<Pkcs7>(&mut string)
      .unwrap();

   return String::from(str::from_utf8(&dec).unwrap());
}

use {
   aes::{
      cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit},
      Aes128,
   },
   cbc::{Decryptor, Encryptor},
   hex,
   rand::{thread_rng, Rng},
   std::str,
};
