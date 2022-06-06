use mirage::Vernam;
use mirage::Cryptography;

// Test the encrypt Vernam function
#[test]
fn test_encrypt_vernam_with_only_lowercase_a() {
    assert_eq!(Vernam::encrypt("Hello", "aaaaa"), "Hello");
}
#[test]
fn test_encrypt_vernam_with_only_lowercase_b() {
    assert_eq!(Vernam::encrypt("Hello", "bbbbb"), "Ifmmp");
}
#[test]
fn test_encrypt_vernam_with_only_uppercase_b() {
    assert_eq!(Vernam::encrypt("Hello", "BBBBB"), "Ifmmp");
}
#[test]
fn test_encrypt_vernam_big_word() {
    assert_eq!(Vernam::encrypt("Lorem ipsum dolor sit amet",
        "VOVQQEEXBSWMLDKINTIBAC"), "Gcmuc mtpve zawrb avm inev")
}
#[test]
fn test_encrypt_vernam_big_word_with_whitespace() {
    assert_eq!(Vernam::encrypt("Lorem ipsum dolor sit amet",
        "VOVQQ EEXBS WMLDK INT IBAC"), "Gcmuc mtpve zawrb avm inev")
}

// Test the decrypt Vernam function
#[test]
fn test_decrypt_vernam_with_only_lowercase_a() {
    assert_eq!(Vernam::decrypt("Hello", "aaaaa"), "Hello");
}
#[test]
fn test_decrypt_vernam_with_only_lowercase_b() {
    assert_eq!(Vernam::decrypt("Ifmmp", "bbbbb"), "Hello");
}
#[test]
fn test_decrypt_vernam_with_only_uppercase_b() {
    assert_eq!(Vernam::decrypt("Ifmmp", "BBBBB"), "Hello");
}
#[test]
fn test_decrypt_vernam_big_word() {
    assert_eq!(Vernam::decrypt("Gcmuc mtpve zawrb avm inev",
        "VOVQQEEXBSWMLDKINTIBAC"), "Lorem ipsum dolor sit amet")
}
#[test]
fn test_decrypt_vernam_big_word_with_whitespace() {
    assert_eq!(Vernam::decrypt("Gcmuc mtpve zawrb avm inev",
        "VOVQQ EEXBS WMLDK INT IBAC"), "Lorem ipsum dolor sit amet")
}