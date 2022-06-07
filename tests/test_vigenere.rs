use mirage::Vigenere;
use mirage::Cryptography;


#[test]
fn test_vigenere_encrypt_with_key_a() {
    assert_eq!(Vigenere::encrypt("Hello", "a"), "Hello");
}
#[test]
fn test_vigenere_encrypt_with_key_b() {
    assert_eq!(Vigenere::encrypt("Hello", "b"), "Ifmmp");
}
#[test]
fn test_vigenere_encrypt() {
    assert_eq!(Vigenere::encrypt("Hello world!", "foobar"), "Mszmo ntfze!");
}
#[test]
fn test_vigenere_encrypt_with_key_with_whitespace() {
    assert_eq!(Vigenere::encrypt("Hello world!", "foo bar"), "Mszmo ntfze!");
}
#[test]
fn test_vigenere_encrypt_decrypt_with_key_length_one() {
    let key = "k";
    let plaintext = "plaintext";
    let ciphertext = "zvksxdohd";

    assert_eq!(Vigenere::encrypt(plaintext, key), ciphertext);
    assert_eq!(Vigenere::decrypt(ciphertext, key), plaintext);

}
#[test]
fn test_vigenere_encrypt_decrypt() {
    let key = "key";
    let plaintext = "plaintext";
    let ciphertext = "zpysrrobr";

    assert_eq!(Vigenere::encrypt(plaintext, key), ciphertext);
    assert_eq!(Vigenere::decrypt(ciphertext, key), plaintext);
}
#[test]
fn test_vigenere_encrypt_decrypt_2() {
    let key = "anotherkey";
    let plaintext = "plaintext";
    let ciphertext = "pyobuxvhx";

    assert_eq!(Vigenere::encrypt(plaintext, key), ciphertext);
    assert_eq!(Vigenere::decrypt(ciphertext, key), plaintext);
}
#[test]
fn test_vigenere_encrypt_decrypt_big_word() {
    let key = "secret key";
    let plaintext = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    let ciphertext = "Dstvq bzwse hqcsk cmr sqgk, ghxwcuxgkyk khghmutmgq ijax, uvh wy igmwofh moqngv kegbnmbmrv lx ekfmji gk hhvspw qcxrt kpgiyc. Lx xxmk sh ozrbw zcfmcd, unsw lgwviyw obcjgkkemssl mpnrqvy pytstzw gswg mx ccmjemn wb gr ghwqmvs efrloussx. Flml kyrw mtlvx nsjgv ke vxzvczipuiksx gf zqcyiderw zgcmm owqw gkcpnw hmdstv in pyeaev eyeve nsvkrxnb. Ivuirkinb wgfx qtgtogyl gwgmwkxyl rqe tkymbwrv, jygd ml uynge jem mxjktmt niqwvwex fypjax cemf sh ckx nrfhbyk.";

    assert_eq!(Vigenere::encrypt(plaintext, key), ciphertext);
    assert_eq!(Vigenere::decrypt(ciphertext, key), plaintext);
}
#[test]
fn test_vigenere_encrypt_decrypt_with_special_characters() {
    let key = "key";
    let plaintext = "plaintext!@#$%^&*()_+=-~`plaintext";
    let ciphertext = "zpysrrobr!@#$%^&*()_+=-~`zpysrrobr";

    assert_eq!(Vigenere::encrypt(plaintext, key), ciphertext);
    assert_eq!(Vigenere::decrypt(ciphertext, key), plaintext);
}
#[test]
fn test_vigenere_encrypt_decrypt_with_non_character() {
    let key = "key";
    let plaintext = "plaintext 料";
    let ciphertext = "zpysrrobr 料";

    assert_eq!(Vigenere::encrypt(plaintext, key), ciphertext);
    assert_eq!(Vigenere::decrypt(ciphertext, key), plaintext);
}
#[test]
#[should_panic]
fn test_vigenere_encrypt_without_key() {
    let key = "";
    let plaintext = "plaintext";

    Vigenere::encrypt(plaintext, key);
}
#[test]
#[should_panic(expected = "Key must not be empty")]
fn test_vigenere_decrypt_without_key() {
    let key = "";
    let ciphertext = "plaintext";

    Vigenere::decrypt(ciphertext, key);
}
#[test]
#[should_panic(expected="Key must only contain alphabetic characters")]
fn test_vigenere_decrypt_with_invalid_key() {
    let key = "料";
    let ciphertext = "plaintext";

    Vigenere::decrypt(ciphertext, key);
}
