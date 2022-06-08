use mirage::Vernam;
use mirage::Method;
use mirage::logger::DummyLogger;

#[cfg(test)]
mod tests_vernam {
    use super::*;
    #[test]
    fn test_encrypt_vernam_with_only_lowercase_a() {
        let vernam = Vernam::new(DummyLogger::new(0));
        assert_eq!(vernam.encrypt("Hello", "aaaaa"), "Hello");
    }
    #[test]
    fn test_encrypt_vernam_with_only_lowercase_b() {
        let vernam = Vernam::new(DummyLogger::new(0));
        assert_eq!(vernam.encrypt("Hello", "bbbbb"), "Ifmmp");
    }
    #[test]
    fn test_encrypt_vernam_with_only_uppercase_b() {
        let vernam = Vernam::new(DummyLogger::new(0));
        assert_eq!(vernam.encrypt("Hello", "BBBBB"), "Ifmmp");
    }
    #[test]
    fn test_encrypt_vernam_big_word() {
        let vernam = Vernam::new(DummyLogger::new(0));
        assert_eq!(vernam.encrypt("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
            "DUPVGYZBSEATCBFQQNAXCDYZNLJUMGCBZAGTMDFHNNYCDLRGJIMVLVEDNXOFOKOOCUPJLAPCSJPTQEAGKXKZITZKBOTRHSGOVQZYLMEXLNYIMQKQTOUDGYCCXZGVLLFUFCYPAOXOROGDHJFPPXANHQWTGRQFYYGHWBSYNUJJYMCGXIGGMPRGCTXGXBJOQHUKQEYEMRMCCNUVZVHTMOVSMVYFVAJOXJNAQWTBBSXSRPEKWIYBWLJKNBCHJYQRWOKHEWVWYOYMQOMCQXKOIIOVYMAZHYBDVAIGWDYACEGMMEIMJUYPZUTFSVCUCXGGDZIYYITTAIRVMFKGFIMXQTPPXUOJXTGFELNDNFDNMXBIGCXEOEROU"), 
            "Oigzs gotmq dhnpw iyg ajgw, anadnwfkvvq ajbblxjvae gotk, ynl pj pdyvzlr yswdct ccltdxfmwi nj pahyoo db wnvpfx dhyto vbhofm. Yq pagu mt wygwg yklkcj, padd ytmytss elbftwzdartc jilntsk egsewgq tpoj mr nfrzsur ku mg iabdufh zuktnekhn. Nkmq egkq kthlz cjshd wi jqkpjcewrbavt yj opmmmlrii fatgu adbo pjnsdk tfhcbl iq aqewyf dixnq mkfqihpp. Qxblnuhpr aojw mceekomx kgycbpsum sgi rlqfjkqs, aslb bg ccckm veo tnrfsbp sbmsargz rswylg fqvy fe myv iepsiig."
        )
    }
    #[test]
    fn test_encrypt_vernam_with_non_character() {
        let vernam = Vernam::new(DummyLogger::new(0));
        assert_eq!(vernam.encrypt("Lorem ipsum dolor sit amet 料",
            "VOVQQ EEXBS WMLDK INT IBAC"), "Gcmuc mtpve zawrb avm inev 料")
    }
    #[test]
    #[should_panic(expected="Key must only contain alphabetic characters")]
    fn test_encrypt_vernam_with_non_character_on_key() {
        let vernam = Vernam::new(DummyLogger::new(0));
        vernam.encrypt("Lorem ipsum dolor sit amet",
            "VOVQQ EEXBS WMLDK INT IBAC 料"
        );
    }
    
    // Test the decrypt Vernam function
    #[test]
    fn test_decrypt_vernam_with_only_lowercase_a() {
        let vernam = Vernam::new(DummyLogger::new(0));
        assert_eq!(vernam.decrypt("Hello", "aaaaa"), "Hello");
    }
    #[test]
    fn test_decrypt_vernam_with_only_lowercase_b() {
        let vernam = Vernam::new(DummyLogger::new(0));
        assert_eq!(vernam.decrypt("Ifmmp", "bbbbb"), "Hello");
    }
    #[test]
    fn test_decrypt_vernam_with_only_uppercase_b() {
        let vernam = Vernam::new(DummyLogger::new(0));
        assert_eq!(vernam.decrypt("Ifmmp", "BBBBB"), "Hello");
    }
    #[test]
    fn test_decrypt_vernam_big_word() {
        let vernam = Vernam::new(DummyLogger::new(0));
        assert_eq!(vernam.decrypt("Oigzs gotmq dhnpw iyg ajgw, anadnwfkvvq ajbblxjvae gotk, ynl pj pdyvzlr yswdct ccltdxfmwi nj pahyoo db wnvpfx dhyto vbhofm. Yq pagu mt wygwg yklkcj, padd ytmytss elbftwzdartc jilntsk egsewgq tpoj mr nfrzsur ku mg iabdufh zuktnekhn. Nkmq egkq kthlz cjshd wi jqkpjcewrbavt yj opmmmlrii fatgu adbo pjnsdk tfhcbl iq aqewyf dixnq mkfqihpp. Qxblnuhpr aojw mceekomx kgycbpsum sgi rlqfjkqs, aslb bg ccckm veo tnrfsbp sbmsargz rswylg fqvy fe myv iepsiig.",
            "DUPVGYZBSEATCBFQQNAXCDYZNLJUMGCBZAGTMDFHNNYCDLRGJIMVLVEDNXOFOKOOCUPJLAPCSJPTQEAGKXKZITZKBOTRHSGOVQZYLMEXLNYIMQKQTOUDGYCCXZGVLLFUFCYPAOXOROGDHJFPPXANHQWTGRQFYYGHWBSYNUJJYMCGXIGGMPRGCTXGXBJOQHUKQEYEMRMCCNUVZVHTMOVSMVYFVAJOXJNAQWTBBSXSRPEKWIYBWLJKNBCHJYQRWOKHEWVWYOYMQOMCQXKOIIOVYMAZHYBDVAIGWDYACEGMMEIMJUYPZUTFSVCUCXGGDZIYYITTAIRVMFKGFIMXQTPPXUOJXTGFELNDNFDNMXBIGCXEOEROU"), 
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
        )
    }
    #[test]
    fn test_decrypt_vernam_with_non_character() {
        let vernam = Vernam::new(DummyLogger::new(0));
        assert_eq!(vernam.decrypt("Gcmuc mtpve zawrb avm inev 料",
            "VOVQQ EEXBS WMLDK INT IBAC"), "Lorem ipsum dolor sit amet 料")
    }
    #[test]
    #[should_panic(expected="Key must only contain alphabetic characters")]
    fn test_decrypt_vernam_with_non_character_on_key() {
        let vernam = Vernam::new(DummyLogger::new(0));
        vernam.decrypt("Gcmuc mtpve zawrb avm inev 料",
            "VOVQQ EEXBS WMLDK INT IBAC 料"
        );
    }
}