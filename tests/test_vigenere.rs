use mirage::Vigenere;
use mirage::Method;
use mirage::logger::DummyLogger;

#[cfg(test)]
mod tests_vigenere {
    use super::*;
    #[test]
    fn test_vigenere_encrypt_with_key_a() {
        let vigenere = Vigenere::new(DummyLogger::new(0));
        assert_eq!(vigenere.encrypt("Hello", "a"), "Hello");
    }
    #[test]
    fn test_vigenere_encrypt_with_key_b() {
        let vigenere = Vigenere::new(DummyLogger::new(0));
        assert_eq!(vigenere.encrypt("Hello", "b"), "Ifmmp");
    }
    #[test]
    fn test_vigenere_encrypt() {
        let vigenere = Vigenere::new(DummyLogger::new(0));
        assert_eq!(vigenere.encrypt("Hello world!", "foobar"), "Mszmo ntfze!");
    }
    #[test]
    fn test_vigenere_encrypt_with_key_with_whitespace() {
        let vigenere = Vigenere::new(DummyLogger::new(0));
        assert_eq!(vigenere.encrypt("Hello world!", "foo bar"), "Mszmo ntfze!");
    }
    #[test]
    fn test_vigenere_encrypt_decrypt_with_key_length_one() {
        let vigenere = Vigenere::new(DummyLogger::new(0));
        let key = "k";
        let plaintext = "plaintext";
        let ciphertext = "zvksxdohd";
    
        assert_eq!(vigenere.encrypt(plaintext, key), ciphertext);
        assert_eq!(vigenere.decrypt(ciphertext, key), plaintext);
    
    }
    #[test]
    fn test_vigenere_encrypt_decrypt() {
        let vigenere = Vigenere::new(DummyLogger::new(0));
        let key = "key";
        let plaintext = "plaintext";
        let ciphertext = "zpysrrobr";
    
        assert_eq!(vigenere.encrypt(plaintext, key), ciphertext);
        assert_eq!(vigenere.decrypt(ciphertext, key), plaintext);
    }
    #[test]
    fn test_vigenere_encrypt_decrypt_2() {
        let vigenere = Vigenere::new(DummyLogger::new(0));
        let key = "anotherkey";
        let plaintext = "plaintext";
        let ciphertext = "pyobuxvhx";
    
        assert_eq!(vigenere.encrypt(plaintext, key), ciphertext);
        assert_eq!(vigenere.decrypt(ciphertext, key), plaintext);
    }
    #[test]
    fn test_vigenere_encrypt_decrypt_big_word() {
        let vigenere = Vigenere::new(DummyLogger::new(0));
        let key = "secret key";
        let plaintext = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
        let ciphertext = "Dstvq bzwse hqcsk cmr sqgk, ghxwcuxgkyk khghmutmgq ijax, uvh wy igmwofh moqngv kegbnmbmrv lx ekfmji gk hhvspw qcxrt kpgiyc. Lx xxmk sh ozrbw zcfmcd, unsw lgwviyw obcjgkkemssl mpnrqvy pytstzw gswg mx ccmjemn wb gr ghwqmvs efrloussx. Flml kyrw mtlvx nsjgv ke vxzvczipuiksx gf zqcyiderw zgcmm owqw gkcpnw hmdstv in pyeaev eyeve nsvkrxnb. Ivuirkinb wgfx qtgtogyl gwgmwkxyl rqe tkymbwrv, jygd ml uynge jem mxjktmt niqwvwex fypjax cemf sh ckx nrfhbyk.";
    
        assert_eq!(vigenere.encrypt(plaintext, key), ciphertext);
        assert_eq!(vigenere.decrypt(ciphertext, key), plaintext);
    }
    #[test]
    fn test_vigenere_encrypt_decrypt_with_special_characters() {
        let vigenere = Vigenere::new(DummyLogger::new(0));
        let key = "key";
        let plaintext = "plaintext!@#$%^&*()_+=-~`plaintext";
        let ciphertext = "zpysrrobr!@#$%^&*()_+=-~`zpysrrobr";
    
        assert_eq!(vigenere.encrypt(plaintext, key), ciphertext);
        assert_eq!(vigenere.decrypt(ciphertext, key), plaintext);
    }
    #[test]
    fn test_vigenere_encrypt_decrypt_with_non_character() {
        let vigenere = Vigenere::new(DummyLogger::new(0));
        let key = "key";
        let plaintext = "plaintext 料";
        let ciphertext = "zpysrrobr 料";
    
        assert_eq!(vigenere.encrypt(plaintext, key), ciphertext);
        assert_eq!(vigenere.decrypt(ciphertext, key), plaintext);
    }
    #[test]
    #[should_panic]
    fn test_vigenere_encrypt_without_key() {
        let vigenere = Vigenere::new(DummyLogger::new(0));
        let key = "";
        let plaintext = "plaintext";
    
        vigenere.encrypt(plaintext, key);
    }
    #[test]
    #[should_panic(expected = "Key must not be empty")]
    fn test_vigenere_decrypt_without_key() {
        let vigenere = Vigenere::new(DummyLogger::new(0));
        let key = "";
        let ciphertext = "plaintext";
    
        vigenere.decrypt(ciphertext, key);
    }
    #[test]
    #[should_panic(expected="Key must only contain alphabetic characters")]
    fn test_vigenere_decrypt_with_invalid_key() {
        let vigenere = Vigenere::new(DummyLogger::new(0));
        let key = "料";
        let ciphertext = "plaintext";
    
        vigenere.decrypt(ciphertext, key);
    }
}
