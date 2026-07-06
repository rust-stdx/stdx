struct Gcm128Vector {
    key: &'static str,
    nonce: &'static str,
    pt: &'static str,
    aad: &'static str,
    ct: &'static str,
    tag: &'static str,
}

// NIST SP 800-38D test vectors (128-bit keys)
const NIST_GCM_128_VECTORS: &[Gcm128Vector] = &[
    Gcm128Vector {
        key: "00000000000000000000000000000000",
        nonce: "000000000000000000000000",
        pt: "",
        aad: "",
        ct: "",
        tag: "58e2fccefa7e3061367f1d57a4e7455a",
    },
    Gcm128Vector {
        key: "00000000000000000000000000000000",
        nonce: "000000000000000000000000",
        pt: "00000000000000000000000000000000",
        aad: "",
        ct: "0388dace60b6a392f328c2b971b2fe78",
        tag: "ab6e47d42cec13bdf53a67b21257bddf",
    },
    Gcm128Vector {
        key: "feffe9928665731c6d6a8f9467308308",
        nonce: "cafebabefacedbaddecaf888",
        pt: "d9313225f88406e5a55909c5aff5269a86a7a9531534f7da2e4c303d8a318a721c3c0c95956809532fcf0e2449a6b525b16aedf5aa0de657ba637b391aafd255",
        aad: "",
        ct: "42831ec2217774244b7221b784d0d49ce3aa212f2c02a4e035c17e2329aca12e21d514b25466931c7d8f6a5aac84aa051ba30b396a0aac973d58e091473f5985",
        tag: "4d5c2af327cd64a62cf35abd2ba6fab4",
    },
    Gcm128Vector {
        key: "feffe9928665731c6d6a8f9467308308",
        nonce: "cafebabefacedbaddecaf888",
        pt: "d9313225f88406e5a55909c5aff5269a86a7a9531534f7da2e4c303d8a318a721c3c0c95956809532fcf0e2449a6b525b16aedf5aa0de657ba637b39",
        aad: "feedfacedeadbeeffeedfacedeadbeefabaddad2",
        ct: "42831ec2217774244b7221b784d0d49ce3aa212f2c02a4e035c17e2329aca12e21d514b25466931c7d8f6a5aac84aa051ba30b396a0aac973d58e091",
        tag: "5bc94fbc3221a5db94fae95ae7121a47",
    },
];
