use mirage::Caesar;
use mirage::Cryptography;

// Test the encrypt caesar function
#[test]
fn test_encrypt_caesar_with_shift_zero() {
    assert_eq!(Caesar::encrypt("Hello", 0), "Hello");
}
#[test]
fn test_encrypt_caesar_with_shift_one() {
    assert_eq!(Caesar::encrypt("Hello", 1), "Ifmmp");
}
#[test]
fn test_encrypt_caesar_with_shift_twenty_six() {
    assert_eq!(Caesar::encrypt("Hello", 26), "Hello");
}
#[test]
fn test_encrypt_caesar_with_shift_twenty_seven() {
    assert_eq!(Caesar::encrypt("Hello", 27), "Ifmmp");
}
#[test]
fn test_encrypt_caesar_with_shift_negative_one() {
    assert_eq!(Caesar::encrypt("Hello", -1), "Gdkkn");
}
#[test]
fn test_encrypt_caesar_with_shift_negative_twenty_seven() {
    assert_eq!(Caesar::encrypt("Hello", -27), "Gdkkn");
}
#[test]
fn test_encrypt_caesar_with_non_character() {
    assert_eq!(Caesar::encrypt("Hello!", 1), "Ifmmp!");
}


// Test the decrypt caesar function
#[test]
fn test_decrypt_caesar_with_shift_zero() {
    assert_eq!(Caesar::decrypt("Hello", 0), "Hello");
}
#[test]
fn test_decrypt_caesar_with_shift_one() {
    assert_eq!(Caesar::decrypt("Ifmmp", 1), "Hello");
}
#[test]
fn test_decrypt_caesar_with_shift_twenty_six() {
    assert_eq!(Caesar::decrypt("Hello", 26), "Hello");
}
#[test]
fn test_decrypt_caesar_with_shift_twenty_seven() {
    assert_eq!(Caesar::decrypt("Ifmmp", 27), "Hello");
}
#[test]
fn test_decrypt_caesar_with_shift_negative_one() {
    assert_eq!(Caesar::decrypt("Gdkkn", -1), "Hello");
}
#[test]
fn test_decrypt_caesar_with_shift_negative_twenty_seven() {
    assert_eq!(Caesar::decrypt("Gdkkn", -27), "Hello");
}
#[test]
fn test_decrypt_caesar_with_non_character() {
    assert_eq!(Caesar::decrypt("Ifmmp!", 1), "Hello!");
}