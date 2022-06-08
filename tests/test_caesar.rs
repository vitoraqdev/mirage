use mirage::Caesar;
use mirage::Method;
use mirage::logger::DummyLogger;

// Test the encrypt caesar function
#[cfg(test)]
mod tests_caesar {
    use super::*;
    #[test]
    fn test_encrypt_caesar_with_shift_zero() {
        let caesar = Caesar::new(DummyLogger::new(0));
        assert_eq!(caesar.encrypt("Hello", "0"), "Hello");
    }
    #[test]
    fn test_encrypt_caesar_with_shift_one() {
        let caesar = Caesar::new(DummyLogger::new(0));
        assert_eq!(caesar.encrypt("Hello", "1"), "Ifmmp");
    }
    #[test]
    fn test_encrypt_caesar_with_shift_twenty_six() {
        let caesar = Caesar::new(DummyLogger::new(0));
        assert_eq!(caesar.encrypt("Hello", "26"), "Hello");
    }
    #[test]
    fn test_encrypt_caesar_with_shift_twenty_seven() {
        let caesar = Caesar::new(DummyLogger::new(0));
        assert_eq!(caesar.encrypt("Hello", "27"), "Ifmmp");
    }
    #[test]
    fn test_encrypt_caesar_with_shift_negative_one() {
        let caesar = Caesar::new(DummyLogger::new(0));
        assert_eq!(caesar.encrypt("Hello", "-1"), "Gdkkn");
    }
    #[test]
    fn test_encrypt_caesar_with_shift_negative_twenty_seven() {
        let caesar = Caesar::new(DummyLogger::new(0));
        assert_eq!(caesar.encrypt("Hello", "-27"), "Gdkkn");
    }
    #[test]
    fn test_encrypt_caesar_with_non_character() {
        let caesar = Caesar::new(DummyLogger::new(0));
        assert_eq!(caesar.encrypt("Hello!", "1"), "Ifmmp!");
    }
    #[test]
    fn test_encrypt_caesar_with_non_character_2() {
        let caesar = Caesar::new(DummyLogger::new(0));
        assert_eq!(caesar.encrypt("Hello!料", "1"), "Ifmmp!料");
    }


    // Test the decrypt caesar function
    #[test]
    fn test_decrypt_caesar_with_shift_zero() {
        let caesar = Caesar::new(DummyLogger::new(0));
        assert_eq!(caesar.decrypt("Hello", "0"), "Hello");
    }
    #[test]
    fn test_decrypt_caesar_with_shift_one() {
        let caesar = Caesar::new(DummyLogger::new(0));
        assert_eq!(caesar.decrypt("Ifmmp", "1"), "Hello");
    }
    #[test]
    fn test_decrypt_caesar_with_shift_twenty_six() {
        let caesar = Caesar::new(DummyLogger::new(0));
        assert_eq!(caesar.decrypt("Hello", "26"), "Hello");
    }
    #[test]
    fn test_decrypt_caesar_with_shift_twenty_seven() {
        let caesar = Caesar::new(DummyLogger::new(0));
        assert_eq!(caesar.decrypt("Ifmmp", "27"), "Hello");
    }
    #[test]
    fn test_decrypt_caesar_with_shift_negative_one() {
        let caesar = Caesar::new(DummyLogger::new(0));
        assert_eq!(caesar.decrypt("Gdkkn", "-1"), "Hello");
    }
    #[test]
    fn test_decrypt_caesar_with_shift_negative_twenty_seven() {
        let caesar = Caesar::new(DummyLogger::new(0));
        assert_eq!(caesar.decrypt("Gdkkn", "-27"), "Hello");
    }
    #[test]
    fn test_decrypt_caesar_with_non_character() {
        let caesar = Caesar::new(DummyLogger::new(0));
        assert_eq!(caesar.decrypt("Ifmmp!", "1"), "Hello!");
    }
    #[test]
    fn test_decrypt_caesar_with_non_character_2() {
        let caesar = Caesar::new(DummyLogger::new(0));
        assert_eq!(caesar.decrypt("Ifmmp!料", "1"), "Hello!料");
    }
}