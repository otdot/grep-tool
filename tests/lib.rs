use std::path::PathBuf;

use magic_crypt::{MagicCryptTrait, new_magic_crypt};
use pwtool::models::{Config, PwEntry};

#[test]
pub fn check_found_matches() {
    let pw_entry: PwEntry = serde_json::from_str(
        "{
        \"id\": \"a1\",
        \"key\": \"my-school\",
        \"url\": \"localhost:3000\",
        \"username\": \"user1\",
        \"password\": \"pass1\",
        \"note\": \"note\"
    }",
    )
    .unwrap();
    let mut result = vec![];
    pwtool::find_matches("school", pw_entry, &mut result);

    assert_eq!(result.first().unwrap().id, "a1");
}

#[test]
pub fn check_default_config() {
    let path = PathBuf::from("test-config.toml");
    let config: Config = pwtool::load_config(&path).unwrap();

    assert_eq!(
        config.password_vault_path.to_str(),
        Some("test-sample.json")
    )
}

#[test]
pub fn encrypt_string() {
    let path = PathBuf::from("test-config.toml");
    let config: Config = pwtool::load_config(&path).unwrap();
    let str_to_encrypt = "string_to_encrypt";

    let mc = new_magic_crypt!(&config.encryption_key, 256);
    let encrypted_str = mc.encrypt_to_base64(&str_to_encrypt);
    let expected_str = "uCOjocrbBfFefoB7GUyuVcxcFsee4/ZDxtKZeciFQ+Y=";

    assert_eq!(&expected_str, &encrypted_str);
}

#[test]
pub fn decrypt_string() {
    let path = PathBuf::from("test-config.toml");
    let config: Config = pwtool::load_config(&path).unwrap();
    let str_to_encrypt = "string_to_encrypt";

    let mc = new_magic_crypt!(&config.encryption_key, 256);
    let encrypted_str = mc.encrypt_to_base64(&str_to_encrypt);

    let decrypted_str = mc.decrypt_base64_to_string(&encrypted_str).unwrap();

    assert_eq!(&str_to_encrypt, &decrypted_str);
}

#[test]
pub fn decrypt_string_when_incorrect_key() {
    let path = PathBuf::from("test-config.toml");
    let config: Config = pwtool::load_config(&path).unwrap();
    let str_to_encrypt = "string_to_encrypt";

    let first_mc = new_magic_crypt!(&config.encryption_key, 256);
    let encrypted_str = first_mc.encrypt_to_base64(&str_to_encrypt);

    let incorrect_key = "my_incorrect_key";
    let second_mc = new_magic_crypt!(&incorrect_key, 256);

    let decrypted_string = second_mc.decrypt_base64_to_string(&encrypted_str);
    assert!(
        decrypted_string.is_err(),
        "expect decryption to return an error"
    )
}
