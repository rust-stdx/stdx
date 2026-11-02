use padme::{padded_len, padding_len};

/// Examples from the Padmé paper's Table 1 (page 17): the values 8, 9, and 10
/// are used to illustrate the floating-point representation. `9` is not a
/// permitted length and is rounded up to `10`.
const OFFICIAL_VECTORS: &[(usize, usize)] = &[(8, 8), (9, 10), (10, 10)];

/// Generated with `jedisct1/rust-padme-padding` at commit
/// `4861700fa4e75fa9c50c8f6168257dfa581b46a7`
/// (`Padme::padded_len`). The reference implementation underflows for zero;
/// zero is covered separately as a defined edge case.
const JEDISCT1_VECTORS: &[(usize, usize)] = &[
    (1, 1),
    (2, 2),
    (3, 3),
    (4, 4),
    (5, 5),
    (6, 6),
    (7, 7),
    (8, 8),
    (9, 10),
    (10, 10),
    (11, 12),
    (12, 12),
    (13, 14),
    (14, 14),
    (15, 16),
    (16, 16),
    (17, 18),
    (18, 18),
    (19, 20),
    (20, 20),
    (21, 22),
    (22, 22),
    (23, 24),
    (24, 24),
    (25, 26),
    (26, 26),
    (27, 28),
    (28, 28),
    (29, 30),
    (30, 30),
    (31, 32),
    (32, 32),
    (33, 36),
    (34, 36),
    (35, 36),
    (36, 36),
    (37, 40),
    (38, 40),
    (39, 40),
    (40, 40),
    (41, 44),
    (42, 44),
    (43, 44),
    (44, 44),
    (45, 48),
    (46, 48),
    (47, 48),
    (48, 48),
    (49, 52),
    (50, 52),
    (51, 52),
    (52, 52),
    (53, 56),
    (54, 56),
    (55, 56),
    (56, 56),
    (57, 60),
    (58, 60),
    (59, 60),
    (60, 60),
    (61, 64),
    (62, 64),
    (63, 64),
    (64, 64),
    (100, 104),
    (127, 128),
    (128, 128),
    (129, 144),
    (200, 208),
    (255, 256),
    (256, 256),
    (257, 272),
    (300, 304),
    (384, 384),
    (385, 400),
    (400, 400),
    (511, 512),
    (512, 512),
    (513, 544),
    (600, 608),
    (768, 768),
    (800, 800),
    (1000, 1024),
    (1023, 1024),
    (1024, 1024),
    (1025, 1088),
    (1200, 1216),
    (1500, 1536),
    (1536, 1536),
    (1800, 1856),
    (2000, 2048),
    (2048, 2048),
    (2049, 2176),
    (2500, 2560),
    (3000, 3072),
    (4000, 4096),
    (4096, 4096),
    (4097, 4352),
    (5000, 5120),
    (6000, 6144),
    (8000, 8192),
    (8192, 8192),
    (10000, 10240),
    (12000, 12288),
    (16384, 16384),
    (20000, 20480),
    (24576, 24576),
    (32768, 32768),
    (40000, 40960),
    (49152, 49152),
    (65536, 65536),
    (80000, 81920),
    (98304, 98304),
    (131072, 131072),
    (160000, 163840),
    (196608, 196608),
    (262144, 262144),
    (320000, 327680),
    (393216, 393216),
    (524288, 524288),
    (640000, 655360),
    (786432, 786432),
    (900000, 901120),
    (902034, 917504),
    (1000000, 1015808),
    (1048576, 1048576),
];

#[test]
fn official_paper_examples() {
    for &(len, expected) in OFFICIAL_VECTORS {
        assert_eq!(padded_len(len), expected, "length {len}");
    }
}

#[test]
fn jedisct1_implementation_vectors() {
    for &(len, expected) in JEDISCT1_VECTORS {
        assert_eq!(padded_len(len), expected, "length {len}");
        assert_eq!(padding_len(len), expected - len, "padding length {len}");
    }
}

#[test]
fn edge_cases() {
    assert_eq!(padded_len(0), 0);
    assert_eq!(padding_len(0), 0);
    assert_eq!(padded_len(1), 1);
    assert_eq!(padding_len(1), 0);
    assert_eq!(padded_len(2), 2);
    assert_eq!(padding_len(2), 0);

    // Beyond the addressable range the caller has already exceeded every
    // supported payload limit. The function still returns a value >= len.
    assert_eq!(padded_len(usize::MAX), usize::MAX);
}

#[test]
fn padding_is_monotonic_and_idempotent() {
    let mut previous = 0;
    for len in 0..=100_000 {
        let padded = padded_len(len);
        assert!(padded >= len, "length {len} shrank to {padded}");
        assert!(padded >= previous, "length {len} went backwards");
        assert_eq!(padded_len(padded), padded, "length {len} is not stable");
        previous = padded;
    }
}

#[test]
fn overhead_is_bounded() {
    for len in 2..=1_000_000 {
        let padded = padded_len(len);
        assert!(padded <= len + len / 8 + 1, "length {len} padded to {padded}",);
    }
}
