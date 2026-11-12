#![cfg(feature = "alloc")]

use std::{fs, path::Path};

use cose::{Algorithm, Mac, Mac0, Sign1, SymmetricKey, VerifyOptions};
use serde_json::Value;

fn files(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            files(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
            out.push(path);
        }
    }
}

/// Fixtures are resolved relative to the package root: Cargo runs test
/// binaries there, and the wasm test runner preopens that directory as `/`.
/// Build-time `CARGO_MANIFEST_DIR` is a host path and does not exist inside
/// the WASI sandbox.
fn examples_dir() -> &'static Path {
    Path::new("testdata/examples")
}

fn b64(value: &Value) -> Vec<u8> {
    base64::decode(
        value.as_str().expect("base64 string").as_bytes(),
        base64::Alphabet::UrlNoPadding,
    )
    .expect("valid base64url")
}

fn hex_bytes(value: Option<&Value>) -> Vec<u8> {
    match value {
        Some(Value::String(value)) => hex::decode(value).unwrap_or_default(),
        _ => Vec::new(),
    }
}

fn verify_sign1(key: &Value, token: &[u8], external_aad: &[u8]) -> cose::Result<()> {
    let message = Sign1::from_slice(token)?;
    let kty = key["kty"].as_str().ok_or(cose::Error::InvalidKey)?;
    match kty {
        "EC" => {
            let crv = key["crv"].as_str().ok_or(cose::Error::InvalidKey)?;
            let x = b64(&key["x"]);
            let y = b64(&key["y"]);
            match crv {
                "P-256" => {
                    let key = crypto::p256::PublicKey::from_x_y(
                        x.as_slice().try_into().map_err(|_| cose::Error::InvalidKey)?,
                        y.as_slice().try_into().map_err(|_| cose::Error::InvalidKey)?,
                    )?;
                    message.verify(&key, external_aad, &VerifyOptions::default())
                }
                "P-384" => {
                    let key = crypto::p384::PublicKey::from_x_y(
                        x.as_slice().try_into().map_err(|_| cose::Error::InvalidKey)?,
                        y.as_slice().try_into().map_err(|_| cose::Error::InvalidKey)?,
                    )?;
                    message.verify(&key, external_aad, &VerifyOptions::default())
                }
                "P-521" => {
                    let key = crypto::p521::PublicKey::from_x_y(
                        x.as_slice().try_into().map_err(|_| cose::Error::InvalidKey)?,
                        y.as_slice().try_into().map_err(|_| cose::Error::InvalidKey)?,
                    )?;
                    message.verify(&key, external_aad, &VerifyOptions::default())
                }
                _ => Err(cose::Error::InvalidKey),
            }
        }
        "OKP" => {
            let x: [u8; 32] = b64(&key["x"]).try_into().map_err(|_| cose::Error::InvalidKey)?;
            let key = crypto::curve25519::ed25519::PublicKey::from_bytes(&x)?;
            message.verify(&key, external_aad, &VerifyOptions::default())
        }
        _ => Err(cose::Error::UnsupportedKeyType(cose::KeyType::Unknown(0))),
    }
}

fn verify_mac0(key: &Value, token: &[u8], external_aad: &[u8]) -> cose::Result<()> {
    let message = Mac0::from_slice(token)?;
    let algorithm = message.headers()?.algorithm().ok_or(cose::Error::MissingAlgorithm)?;
    let key_bytes = b64(&key["k"]);
    let key = SymmetricKey::new(algorithm, &key_bytes)?;
    message.verify(&key, external_aad, &VerifyOptions::default())
}

fn verify_mac(key: &Value, token: &[u8], external_aad: &[u8]) -> cose::Result<()> {
    let message = Mac::from_slice(token)?;
    let algorithm = message.headers()?.algorithm().ok_or(cose::Error::MissingAlgorithm)?;
    let key_bytes = b64(&key["k"]);
    let key = SymmetricKey::new(algorithm, &key_bytes)?;
    message.verify(&key, external_aad, &VerifyOptions::default())
}

#[test]
fn conformance_sign1_mac0_mac() {
    let root = examples_dir();
    let mut paths = Vec::new();
    for dir in ["sign1-tests", "mac0-tests", "mac-tests"] {
        files(&root.join(dir), &mut paths);
    }
    paths.sort();

    let mut passed = 0usize;
    let mut failed = Vec::new();

    for path in paths {
        let document: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        let fail = document.get("fail").and_then(Value::as_bool).unwrap_or(false);
        let output = document["output"]["cbor"].as_str().expect("output cbor").to_owned();
        let token = hex::decode(output).unwrap();

        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let dir = path
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let result = match dir.as_str() {
            "sign1-tests" => {
                let key = &document["input"]["sign0"]["key"];
                let aad = hex_bytes(document["input"]["sign0"].get("external"));
                verify_sign1(key, &token, &aad)
            }
            "mac0-tests" => {
                let key = &document["input"]["mac0"]["recipients"][0]["key"];
                let aad = hex_bytes(document["input"]["mac0"].get("external"));
                verify_mac0(key, &token, &aad)
            }
            "mac-tests" => {
                let key = &document["input"]["mac"]["recipients"][0]["key"];
                let aad = hex_bytes(document["input"]["mac"].get("external"));
                verify_mac(key, &token, &aad)
            }
            _ => unreachable!(),
        };

        if fail {
            if result.is_err() {
                passed += 1;
            } else {
                failed.push(format!("{dir}/{name}: expected failure, verified"));
            }
        } else if result.is_ok() {
            passed += 1;
        } else {
            failed.push(format!("{dir}/{name}: {result:?}"));
        }
    }

    assert!(failed.is_empty(), "{} failures:\n{}", failed.len(), failed.join("\n"));
    assert!(passed >= 20, "only {passed} vectors passed");
}

#[test]
fn conformance_rsa_pss_sign_vectors() {
    let root = examples_dir().join("rsa-pss-examples");
    let mut paths = Vec::new();
    files(&root, &mut paths);
    paths.sort();

    for path in paths {
        let document: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        let token = hex::decode(document["output"]["cbor"].as_str().unwrap()).unwrap();
        let message = cose::Sign::from_slice(&token).unwrap();
        let signers = document["input"]["sign"]["signers"].as_array().expect("signers");
        for (index, signer) in signers.iter().enumerate() {
            let key = &signer["key"];
            let n = hex::decode(key["n_hex"].as_str().unwrap()).unwrap();
            let e = hex::decode(key["e_hex"].as_str().unwrap()).unwrap();
            let algorithm = match signer["protected"]["alg"].as_str().unwrap() {
                "RSA-PSS-256" => Algorithm::PS256,
                "RSA-PSS-384" => Algorithm::PS384,
                "RSA-PSS-512" => Algorithm::PS512,
                other => panic!("unexpected RSA algorithm {other}"),
            };
            let verifier = cose::RsaPublicKey::from_n_e(algorithm, &n, &e).unwrap();
            message
                .verify(index, &verifier, b"", &VerifyOptions::default())
                .unwrap();

            let cose_key = cose::CoseKey::from(&verifier);
            let key_bytes = cose_key.to_vec();
            let parsed = cose::CoseKey::from_slice(&key_bytes).unwrap();
            assert_eq!(parsed.kty().unwrap(), cose::KeyType::Rsa);
            let recovered = cose::RsaPublicKey::try_from(&parsed).unwrap();
            message
                .verify(index, &recovered, b"", &VerifyOptions::default())
                .unwrap();
        }
    }
}

fn public_key_for_signer(key: &Value) -> Result<Box<dyn cose::Verifier>, cose::Error> {
    let kty = key["kty"].as_str().ok_or(cose::Error::InvalidKey)?;
    match kty {
        "EC" => match key["crv"].as_str().ok_or(cose::Error::InvalidKey)? {
            "P-256" => {
                let x = b64(&key["x"]);
                let y = b64(&key["y"]);
                Ok(Box::new(crypto::p256::PublicKey::from_x_y(
                    x.as_slice().try_into().map_err(|_| cose::Error::InvalidKey)?,
                    y.as_slice().try_into().map_err(|_| cose::Error::InvalidKey)?,
                )?))
            }
            "P-384" => {
                let x = b64(&key["x"]);
                let y = b64(&key["y"]);
                Ok(Box::new(crypto::p384::PublicKey::from_x_y(
                    x.as_slice().try_into().map_err(|_| cose::Error::InvalidKey)?,
                    y.as_slice().try_into().map_err(|_| cose::Error::InvalidKey)?,
                )?))
            }
            "P-521" => {
                let x = b64(&key["x"]);
                let y = b64(&key["y"]);
                Ok(Box::new(crypto::p521::PublicKey::from_x_y(
                    x.as_slice().try_into().map_err(|_| cose::Error::InvalidKey)?,
                    y.as_slice().try_into().map_err(|_| cose::Error::InvalidKey)?,
                )?))
            }
            _ => Err(cose::Error::InvalidKey),
        },
        "OKP" => {
            let bytes = if let Some(x) = key.get("x_hex").and_then(Value::as_str) {
                hex::decode(x).unwrap()
            } else {
                b64(&key["x"])
            };
            let x: [u8; 32] = bytes.try_into().map_err(|_| cose::Error::InvalidKey)?;
            Ok(Box::new(crypto::curve25519::ed25519::PublicKey::from_bytes(&x)?))
        }
        _ => Err(cose::Error::InvalidKey),
    }
}

#[test]
fn conformance_sign_and_ecdsa_eddsa() {
    let root = examples_dir();
    let mut paths = Vec::new();
    for dir in ["sign-tests", "ecdsa-examples", "eddsa-examples"] {
        files(&root.join(dir), &mut paths);
    }
    paths.sort();

    for path in paths {
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if name.contains("448") || name.starts_with("ecdsa-04") {
            // Ed448 and cross-curve SHA-512 ECDSA are outside the crypto crate's
            // supported key/algorithm pairs.
            continue;
        }
        let document: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        if document["input"].get("sign").is_none() {
            continue;
        }
        let has_unsupported_curve = document["input"]["sign"]["signers"]
            .as_array()
            .map(|signers| {
                signers
                    .iter()
                    .any(|signer| signer["key"]["crv"].as_str() == Some("Ed448"))
            })
            .unwrap_or(false);
        if has_unsupported_curve {
            continue;
        }
        let fail = document.get("fail").and_then(Value::as_bool).unwrap_or(false);
        let token = hex::decode(document["output"]["cbor"].as_str().unwrap()).unwrap();
        let signers = document["input"]["sign"]["signers"].as_array().expect("signers");

        let result = (|| -> cose::Result<()> {
            let message = cose::Sign::from_slice(&token)?;
            for (index, signer) in signers.iter().enumerate() {
                let verifier = public_key_for_signer(&signer["key"])?;
                let aad = hex_bytes(signer.get("external"));
                message.verify(index, verifier.as_ref(), &aad, &VerifyOptions::default())?;
            }
            Ok(())
        })();

        if fail {
            assert!(
                result.is_err(),
                "{}/{}: expected failure but verified",
                path.parent().unwrap().file_name().unwrap().to_string_lossy(),
                path.file_name().unwrap().to_string_lossy()
            );
        } else {
            assert!(
                result.is_ok(),
                "{}/{}: {:?}",
                path.parent().unwrap().file_name().unwrap().to_string_lossy(),
                path.file_name().unwrap().to_string_lossy(),
                result
            );
        }
    }
}
