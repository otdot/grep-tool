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

#[test]
pub fn generate_pw_from_config() {
    let path = PathBuf::from("test-config.toml");
    let config: Config = pwtool::load_config(&path).unwrap();

    let generated_pw_vec = pwtool::generate_password_from_config(&config).unwrap();

    let assertion_length =
        u64::try_from(generated_pw_vec.len()).unwrap() == config.auto_generated_pws.word_length;
    let assertion_contains = generated_pw_vec.contains(&"santA1.".to_string())
        || generated_pw_vec.contains(&"k3kkoneN!".to_string())
        || generated_pw_vec.contains(&"autob4HN=".to_string());

    println!("generatedPwVecJoined: {}", generated_pw_vec.join(""));
    assert!(assertion_length);
    assert!(assertion_contains);
}

#[test]
pub fn transform_string_to_varying_case() {}
