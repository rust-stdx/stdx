use std::{env, fs, process};

use crypto::{Aead, Xof, aes::Aes256Gcm, chacha::ChaCha20Blake3, sha3::Shake256};
use zeroize::{Zeroize, Zeroizing};

const KEY_LENGTH: usize = 32;
const NONCE_SEED_LENGTH: usize = 32;

const KDF_INFO_CHACHA20_BLAKE3_KEY: &str = "crypt ChaCha20-BLAKE3 key";
const KDF_INFO_CHACHA20_BLAKE3_NONCE: &str = "crypt ChaCha20-BLAKE3 nonce";
const CHACHA20_BLAKE3_NONCE_LENGTH: usize = 32;

const KDF_INFO_AES_KEY: &str = "crypt AES-256-GCM key";
const KDF_INFO_AES_NONCE: &str = "crypt AES-256-GCM nonce";
const AES_NONCE_LENGTH: usize = 12;

const ARGON2_SALT_LENGTH: usize = 32;
const ARGON2_ITERATIONS: u32 = 8;
const ARGON2_MEMORY_KB: u32 = 1024 * 1024; // 1 GiB
const ARGON2_LANES: u32 = 4;
const KDF_INFO_ARGON2_SALT: &str = "crypt Argon2 salt";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        print_help_and_exit(1);
    }

    let action = &args[1];
    let file_in = &args[2];
    let file_out = &args[3];

    let (confirm_password, encrypt_mode) = match action.as_str() {
        "encrypt" => (true, true),
        "decrypt" => (false, false),
        _ => print_help_and_exit(1),
    };

    let mut password = match ask_for_password(confirm_password) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    };

    let fn_ptr: fn(&[u8], &[u8]) -> Result<Vec<u8>, String> = if encrypt_mode { encrypt } else { decrypt };
    let result = process_file(password.as_bytes(), file_in, file_out, fn_ptr);
    password.zeroize();

    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn process_file(
    password: &[u8],
    file_in: &str,
    file_out: &str,
    f: fn(&[u8], &[u8]) -> Result<Vec<u8>, String>,
) -> Result<(), String> {
    if file_in == file_out {
        return Err("input file can't be the same as output file".to_string());
    }

    let mut data_in = fs::read(file_in).map_err(|e| format!("error reading [{file_in}]: {e}"))?;

    let mut data_out = f(password, &data_in)?;

    let write_result = fs::write(file_out, &data_out).map_err(|e| format!("error writing to [{file_out}]: {e}"));

    data_in.zeroize();
    data_out.zeroize();

    write_result
}

/// Production Argon2id parameters used by the CLI.
fn production_params() -> crypto::argon2::Params {
    crypto::argon2::Params {
        iterations: ARGON2_ITERATIONS,
        memory: ARGON2_MEMORY_KB,
        parallelism: ARGON2_LANES,
    }
}

// Returns nonce_seed (32 bytes) || chacha20_blake3_ciphertext
//
// chacha20_blake3_nonce = derive_key(nonce_seed, "...", 24)
// aes_nonce             = derive_key(nonce_seed, "...", 12)
// argon2_salt           = derive_key(nonce_seed, "...", 32)
fn encrypt(password: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, String> {
    encrypt_with_params(password, plaintext, &production_params())
}

/// Encrypt `plaintext` with `password` using explicit Argon2id parameters.
fn encrypt_with_params(password: &[u8], plaintext: &[u8], params: &crypto::argon2::Params) -> Result<Vec<u8>, String> {
    let nonce_seed: [u8; NONCE_SEED_LENGTH] = rand::random();

    let chacha20_nonce = derive_key::<CHACHA20_BLAKE3_NONCE_LENGTH>(&nonce_seed, KDF_INFO_CHACHA20_BLAKE3_NONCE);
    let aes_nonce = derive_key::<AES_NONCE_LENGTH>(&nonce_seed, KDF_INFO_AES_NONCE);
    let argon2_salt = derive_key::<ARGON2_SALT_LENGTH>(&nonce_seed, KDF_INFO_ARGON2_SALT);

    let root_key = argon2_derive_key(password, argon2_salt.as_slice(), params)?;

    let aes_key = derive_key::<KEY_LENGTH>(root_key.as_slice(), KDF_INFO_AES_KEY);
    let chacha20_key = derive_key::<KEY_LENGTH>(root_key.as_slice(), KDF_INFO_CHACHA20_BLAKE3_KEY);

    // Encrypt inner layer with AES-256-GCM
    let aes = Aes256Gcm::new(&aes_key);
    let mut aes_buf = plaintext.to_vec();
    let tag = aes.encrypt_in_place(&mut aes_buf, aes_nonce.as_slice(), &[]);
    aes_buf.extend_from_slice(tag.as_ref());

    // Encrypt outer layer with ChaCha20-BLAKE3
    let cipher = ChaCha20Blake3::new(&*chacha20_key);
    let outer_ciphertext = cipher.encrypt(&aes_buf, &*chacha20_nonce, &[]);

    let mut result = Vec::with_capacity(NONCE_SEED_LENGTH + outer_ciphertext.len());
    result.extend_from_slice(&nonce_seed);
    result.extend_from_slice(&outer_ciphertext);

    Ok(result)
}

fn decrypt(password: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
    decrypt_with_params(password, ciphertext, &production_params())
}

/// Decrypt `ciphertext` with `password` using explicit Argon2id parameters.
fn decrypt_with_params(password: &[u8], ciphertext: &[u8], params: &crypto::argon2::Params) -> Result<Vec<u8>, String> {
    if ciphertext.len() < (NONCE_SEED_LENGTH + ChaCha20Blake3::TAG_SIZE) {
        return Err("ciphertext is too short".to_string());
    }

    let nonce_seed = &ciphertext[..NONCE_SEED_LENGTH];
    let ciphertext = &ciphertext[NONCE_SEED_LENGTH..];

    let chacha20_nonce = derive_key::<CHACHA20_BLAKE3_NONCE_LENGTH>(&nonce_seed, KDF_INFO_CHACHA20_BLAKE3_NONCE);
    let aes_nonce = derive_key::<AES_NONCE_LENGTH>(&nonce_seed, KDF_INFO_AES_NONCE);
    let argon2_salt = derive_key::<ARGON2_SALT_LENGTH>(&nonce_seed, KDF_INFO_ARGON2_SALT);

    let root_key = argon2_derive_key(password, argon2_salt.as_slice(), params)?;

    let aes_key = derive_key::<KEY_LENGTH>(root_key.as_slice(), KDF_INFO_AES_KEY);
    let chacha20_key = derive_key::<KEY_LENGTH>(root_key.as_slice(), KDF_INFO_CHACHA20_BLAKE3_KEY);

    // Decrypt outer layer with ChaCha20-BLAKE3
    let cipher = ChaCha20Blake3::new(&*chacha20_key);
    let aes_ciphertext = cipher
        .decrypt(ciphertext, &*chacha20_nonce, &[])
        .map_err(|e| format!("error decrypting data with ChaCha20-BLAKE3: {e}"))?;

    // Decrypt inner layer with AES-256-GCM
    if aes_ciphertext.len() < Aes256Gcm::TAG_SIZE {
        return Err("ciphertext is too short for AES-256-GCM tag".to_string());
    }

    let aes = Aes256Gcm::new(&aes_key);
    let tag_pos = aes_ciphertext.len() - Aes256Gcm::TAG_SIZE;
    let tag: [u8; 16] = aes_ciphertext[tag_pos..].try_into().unwrap();
    let mut plaintext_buf = aes_ciphertext[..tag_pos].to_vec();
    aes.decrypt_in_place(&mut plaintext_buf, aes_nonce.as_slice(), &[], &tag)
        .map_err(|_| "error decrypting data with AES-256-GCM: authentication failed".to_string())?;

    Ok(plaintext_buf)
}

fn derive_key<const N: usize>(root_key: &[u8], info: &str) -> Zeroizing<[u8; N]> {
    let mut out = Zeroizing::new([0u8; N]);

    let mut shake = Shake256::new();
    shake.absorb(root_key);
    shake.absorb(&(root_key.len() as u64).to_le_bytes());
    shake.absorb(info.as_bytes());
    shake.absorb(&(info.len() as u64).to_le_bytes());
    shake.absorb(&(N as u64).to_le_bytes());
    shake.squeeze(out.as_mut_slice());

    return out;
}

fn argon2_derive_key(
    password: &[u8],
    salt: &[u8],
    params: &crypto::argon2::Params,
) -> Result<Zeroizing<[u8; KEY_LENGTH]>, String> {
    let mut key = Zeroizing::new([0u8; KEY_LENGTH]);
    crypto::argon2::derive_key(key.as_mut_slice(), password, salt, &[], &[], params)
        .map_err(|e| format!("error deriving key with argon2: {e}"))?;

    Ok(key)
}

fn ask_for_password(confirm: bool) -> Result<String, String> {
    eprint!("Password: ");
    let password = term::read_password().map_err(|e| format!("error reading password: {e}"))?;
    eprintln!();

    if password.is_empty() {
        return Err("password is empty".to_string());
    }

    if confirm {
        eprint!("Confirm Password: ");
        let confirmation = term::read_password().map_err(|e| format!("error reading password confirmation: {e}"))?;
        eprintln!();

        if password != confirmation {
            return Err("passwords don't match".to_string());
        }
    }

    Ok(password)
}

fn print_help_and_exit(exit_code: i32) -> ! {
    eprintln!("usage: crypt <encrypt|decrypt> <in> <out>");
    process::exit(exit_code);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        password: &'static str,
        data: &'static str,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                password: "",
                data: "",
            },
            TestCase {
                password: "password",
                data: "",
            },
            TestCase {
                password: "",
                data: "data",
            },
            TestCase {
                password: "password",
                data: "data",
            },
            TestCase {
                password: "password",
                // echo -n 'data' | shasum -a 512, repeated
                data: "77c7ce9a5d86bb386d443bb96390faa120633158699c8844c30b13ab0bf92760b7e4416aea397db91b4ac0e5dd56b8ef7e4b066162ab1fdc088319ce6defc87677c7ce9a5d86bb386d443bb96390faa120633158699c8844c30b13ab0bf92760b7e4416aea397db91b4ac0e5dd56b8ef7e4b066162ab1fdc088319ce6defc87677c7ce9a5d86bb386d443bb96390faa120633158699c8844c30b13ab0bf92760b7e4416aea397db91b4ac0e5dd56b8ef7e4b066162ab1fdc088319ce6defc876",
            },
        ]
    }

    /// Lightweight Argon2id parameters so the test suite stays fast and
    /// memory-friendly. The full production parameters are exercised by
    /// `test_encrypt_decrypt_full_params`, which is ignored by default.
    fn test_params() -> crypto::argon2::Params {
        crypto::argon2::Params {
            iterations: 2,
            memory: 64,
            parallelism: 1,
        }
    }

    fn roundtrip(test: &TestCase, params: &crypto::argon2::Params, i: usize) {
        let password = test.password.as_bytes();
        let data = test.data.as_bytes();

        let ciphertext = encrypt_with_params(password, data, params)
            .unwrap_or_else(|e| panic!("error encrypting data [{}]: {}", i, e));

        // Ciphertext must not equal plaintext
        assert!(
            ciphertext != data && (data.is_empty() || &ciphertext[..data.len()] != data),
            "ciphertext == data for {}",
            i
        );

        let plaintext = decrypt_with_params(password, &ciphertext, params)
            .unwrap_or_else(|e| panic!("error decrypting data [{}]: {}", i, e));

        // Wrong password must fail
        let mut wrong_password = test.password.to_string();
        wrong_password.push('1');
        let ciphertext2 = ciphertext.clone();
        let wrong_result = decrypt_with_params(wrong_password.as_bytes(), &ciphertext2, params);
        assert!(
            wrong_result.is_err(),
            "expected error when using invalid password decrypting data for [{}]",
            i
        );

        assert_eq!(
            plaintext,
            data,
            "data ({}) != decrypted plaintext ({}) for {}",
            test.data,
            String::from_utf8_lossy(&plaintext),
            i
        );
    }

    #[test]
    fn test_encrypt_decrypt() {
        let params = test_params();
        for (i, test) in test_cases().iter().enumerate() {
            roundtrip(test, &params, i);
        }
    }

    #[test]
    #[ignore = "uses production Argon2id parameters (1 GiB, 8 passes); run manually with --ignored"]
    fn test_encrypt_decrypt_full_params() {
        let params = production_params();
        // A single case is enough to validate the production profile without
        // dominating the test runtime.
        roundtrip(&test_cases()[3], &params, 3);
    }
}
