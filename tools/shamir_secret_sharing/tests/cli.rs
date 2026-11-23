use std::{
    io::Write,
    process::{Command, Output, Stdio},
};

use base64::{Alphabet, decode, encode};

fn run(args: &[&str], input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_shamir_secret_sharing"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to start shamir_secret_sharing");
    child
        .stdin
        .as_mut()
        .expect("stdin must be piped")
        .write_all(input.as_bytes())
        .expect("failed to write test input");
    return child
        .wait_with_output()
        .expect("failed to wait for shamir_secret_sharing");
}

fn stdout_lines(output: &Output) -> Vec<String> {
    assert!(
        output.status.success(),
        "expected success, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    return String::from_utf8(output.stdout.clone())
        .expect("stdout must be UTF-8")
        .lines()
        .map(String::from)
        .collect();
}

fn failure(output: &Output) -> String {
    assert!(
        !output.status.success(),
        "expected failure, stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    return String::from_utf8_lossy(&output.stderr).into_owned();
}

#[test]
fn split_then_combine_roundtrips_a_typed_secret() {
    let secret = "correct horse battery staple";
    let split = run(&["split", "-n", "5", "-t", "3"], &format!("{secret}\n{secret}\n"));
    let shares = stdout_lines(&split);
    assert_eq!(shares.len(), 5);

    let combined = run(&["combine"], &format!("{}\n{}\n{}\n", shares[0], shares[2], shares[4]));
    assert!(
        combined.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&combined.stderr)
    );
    assert_eq!(combined.stdout.as_slice(), secret.as_bytes());
}

#[test]
fn split_then_combine_roundtrips_a_multibyte_utf8_secret() {
    let secret = "héllo, 🔐 wörld";
    let split = run(&["split", "-n", "2"], &format!("{secret}\n{secret}\n"));
    let shares = stdout_lines(&split);
    assert_eq!(shares.len(), 2);

    let combined = run(&["combine"], &format!("{}\n{}\n", shares[0], shares[1]));
    assert_eq!(combined.stdout.as_slice(), secret.as_bytes());
}

#[test]
fn split_defaults_to_three_shares() {
    let secret = "default shares";
    let split = run(&["split"], &format!("{secret}\n{secret}\n"));
    let shares = stdout_lines(&split);
    assert_eq!(shares.len(), 3);

    let combined = run(&["combine"], &format!("{}\n{}\n", shares[0], shares[2]));
    assert_eq!(combined.stdout.as_slice(), secret.as_bytes());
}

#[test]
fn default_threshold_is_two_for_three_shares() {
    let secret = "default threshold";
    let split = run(&["split", "-n", "3"], &format!("{secret}\n{secret}\n"));
    let shares = stdout_lines(&split);

    let combined = run(&["combine"], &format!("{}\n{}\n", shares[0], shares[2]));
    assert_eq!(combined.stdout.as_slice(), secret.as_bytes());
}

#[test]
fn random_secret_is_printed_and_reconstructs_exactly() {
    let split = run(&["split", "--random=64", "-n", "4", "-t", "2"], "");
    let lines = stdout_lines(&split);
    assert_eq!(lines.len(), 5);

    let encoded_secret = lines[0].strip_prefix("secret: ").expect("secret line prefix");
    let secret = decode(encoded_secret, Alphabet::Standard).expect("secret must be base64");
    assert_eq!(secret.len(), 64);

    let combined = run(&["combine"], &format!("{}\n{}\n", lines[1], lines[4]));
    assert_eq!(combined.stdout, secret);
}

#[test]
fn full_random_output_can_be_piped_back_into_combine() {
    let split = run(&["split", "--random=48", "-n", "3", "-t", "2"], "");
    let lines = stdout_lines(&split);
    let encoded_secret = lines[0].strip_prefix("secret: ").unwrap();
    let secret = decode(encoded_secret, Alphabet::Standard).unwrap();

    let combined = run(&["combine"], &String::from_utf8(split.stdout.clone()).unwrap());
    assert_eq!(combined.stdout, secret);
}

#[test]
fn random_accepts_separate_long_option_and_short_option() {
    for args in [
        vec!["split", "-n", "2", "--random", "16"],
        vec!["split", "-n", "2", "--random=16"],
        vec!["split", "-n", "2", "-r", "16"],
        vec!["split", "-n", "2", "-r16"],
    ] {
        let split = run(&args, "");
        let lines = stdout_lines(&split);
        let encoded_secret = lines[0].strip_prefix("secret: ").unwrap();
        assert_eq!(decode(encoded_secret, Alphabet::Standard).unwrap().len(), 16, "{args:?}");
    }
}

#[test]
fn random_defaults_to_32_bytes() {
    let split = run(&["split", "-n", "2", "-r"], "");
    let lines = stdout_lines(&split);
    let encoded_secret = lines[0].strip_prefix("secret: ").unwrap();
    assert_eq!(decode(encoded_secret, Alphabet::Standard).unwrap().len(), 32);
}

#[test]
fn random_across_payload_and_base64_boundaries() {
    for length in [1usize, 2, 3, 4, 5, 6, 7, 47, 48, 49, 135, 136, 137, 255, 256, 257] {
        let split = run(&["split", "-n", "2", "--random", &length.to_string()], "");
        let lines = stdout_lines(&split);
        assert_eq!(lines.len(), 3, "length {length}");

        let encoded_secret = lines[0].strip_prefix("secret: ").unwrap();
        let secret = decode(encoded_secret, Alphabet::Standard).unwrap();
        assert_eq!(secret.len(), length);

        let combined = run(&["combine"], &format!("{}\n{}\n", lines[1], lines[2]));
        assert_eq!(combined.stdout, secret, "length {length}");
    }
}

#[test]
fn combine_accepts_input_without_a_trailing_newline() {
    let split = run(&["split", "-n", "2", "-t", "2"], "secret\nsecret\n");
    let lines = stdout_lines(&split);
    let combined = run(&["combine"], &format!("{}\n{}", lines[0], lines[1]));
    assert_eq!(combined.stdout.as_slice(), b"secret");
}

#[test]
fn combine_accepts_crlf_input() {
    let split = run(&["split", "-n", "2", "-t", "2"], "secret\nsecret\n");
    let lines = stdout_lines(&split);
    let combined = run(&["combine"], &format!("{}\r\n{}\r\n", lines[0], lines[1]));
    assert_eq!(combined.stdout.as_slice(), b"secret");
}

#[test]
fn split_reads_crlf_secrets_from_a_pipe() {
    let split = run(&["split", "-n", "2"], "secret\r\nsecret\r\n");
    let lines = stdout_lines(&split);
    let combined = run(&["combine"], &format!("{}\n{}\n", lines[0], lines[1]));
    assert_eq!(combined.stdout.as_slice(), b"secret");
}

#[test]
fn combine_reports_an_empty_line_before_enough_shares() {
    let split = run(&["split", "-n", "3", "-t", "3"], "secret\nsecret\n");
    let lines = stdout_lines(&split);
    let error = failure(&run(&["combine"], &format!("{}\n\n{}\n{}\n", lines[0], lines[1], lines[2])));
    assert!(error.contains("not enough shares"), "{error}");
}

#[test]
fn generated_shares_are_distinct_and_differ_from_the_secret() {
    let split = run(&["split", "-n", "5", "-t", "2"], "secret\nsecret\n");
    let lines = stdout_lines(&split);
    assert_eq!(lines.len(), 5);
    for (i, line) in lines.iter().enumerate() {
        assert_ne!(line.as_bytes(), b"secret");
        for other in &lines[..i] {
            assert_ne!(line, other);
        }
    }
}

#[test]
fn combine_ignores_secret_lines_even_without_a_random_split() {
    let split = run(&["split", "-n", "2", "-t", "2"], "s3cret\ns3cret\n");
    let lines = stdout_lines(&split);
    let input = format!("secret: aWdub3JlZA==\n{}\n{}\n", lines[0], lines[1]);
    let combined = run(&["combine"], &input);
    assert_eq!(combined.stdout.as_slice(), b"s3cret");
}

#[test]
fn one_of_n_shares_reconstructs_the_secret() {
    let split = run(&["split", "-n", "3", "-t", "1"], "trivial\ntrivial\n");
    let lines = stdout_lines(&split);
    let combined = run(&["combine"], &format!("{}\n", lines[1]));
    assert_eq!(combined.stdout.as_slice(), b"trivial");
}

#[test]
fn combine_reports_too_few_shares() {
    let split = run(&["split", "-n", "3", "-t", "2"], "secret\nsecret\n");
    let lines = stdout_lines(&split);
    let error = failure(&run(&["combine"], &format!("{}\n", lines[0])));
    assert!(error.contains("not enough shares"), "{error}");
}

#[test]
fn combine_reports_duplicate_shares() {
    let split = run(&["split", "-n", "3", "-t", "2"], "secret\nsecret\n");
    let lines = stdout_lines(&split);
    let error = failure(&run(&["combine"], &format!("{}\n{}\n", lines[0], lines[0])));
    assert!(error.contains("duplicate share index"), "{error}");
}

#[test]
fn combine_reports_mixed_thresholds() {
    let first = run(&["split", "-n", "2", "-t", "2"], "secret\nsecret\n");
    let first_lines = stdout_lines(&first);
    let second = run(&["split", "-n", "3", "-t", "3"], "secret\nsecret\n");
    let second_lines = stdout_lines(&second);

    let error = failure(&run(&["combine"], &format!("{}\n{}\n", first_lines[0], second_lines[1])));
    assert!(error.contains("checksum") || error.contains("threshold"), "{error}");
}

#[test]
fn combine_reports_a_corrupted_share() {
    let split = run(&["split", "-n", "3", "-t", "2"], "secret\nsecret\n");
    let lines = stdout_lines(&split);

    let mut payload = decode(&lines[0], Alphabet::Standard).unwrap();
    let last = payload.len() - 1;
    payload[last] ^= 0x01;
    let corrupted = encode(&payload, Alphabet::Standard);

    let error = failure(&run(&["combine"], &format!("{corrupted}\n{}\n", lines[1])));
    assert!(error.contains("checksum"), "{error}");
}

#[test]
fn combine_rejects_a_truncated_share() {
    let split = run(&["split", "-n", "2", "-t", "2"], "secret\nsecret\n");
    let lines = stdout_lines(&split);
    let error = failure(&run(&["combine"], &format!("{}\n", &lines[0][..4])));
    assert!(error.contains("base64") || error.contains("too short"), "{error}");
}

#[test]
fn combine_rejects_free_arguments() {
    let error = failure(&run(&["combine", "some-share"], ""));
    assert!(error.contains("unexpected argument"), "{error}");
}

#[test]
fn split_rejects_invalid_options() {
    let cases = [
        vec!["split", "-n", "0"],
        vec!["split", "-n", "1"],
        vec!["split", "-n", "256"],
        vec!["split", "-n", "abc"],
        vec!["split", "-n", "3", "-t", "0"],
        vec!["split", "-n", "3", "-t", "4"],
        vec!["split", "-n", "2", "--random=0"],
        vec!["split", "-n", "2", "--random=1048577"],
        vec!["split", "-n", "2", "extra"],
        vec!["unknown"],
    ];
    for args in cases {
        let error = failure(&run(&args, ""));
        assert!(!error.is_empty(), "expected an error for {args:?}");
    }
}

#[test]
fn split_rejects_empty_and_mismatched_secrets() {
    let error = failure(&run(&["split", "-n", "2"], "\n\n"));
    assert!(error.contains("empty"), "{error}");

    let error = failure(&run(&["split", "-n", "2"], "first\nsecond\n"));
    assert!(error.contains("don't match"), "{error}");
}

#[test]
fn help_is_available_for_the_program_and_each_command() {
    for args in [vec!["--help"], vec!["split", "--help"], vec!["combine", "--help"]] {
        let output = run(&args, "");
        assert!(output.status.success(), "{args:?}");
        assert!(!output.stdout.is_empty(), "{args:?}");
    }
}
