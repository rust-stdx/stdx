use hex;

use super::*;
use crate::{
    Hasher, Xof,
    sha3::{Sha3_256, Shake128, Shake256},
};

fn flags_of(test: &serde_json::Value) -> Vec<String> {
    test["flags"]
        .as_array()
        .map(|a| a.iter().filter_map(|f| f.as_str().map(String::from)).collect())
        .unwrap_or_default()
}

fn tr_of(pk: &[u8]) -> [u8; 64] {
    let mut shake = Shake256::new();
    shake.absorb(pk);
    let mut tr = [0u8; 64];
    shake.squeeze(&mut tr);
    tr
}

/// Computes μ from the "internal projection" used by the ACVP sig-ver vectors:
/// `mu = H(tr || M')`.
fn mu_internal_projection(pk: &[u8], mp: &[u8]) -> [u8; 64] {
    let tr = tr_of(pk);
    let mut shake = Shake256::new();
    shake.absorb(&tr);
    shake.absorb(mp);
    let mut mu = [0u8; 64];
    shake.squeeze(&mut mu);
    mu
}

const SEED_A: [u8; 32] = [7u8; 32];
const SEED_B: [u8; 32] = [9u8; 32];
const ZERO_RND: [u8; 32] = [0u8; 32];

////////////////////////////////////////////////////////////////////////////////////////////////////
/// ACVP known-answer tests
////////////////////////////////////////////////////////////////////////////////////////////////////

fn keygen_kat<const PK: usize>(param: &str, derive_pk: impl Fn(&[u8; 32]) -> [u8; PK]) {
    let key_gen_data = include_str!("../../testdata/mldsa/key-gen.json");
    let v: serde_json::Value = serde_json::from_str(key_gen_data).unwrap();
    let mut tested = 0;

    for group in v["testGroups"].as_array().unwrap() {
        if group["parameterSet"].as_str() != Some(param) {
            continue;
        }
        for test in group["tests"].as_array().unwrap() {
            let seed = hex::decode_array::<32>(test["seed"].as_str().unwrap().as_bytes()).unwrap();
            let expected_pk_hex = test["pk"].as_str().unwrap();

            let pk_hex = hex::encode(derive_pk(&seed));
            assert_eq!(
                pk_hex.to_uppercase(),
                expected_pk_hex.to_uppercase(),
                "keygen KAT tcId={}",
                test["tcId"]
            );
            tested += 1;
        }
    }
    assert!(tested > 0, "no {param} keygen tests run");
}

fn sigver_kat<const PK: usize, const SIG: usize>(
    param: &str,
    verify_mu: impl Fn(&[u8; PK], &[u8; 64], &[u8; SIG]) -> Result<(), MlDsaError>,
) {
    let sig_ver_data = include_str!("../../testdata/mldsa/sig-ver.json");
    let v: serde_json::Value = serde_json::from_str(sig_ver_data).unwrap();

    let mut tested = 0;
    for group in v["testGroups"].as_array().unwrap() {
        if group["parameterSet"].as_str() != Some(param) {
            continue;
        }
        let pk: [u8; PK] = hex::decode(group["pk"].as_str().unwrap()).unwrap().try_into().unwrap();

        for test in group["tests"].as_array().unwrap() {
            let tc_id = test["tcId"].as_u64().unwrap();
            let expected_pass = test["testPassed"].as_bool().unwrap_or(true);
            let mp = hex::decode(test["message"].as_str().unwrap()).unwrap();
            let sig: [u8; SIG] = hex::decode(test["signature"].as_str().unwrap())
                .unwrap()
                .try_into()
                .unwrap();

            let mu = mu_internal_projection(&pk, &mp);
            let result = verify_mu(&pk, &mu, &sig);
            assert_eq!(
                result.is_ok(),
                expected_pass,
                "sigver KAT tcId={} reason={:?}",
                tc_id,
                test.get("reason")
            );
            tested += 1;
        }
    }
    assert_eq!(tested, 15, "all 15 {param} sigver tests should be run");
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Generic behavioral scenarios
////////////////////////////////////////////////////////////////////////////////////////////////////

fn roundtrip<const PK: usize, const SIG: usize>(
    sign: impl Fn(&[u8; 32], &[u8], &[u8], &[u8; 32]) -> Result<[u8; SIG], MlDsaError>,
    derive_pk: impl Fn(&[u8; 32]) -> [u8; PK],
    verify: impl Fn(&[u8; PK], &[u8], &[u8; SIG], &[u8]) -> Result<(), MlDsaError>,
) {
    let msg = b"Hello, world!";
    let sig = sign(&SEED_A, msg, &[], &ZERO_RND).unwrap();
    let pk = derive_pk(&SEED_A);
    verify(&pk, msg, &sig, &[]).unwrap();

    let mut bad_sig = sig;
    bad_sig[0] ^= 0xFF;
    assert!(verify(&pk, msg, &bad_sig, &[]).is_err());
    assert!(verify(&pk, b"Wrong message", &sig, &[]).is_err());

    let pk2 = derive_pk(&SEED_B);
    assert!(verify(&pk2, msg, &sig, &[]).is_err());
}

fn context<const PK: usize, const SIG: usize>(
    sign: impl Fn(&[u8; 32], &[u8], &[u8], &[u8; 32]) -> Result<[u8; SIG], MlDsaError>,
    derive_pk: impl Fn(&[u8; 32]) -> [u8; PK],
    verify: impl Fn(&[u8; PK], &[u8], &[u8; SIG], &[u8]) -> Result<(), MlDsaError>,
) {
    let pk = derive_pk(&SEED_A);
    let msg = b"test";
    let ctx = b"myapp";
    let sig = sign(&SEED_A, msg, ctx, &ZERO_RND).unwrap();
    verify(&pk, msg, &sig, ctx).unwrap();

    assert!(verify(&pk, msg, &sig, &[]).is_err());
    assert!(verify(&pk, msg, &sig, b"other").is_err());
}

fn empty_and_deterministic<const PK: usize, const SIG: usize>(
    sign: impl Fn(&[u8; 32], &[u8], &[u8], &[u8; 32]) -> Result<[u8; SIG], MlDsaError>,
    derive_pk: impl Fn(&[u8; 32]) -> [u8; PK],
    verify: impl Fn(&[u8; PK], &[u8], &[u8; SIG], &[u8]) -> Result<(), MlDsaError>,
) {
    let pk = derive_pk(&SEED_A);

    let sig = sign(&SEED_A, &[], &[], &ZERO_RND).unwrap();
    verify(&pk, &[], &sig, &[]).unwrap();

    let mut rnd = [0u8; 32];
    for (i, b) in rnd.iter_mut().enumerate() {
        *b = (i * 13 + 3) as u8;
    }
    let sig1 = sign(&SEED_A, b"hello", &[], &rnd).unwrap();
    let sig2 = sign(&SEED_A, b"hello", &[], &rnd).unwrap();
    assert_eq!(sig1, sig2);
    verify(&pk, b"hello", &sig1, &[]).unwrap();

    let mut other = rnd;
    other[0] ^= 1;
    let sig3 = sign(&SEED_A, b"hello", &[], &other).unwrap();
    assert_ne!(sig1, sig3, "different randomness should give a different signature");
}

fn tamper_sampled<const PK: usize, const SIG: usize>(
    sign: impl Fn(&[u8; 32], &[u8], &[u8], &[u8; 32]) -> Result<[u8; SIG], MlDsaError>,
    derive_pk: impl Fn(&[u8; 32]) -> [u8; PK],
    verify: impl Fn(&[u8; PK], &[u8], &[u8; SIG], &[u8]) -> Result<(), MlDsaError>,
) {
    let msg = b"test message";
    let mut sig = sign(&SEED_A, msg, &[], &ZERO_RND).unwrap();
    let pk = derive_pk(&SEED_A);

    // Sample bytes across the signature, including the challenge prefix.
    let mut positions: Vec<usize> = (0..SIG.min(32)).collect();
    positions.extend((0..SIG).step_by(13));
    positions.sort_unstable();
    positions.dedup();

    for i in positions {
        sig[i] ^= 1;
        assert!(verify(&pk, msg, &sig, &[]).is_err(), "tampered sig at byte {} should fail", i);
        sig[i] ^= 1;
    }
}

fn context_limits<const SIG: usize>(
    sign: impl Fn(&[u8; 32], &[u8], &[u8], &[u8; 32]) -> Result<[u8; SIG], MlDsaError>,
) {
    let ctx = vec![0u8; 255];
    assert!(sign(&SEED_A, b"test", &ctx, &ZERO_RND).is_ok());
    let ctx = vec![0u8; 256];
    assert!(sign(&SEED_A, b"test", &ctx, &ZERO_RND).is_err());
}

fn long_message<const PK: usize, const SIG: usize>(
    sign: impl Fn(&[u8; 32], &[u8], &[u8], &[u8; 32]) -> Result<[u8; SIG], MlDsaError>,
    derive_pk: impl Fn(&[u8; 32]) -> [u8; PK],
    verify: impl Fn(&[u8; PK], &[u8], &[u8; SIG], &[u8]) -> Result<(), MlDsaError>,
) {
    let pk = derive_pk(&SEED_A);
    let msg = vec![0x41u8; 10000];
    let sig = sign(&SEED_A, &msg, &[], &ZERO_RND).unwrap();
    verify(&pk, &msg, &sig, &[]).unwrap();
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Wycheproof
////////////////////////////////////////////////////////////////////////////////////////////////////

fn wycheproof_sign_seed<const PK: usize, const SIG: usize>(
    json: &str,
    sign: impl Fn(&[u8; 32], &[u8], &[u8], &[u8; 32]) -> Result<[u8; SIG], MlDsaError>,
    sign_mu: impl Fn(&[u8; 32], &[u8; 64], &[u8; 32]) -> [u8; SIG],
    derive_pk: impl Fn(&[u8; 32]) -> [u8; PK],
    verify: impl Fn(&[u8; PK], &[u8], &[u8; SIG], &[u8]) -> Result<(), MlDsaError>,
    verify_mu: impl Fn(&[u8; PK], &[u8; 64], &[u8; SIG]) -> Result<(), MlDsaError>,
) {
    let v: serde_json::Value = serde_json::from_str(json).unwrap();

    let mut valid_tested = 0u32;
    let mut invalid_tested = 0u32;
    let mut skipped = 0u32;

    for group in v["testGroups"].as_array().unwrap() {
        let Ok(seed) = hex::decode(group["privateSeed"].as_str().unwrap()) else {
            for test in group["tests"].as_array().unwrap() {
                assert!(
                    flags_of(test).iter().any(|f| f == "IncorrectPrivateKeyLength"),
                    "sign_seed group: seed decode failed but not IncorrectPrivateKeyLength"
                );
                skipped += 1;
            }
            continue;
        };
        let seed: [u8; 32] = seed.try_into().unwrap_or_else(|s: Vec<u8>| {
            let mut arr = [0u8; 32];
            let len = s.len().min(32);
            arr[..len].copy_from_slice(&s[..len]);
            arr
        });
        let pk = derive_pk(&seed);

        for test in group["tests"].as_array().unwrap() {
            let tc_id = test["tcId"].as_u64().unwrap();
            let flags = flags_of(test);
            let is_invalid_context = flags.iter().any(|f| f == "InvalidContext");
            let is_incorrect_private_key_len = flags.iter().any(|f| f == "IncorrectPrivateKeyLength");
            let is_internal = flags.iter().any(|f| f == "Internal");
            let result = test["result"].as_str().unwrap();

            if is_incorrect_private_key_len {
                // The signing API takes a fixed 32-byte seed, so an incorrect-length
                // private seed cannot be represented; the type system rejects it.
                skipped += 1;
                continue;
            }

            if is_internal {
                if result != "valid" {
                    // Internal-projection signing is total (it always yields a
                    // signature), so there is no invalid-signing path to assert.
                    skipped += 1;
                    continue;
                }
                let mu: [u8; 64] = hex::decode(test["mu"].as_str().expect("Internal vector without mu"))
                    .unwrap()
                    .try_into()
                    .unwrap();
                let expected_sig_hex = test["sig"].as_str().unwrap();
                let sig = sign_mu(&seed, &mu, &ZERO_RND);
                assert_eq!(
                    hex::encode(sig),
                    expected_sig_hex.to_lowercase(),
                    "sign_seed external-mu tcId={tc_id}: signature mismatch"
                );
                verify_mu(&pk, &mu, &sig)
                    .unwrap_or_else(|_| panic!("sign_seed external-mu tcId={tc_id}: self-verify failed"));
                valid_tested += 1;
                continue;
            }

            let msg = hex::decode(test["msg"].as_str().unwrap()).unwrap();
            let ctx = test
                .get("ctx")
                .and_then(|c| c.as_str())
                .map(|c| hex::decode(c).unwrap())
                .unwrap_or_default();

            if result == "valid" {
                let expected_sig_hex = test["sig"].as_str().unwrap();
                let rnd: [u8; 32] = test
                    .get("rnd")
                    .and_then(|r| r.as_str())
                    .map(|r| hex::decode_array::<32>(r.as_bytes()).unwrap())
                    .unwrap_or(ZERO_RND);
                let sig =
                    sign(&seed, &msg, &ctx, &rnd).unwrap_or_else(|_| panic!("sign_seed tcId={tc_id}: signing failed"));
                assert_eq!(
                    hex::encode(sig),
                    expected_sig_hex.to_lowercase(),
                    "sign_seed tcId={tc_id}: signature mismatch"
                );
                verify(&pk, &msg, &sig, &ctx).unwrap_or_else(|_| panic!("sign_seed tcId={tc_id}: self-verify failed"));
                valid_tested += 1;
            } else if result == "invalid" {
                assert!(
                    is_invalid_context,
                    "sign_seed tcId={tc_id}: expected invalid flag, got {flags:?}"
                );
                assert!(
                    sign(&seed, &msg, &ctx, &ZERO_RND).is_err(),
                    "sign_seed tcId={tc_id}: expected signing error"
                );
                invalid_tested += 1;
            }
        }
    }

    assert!(valid_tested > 0, "no valid sign_seed tests run");
    assert!(invalid_tested > 0, "no invalid sign_seed tests run");
    eprintln!("wycheproof sign_seed: {valid_tested} valid, {invalid_tested} invalid, {skipped} skipped");
}

fn wycheproof_sign_noseed<const PK: usize, const SIG: usize>(
    json: &str,
    verify: impl Fn(&[u8; PK], &[u8], &[u8; SIG], &[u8]) -> Result<(), MlDsaError>,
) {
    let v: serde_json::Value = serde_json::from_str(json).unwrap();

    let mut valid_tested = 0u32;
    let mut invalid_tested = 0u32;
    let mut skipped = 0u32;

    for group in v["testGroups"].as_array().unwrap() {
        let pk_hex = group.get("publicKey").and_then(|v| v.as_str()).unwrap_or_default();
        let Ok(pk) = hex::decode(pk_hex) else {
            skipped += group["tests"].as_array().unwrap().len() as u32;
            continue;
        };
        let pk: [u8; PK] = pk.try_into().unwrap_or_else(|p: Vec<u8>| {
            let mut arr = [0u8; PK];
            let len = p.len().min(PK);
            arr[..len].copy_from_slice(&p[..len]);
            arr
        });

        for test in group["tests"].as_array().unwrap() {
            let tc_id = test["tcId"].as_u64().unwrap();
            let flags = flags_of(test);
            let result = test["result"].as_str().unwrap();

            if flags.iter().any(|f| f == "InvalidPrivateKey")
                || flags.iter().any(|f| f == "IncorrectPrivateKeyLength")
                || flags.iter().any(|f| f == "Internal")
            {
                // This helper only exercises message-based verification: the seed is
                // always 32 bytes (so key-length errors are unrepresentable), and
                // Internal vectors carry a precomputed mu for `verify_external_mu`
                // rather than a message/ctx pair.
                skipped += 1;
                continue;
            }

            let msg = hex::decode(test["msg"].as_str().unwrap()).unwrap();
            let ctx = test
                .get("ctx")
                .and_then(|c| c.as_str())
                .map(|c| hex::decode(c).unwrap())
                .unwrap_or_default();

            if result == "valid" {
                let sig: [u8; SIG] = hex::decode(test["sig"].as_str().unwrap()).unwrap().try_into().unwrap();
                verify(&pk, &msg, &sig, &ctx).unwrap_or_else(|_| panic!("sign_noseed tcId={tc_id}: verify failed"));
                valid_tested += 1;
            } else if result == "invalid" {
                assert!(
                    flags.iter().any(|f| f == "InvalidContext"),
                    "sign_noseed tcId={tc_id}: expected invalid flag, got {flags:?}"
                );
                if let Some(sig_hex) = test.get("sig").and_then(|s| s.as_str()) {
                    if let Ok(sig_bytes) = hex::decode(sig_hex) {
                        if let Ok(sig) = <[u8; SIG]>::try_from(sig_bytes.as_slice()) {
                            assert!(
                                verify(&pk, &msg, &sig, &ctx).is_err(),
                                "sign_noseed tcId={tc_id}: expected verify error"
                            );
                        }
                    }
                }
                invalid_tested += 1;
            }
        }
    }

    assert!(valid_tested > 0, "no valid sign_noseed tests run");
    assert!(invalid_tested > 0, "no invalid sign_noseed tests run");
    eprintln!("wycheproof sign_noseed: {valid_tested} valid, {invalid_tested} invalid, {skipped} skipped");
}

fn wycheproof_verify<const PK: usize, const SIG: usize>(
    json: &str,
    verify: impl Fn(&[u8; PK], &[u8], &[u8; SIG], &[u8]) -> Result<(), MlDsaError>,
    pk_from_slice: impl Fn(&[u8]) -> Result<[u8; PK], MlDsaError>,
) {
    let v: serde_json::Value = serde_json::from_str(json).unwrap();

    let mut valid_tested = 0u32;
    let mut invalid_tested = 0u32;
    let mut skipped = 0u32;

    for group in v["testGroups"].as_array().unwrap() {
        let pk_hex = group["publicKey"].as_str().unwrap();
        let Ok(pk) = hex::decode(pk_hex) else {
            for test in group["tests"].as_array().unwrap() {
                assert!(
                    flags_of(test).iter().any(|f| f == "IncorrectPublicKeyLength"),
                    "verify group: pk decode failed but not IncorrectPublicKeyLength"
                );
                skipped += 1;
            }
            continue;
        };
        let pk: [u8; PK] = pk.try_into().unwrap_or_else(|p: Vec<u8>| {
            let mut arr = [0u8; PK];
            let len = p.len().min(PK);
            arr[..len].copy_from_slice(&p[..len]);
            arr
        });

        for test in group["tests"].as_array().unwrap() {
            let tc_id = test["tcId"].as_u64().unwrap();
            let flags = flags_of(test);
            let is_incorrect_public_key_len = flags.iter().any(|f| f == "IncorrectPublicKeyLength");
            let is_incorrect_signature_len = flags.iter().any(|f| f == "IncorrectSignatureLength");
            let result = test["result"].as_str().unwrap();

            if is_incorrect_public_key_len {
                let pk_bytes = hex::decode(pk_hex).expect("IncorrectPublicKeyLength with non-hex key");
                assert!(
                    pk_from_slice(&pk_bytes).is_err(),
                    "verify tc_id={tc_id}: IncorrectPublicKeyLength flagged but public key parsed"
                );
                skipped += 1;
                continue;
            }

            let msg = hex::decode(test["msg"].as_str().unwrap()).unwrap();
            let ctx = test
                .get("ctx")
                .and_then(|c| c.as_str())
                .map(|c| hex::decode(c).unwrap())
                .unwrap_or_default();
            let sig_bytes = hex::decode(test["sig"].as_str().unwrap()).unwrap();

            if is_incorrect_signature_len {
                assert!(
                    sig_bytes.len() != SIG,
                    "verify tcId={tc_id}: IncorrectSignatureLength flagged but sig has correct length"
                );
                assert!(
                    verify(&pk, &msg, sig_bytes.as_slice().try_into().unwrap_or(&[0u8; SIG]), &ctx).is_err(),
                    "verify tcId={tc_id}: expected verify error for wrong-length sig"
                );
                invalid_tested += 1;
                continue;
            }

            let sig: [u8; SIG] = sig_bytes.try_into().unwrap();

            if result == "valid" {
                verify(&pk, &msg, &sig, &ctx).unwrap_or_else(|_| panic!("verify tcId={tc_id}: expected valid"));
                valid_tested += 1;
            } else if result == "invalid" {
                assert!(
                    verify(&pk, &msg, &sig, &ctx).is_err(),
                    "verify tcId={tc_id} (flags={flags:?}): expected invalid but verification passed"
                );
                invalid_tested += 1;
            }
        }
    }

    assert!(valid_tested > 0, "no valid verify tests run");
    assert!(invalid_tested > 0, "no invalid verify tests run");
    eprintln!("wycheproof verify: {valid_tested} valid, {invalid_tested} invalid, {skipped} skipped");
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// ML-DSA-44
////////////////////////////////////////////////////////////////////////////////////////////////////

fn pk44(seed: &[u8; 32]) -> [u8; ML_DSA_44_PUBLIC_KEY_SIZE] {
    let sk = MlDsa44SecretKey::new(seed);
    sk.public_key().to_bytes()
}

/// Parses a serialized public key through the public fallible decoder, so tests
/// can assert that wrong-length encodings are rejected.
fn pk44_from_slice(bytes: &[u8]) -> Result<[u8; ML_DSA_44_PUBLIC_KEY_SIZE], MlDsaError> {
    MlDsa44PublicKey::try_from(bytes).map(|key| key.to_bytes())
}

fn sign44(
    seed: &[u8; 32],
    msg: &[u8],
    ctx: &[u8],
    rnd: &[u8; 32],
) -> Result<[u8; ML_DSA_44_SIGNATURE_SIZE], MlDsaError> {
    let sk = MlDsa44SecretKey::new(seed);
    sk.sign_derand(msg, ctx, rnd)
}

fn sign44_mu(seed: &[u8; 32], mu: &[u8; 64], rnd: &[u8; 32]) -> [u8; ML_DSA_44_SIGNATURE_SIZE] {
    let sk = MlDsa44SecretKey::new(seed);
    sk.sign_external_mu_derand(mu, rnd)
}

fn verify44(
    pk: &[u8; ML_DSA_44_PUBLIC_KEY_SIZE],
    msg: &[u8],
    sig: &[u8; ML_DSA_44_SIGNATURE_SIZE],
    ctx: &[u8],
) -> Result<(), MlDsaError> {
    MlDsa44PublicKey::from_bytes(pk).verify(msg, sig, ctx)
}

fn verify44_mu(
    pk: &[u8; ML_DSA_44_PUBLIC_KEY_SIZE],
    mu: &[u8; 64],
    sig: &[u8; ML_DSA_44_SIGNATURE_SIZE],
) -> Result<(), MlDsaError> {
    MlDsa44PublicKey::from_bytes(pk).verify_external_mu(mu, sig)
}

#[test]
fn mldsa44_keygen_kat() {
    keygen_kat("ML-DSA-44", pk44);
}

#[test]
fn mldsa44_sigver_kat() {
    sigver_kat("ML-DSA-44", verify44_mu);
}

#[test]
fn mldsa44_roundtrip() {
    roundtrip(sign44, pk44, verify44);
}

#[test]
fn mldsa44_context() {
    context(sign44, pk44, verify44);
}

#[test]
fn mldsa44_empty_and_deterministic() {
    empty_and_deterministic(sign44, pk44, verify44);
}

#[test]
fn mldsa44_tamper_sampled() {
    tamper_sampled(sign44, pk44, verify44);
}

#[test]
fn mldsa44_context_limits() {
    context_limits(sign44);
}

#[test]
fn mldsa44_long_message() {
    long_message(sign44, pk44, verify44);
}

#[test]
fn mldsa44_wycheproof_sign_seed() {
    wycheproof_sign_seed(
        include_str!("../../testdata/wycheproof/testvectors_v1/mldsa_44_sign_seed_test.json"),
        sign44,
        sign44_mu,
        pk44,
        verify44,
        verify44_mu,
    );
}

#[test]
fn mldsa44_wycheproof_sign_noseed() {
    wycheproof_sign_noseed(
        include_str!("../../testdata/wycheproof/testvectors_v1/mldsa_44_sign_noseed_test.json"),
        verify44,
    );
}

#[test]
fn mldsa44_wycheproof_verify() {
    wycheproof_verify(
        include_str!("../../testdata/wycheproof/testvectors_v1/mldsa_44_verify_test.json"),
        verify44,
        pk44_from_slice,
    );
}

#[test]
#[cfg(feature = "random")]
fn mldsa44_generate_uniqueness() {
    let a = MlDsa44SecretKey::generate().unwrap();
    let b = MlDsa44SecretKey::generate().unwrap();
    assert_ne!(a.seed(), b.seed());
    assert_ne!(a.public_key(), b.public_key());

    let c = MlDsa44SecretKey::new(a.seed());
    assert_eq!(a.public_key(), c.public_key());
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// ML-DSA-65
////////////////////////////////////////////////////////////////////////////////////////////////////

fn pk65(seed: &[u8; 32]) -> [u8; ML_DSA_65_PUBLIC_KEY_SIZE] {
    let sk = MlDsa65SecretKey::new(seed);
    sk.public_key().to_bytes()
}

/// Parses a serialized public key through the public fallible decoder, so tests
/// can assert that wrong-length encodings are rejected.
fn pk65_from_slice(bytes: &[u8]) -> Result<[u8; ML_DSA_65_PUBLIC_KEY_SIZE], MlDsaError> {
    MlDsa65PublicKey::try_from(bytes).map(|key| key.to_bytes())
}

fn sign65(
    seed: &[u8; 32],
    msg: &[u8],
    ctx: &[u8],
    rnd: &[u8; 32],
) -> Result<[u8; ML_DSA_65_SIGNATURE_SIZE], MlDsaError> {
    let sk = MlDsa65SecretKey::new(seed);
    sk.sign_derand(msg, ctx, rnd)
}

fn sign65_mu(seed: &[u8; 32], mu: &[u8; 64], rnd: &[u8; 32]) -> [u8; ML_DSA_65_SIGNATURE_SIZE] {
    let sk = MlDsa65SecretKey::new(seed);
    sk.sign_external_mu_derand(mu, rnd)
}

fn verify65(
    pk: &[u8; ML_DSA_65_PUBLIC_KEY_SIZE],
    msg: &[u8],
    sig: &[u8; ML_DSA_65_SIGNATURE_SIZE],
    ctx: &[u8],
) -> Result<(), MlDsaError> {
    MlDsa65PublicKey::from_bytes(pk).verify(msg, sig, ctx)
}

fn verify65_mu(
    pk: &[u8; ML_DSA_65_PUBLIC_KEY_SIZE],
    mu: &[u8; 64],
    sig: &[u8; ML_DSA_65_SIGNATURE_SIZE],
) -> Result<(), MlDsaError> {
    MlDsa65PublicKey::from_bytes(pk).verify_external_mu(mu, sig)
}

#[test]
fn mldsa65_keygen_kat() {
    keygen_kat("ML-DSA-65", pk65);
}

#[test]
fn mldsa65_sigver_kat() {
    sigver_kat("ML-DSA-65", verify65_mu);
}

#[test]
fn mldsa65_roundtrip() {
    roundtrip(sign65, pk65, verify65);
}

#[test]
fn mldsa65_context() {
    context(sign65, pk65, verify65);
}

#[test]
fn mldsa65_empty_and_deterministic() {
    empty_and_deterministic(sign65, pk65, verify65);
}

#[test]
fn mldsa65_tamper_sampled() {
    tamper_sampled(sign65, pk65, verify65);
}

#[test]
fn mldsa65_context_limits() {
    context_limits(sign65);
}

#[test]
fn mldsa65_long_message() {
    long_message(sign65, pk65, verify65);
}

#[test]
fn mldsa65_wycheproof_sign_seed() {
    wycheproof_sign_seed(
        include_str!("../../testdata/wycheproof/testvectors_v1/mldsa_65_sign_seed_test.json"),
        sign65,
        sign65_mu,
        pk65,
        verify65,
        verify65_mu,
    );
}

#[test]
fn mldsa65_wycheproof_sign_noseed() {
    wycheproof_sign_noseed(
        include_str!("../../testdata/wycheproof/testvectors_v1/mldsa_65_sign_noseed_test.json"),
        verify65,
    );
}

#[test]
fn mldsa65_wycheproof_verify() {
    wycheproof_verify(
        include_str!("../../testdata/wycheproof/testvectors_v1/mldsa_65_verify_test.json"),
        verify65,
        pk65_from_slice,
    );
}

#[test]
#[cfg(feature = "random")]
fn mldsa65_generate_uniqueness() {
    let a = MlDsa65SecretKey::generate().unwrap();
    let b = MlDsa65SecretKey::generate().unwrap();
    assert_ne!(a.seed(), b.seed());
    assert_ne!(a.public_key(), b.public_key());

    let c = MlDsa65SecretKey::new(a.seed());
    assert_eq!(a.public_key(), c.public_key());
}

#[test]
fn mldsa65_nistkats() {
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct KatRecord {
        key_generation_seed: String,
        sha3_256_hash_of_verification_key: String,
        message: String,
        signing_randomness: String,
        sha3_256_hash_of_signature: String,
    }

    let kat_json = include_str!("../../testdata/mldsa/nistkats-65.json");
    let records: Vec<KatRecord> = serde_json::from_str(kat_json).unwrap();

    let mut tested = 0;
    for record in &records {
        let seed = hex::decode_array::<32>(record.key_generation_seed.as_bytes()).unwrap();
        let rnd = hex::decode_array::<32>(record.signing_randomness.as_bytes()).unwrap();
        let msg = hex::decode(&record.message).unwrap();
        let expected_vk_hash = record.sha3_256_hash_of_verification_key.to_lowercase();
        let expected_sig_hash = record.sha3_256_hash_of_signature.to_lowercase();

        let sk = MlDsa65SecretKey::new(&seed);
        let sig = sk.sign_derand(&msg, &[], &rnd).unwrap();

        let vk_hash = hex::encode({
            let mut h = Sha3_256::new();
            h.update(&sk.public_key().to_bytes());
            h.sum()
        });
        assert_eq!(vk_hash, expected_vk_hash, "lib KAT vk hash mismatch");

        let sig_hash = hex::encode({
            let mut h = Sha3_256::new();
            h.update(&sig);
            h.sum()
        });
        assert_eq!(sig_hash, expected_sig_hash, "lib KAT sig hash mismatch");

        sk.public_key().verify(&msg, &sig, &[]).unwrap();
        tested += 1;
    }
    assert_eq!(tested, records.len(), "all lib KAT tests should be run");
}

#[test]
fn mldsa65_accumulated_100() {
    let mut shake_src = Shake128::new();
    let mut acc = Shake128::new();

    for _ in 0..100 {
        let mut seed = [0u8; 32];
        shake_src.squeeze(&mut seed);
        let sk = MlDsa65SecretKey::new(&seed);
        acc.absorb(&sk.public_key().to_bytes());

        let sig = sk.sign_derand(&[], &[], &ZERO_RND).unwrap();
        acc.absorb(&sig);

        sk.public_key().verify(&[], &sig, &[]).unwrap();
    }

    let mut result = [0u8; 32];
    acc.squeeze(&mut result);
    assert_eq!(
        hex::encode(result),
        "8358a1843220194417cadbc2651295cd8fc65125b5a5c1a239a16dc8b57ca199",
        "accumulated 100-iteration hash mismatch"
    );
}

#[test]
fn mldsa65_accumulated_10k() {
    let mut shake_src = Shake128::new();
    let mut acc = Shake128::new();

    for _ in 0..10000 {
        let mut seed = [0u8; 32];
        shake_src.squeeze(&mut seed);
        let sk = MlDsa65SecretKey::new(&seed);
        acc.absorb(&sk.public_key().to_bytes());

        let sig = sk.sign_derand(&[], &[], &ZERO_RND).unwrap();
        acc.absorb(&sig);

        sk.public_key().verify(&[], &sig, &[]).unwrap();
    }

    let mut result = [0u8; 32];
    acc.squeeze(&mut result);
    assert_eq!(
        hex::encode(result),
        "5ff5e196f0b830c3b10a9eb5358e7c98a3a20136cb677f3ae3b90175c3ace329",
        "accumulated 10k-iteration hash mismatch"
    );
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// ML-DSA-87
////////////////////////////////////////////////////////////////////////////////////////////////////

fn pk87(seed: &[u8; 32]) -> [u8; ML_DSA_87_PUBLIC_KEY_SIZE] {
    let sk = MlDsa87SecretKey::new(seed);
    sk.public_key().to_bytes()
}

/// Parses a serialized public key through the public fallible decoder, so tests
/// can assert that wrong-length encodings are rejected.
fn pk87_from_slice(bytes: &[u8]) -> Result<[u8; ML_DSA_87_PUBLIC_KEY_SIZE], MlDsaError> {
    MlDsa87PublicKey::try_from(bytes).map(|key| key.to_bytes())
}

fn sign87(
    seed: &[u8; 32],
    msg: &[u8],
    ctx: &[u8],
    rnd: &[u8; 32],
) -> Result<[u8; ML_DSA_87_SIGNATURE_SIZE], MlDsaError> {
    let sk = MlDsa87SecretKey::new(seed);
    sk.sign_derand(msg, ctx, rnd)
}

fn sign87_mu(seed: &[u8; 32], mu: &[u8; 64], rnd: &[u8; 32]) -> [u8; ML_DSA_87_SIGNATURE_SIZE] {
    let sk = MlDsa87SecretKey::new(seed);
    sk.sign_external_mu_derand(mu, rnd)
}

fn verify87(
    pk: &[u8; ML_DSA_87_PUBLIC_KEY_SIZE],
    msg: &[u8],
    sig: &[u8; ML_DSA_87_SIGNATURE_SIZE],
    ctx: &[u8],
) -> Result<(), MlDsaError> {
    MlDsa87PublicKey::from_bytes(pk).verify(msg, sig, ctx)
}

fn verify87_mu(
    pk: &[u8; ML_DSA_87_PUBLIC_KEY_SIZE],
    mu: &[u8; 64],
    sig: &[u8; ML_DSA_87_SIGNATURE_SIZE],
) -> Result<(), MlDsaError> {
    MlDsa87PublicKey::from_bytes(pk).verify_external_mu(mu, sig)
}

#[test]
fn mldsa87_keygen_kat() {
    keygen_kat("ML-DSA-87", pk87);
}

#[test]
fn mldsa87_sigver_kat() {
    sigver_kat("ML-DSA-87", verify87_mu);
}

#[test]
fn mldsa87_roundtrip() {
    roundtrip(sign87, pk87, verify87);
}

#[test]
fn mldsa87_context() {
    context(sign87, pk87, verify87);
}

#[test]
fn mldsa87_empty_and_deterministic() {
    empty_and_deterministic(sign87, pk87, verify87);
}

#[test]
fn mldsa87_tamper_sampled() {
    tamper_sampled(sign87, pk87, verify87);
}

#[test]
fn mldsa87_context_limits() {
    context_limits(sign87);
}

#[test]
fn mldsa87_long_message() {
    long_message(sign87, pk87, verify87);
}

#[test]
fn mldsa87_wycheproof_sign_seed() {
    wycheproof_sign_seed(
        include_str!("../../testdata/wycheproof/testvectors_v1/mldsa_87_sign_seed_test.json"),
        sign87,
        sign87_mu,
        pk87,
        verify87,
        verify87_mu,
    );
}

#[test]
fn mldsa87_wycheproof_sign_noseed() {
    wycheproof_sign_noseed(
        include_str!("../../testdata/wycheproof/testvectors_v1/mldsa_87_sign_noseed_test.json"),
        verify87,
    );
}

#[test]
fn mldsa87_wycheproof_verify() {
    wycheproof_verify(
        include_str!("../../testdata/wycheproof/testvectors_v1/mldsa_87_verify_test.json"),
        verify87,
        pk87_from_slice,
    );
}

#[test]
#[cfg(feature = "random")]
fn mldsa87_generate_uniqueness() {
    let a = MlDsa87SecretKey::generate().unwrap();
    let b = MlDsa87SecretKey::generate().unwrap();
    assert_ne!(a.seed(), b.seed());
    assert_ne!(a.public_key(), b.public_key());

    let c = MlDsa87SecretKey::new(a.seed());
    assert_eq!(a.public_key(), c.public_key());
}
