struct GcmVector {
    key: &'static str,
    nonce: &'static str,
    pt: &'static str,
    aad: &'static str,
    ct: &'static str,
    tag: &'static str,
}

const NIST_GCM_VECTORS: &[GcmVector] = &[
    // Test Case 13 – empty plaintext, empty AAD, 256-bit key
    GcmVector {
        key: "0000000000000000000000000000000000000000000000000000000000000000",
        nonce: "000000000000000000000000",
        pt: "",
        aad: "",
        ct: "",
        tag: "530f8afbc74536b9a963b4f1c4cb738b",
    },
    // Test Case 14 – plaintext = 16 zero bytes, empty AAD, 256-bit key
    GcmVector {
        key: "0000000000000000000000000000000000000000000000000000000000000000",
        nonce: "000000000000000000000000",
        pt: "00000000000000000000000000000000",
        aad: "",
        ct: "cea7403d4d606b6e074ec5d3baf39d18",
        tag: "d0d1c8a799996bf0265b98b5d48ab919",
    },
    // Test Case 15 – from NIST SP 800-38D
    GcmVector {
        key: "feffe9928665731c6d6a8f9467308308feffe9928665731c6d6a8f9467308308",
        nonce: "cafebabefacedbaddecaf888",
        pt: "d9313225f88406e5a55909c5aff5269a86a7a9531534f7da2e4c303d8a318a721c3c0c95956809532fcf0e2449a6b525b16aedf5aa0de657ba637b391aafd255",
        aad: "",
        ct: "522dc1f099567d07f47f37a32a84427d643a8cdcbfe5c0c97598a2bd2555d1aa8cb08e48590dbb3da7b08b1056828838c5f61e6393ba7a0abcc9f662898015ad",
        tag: "b094dac5d93471bdec1a502270e3cc6c",
    },
    // Test Case 16 – with AAD
    GcmVector {
        key: "feffe9928665731c6d6a8f9467308308feffe9928665731c6d6a8f9467308308",
        nonce: "cafebabefacedbaddecaf888",
        pt: "d9313225f88406e5a55909c5aff5269a86a7a9531534f7da2e4c303d8a318a721c3c0c95956809532fcf0e2449a6b525b16aedf5aa0de657ba637b39",
        aad: "feedfacedeadbeeffeedfacedeadbeefabaddad2",
        ct: "522dc1f099567d07f47f37a32a84427d643a8cdcbfe5c0c97598a2bd2555d1aa8cb08e48590dbb3da7b08b1056828838c5f61e6393ba7a0abcc9f662",
        tag: "76fc6ece0f4e1768cddf8853bb2d551b",
    },
];

const EXTRA_GCM_VECTORS: &[GcmVector] = &[
    // Source: pyca/cryptography (Count=0)
    GcmVector {
        key: "b52c505a37d78eda5dd34f20c22540ea1b58963cf8e5bf8ffa85f9f2492505b4",
        nonce: "516c33929df5a3284ff463d7",
        pt: "",
        aad: "",
        ct: "",
        tag: "bdc1ac884d332457a1d2664f168c76f0",
    },
    // Source: pyca/cryptography (Count=1)
    GcmVector {
        key: "5fe0861cdc2690ce69b3658c7f26f8458eec1c9243c5ba0845305d897e96ca0f",
        nonce: "770ac1a5a3d476d5d96944a1",
        pt: "",
        aad: "",
        ct: "",
        tag: "196d691e1047093ca4b3d2ef4baba216",
    },
    // Source: pyca/cryptography (Count=2)
    GcmVector {
        key: "7620b79b17b21b06d97019aa70e1ca105e1c03d2a0cf8b20b5a0ce5c3903e548",
        nonce: "60f56eb7a4b38d4f03395511",
        pt: "",
        aad: "",
        ct: "",
        tag: "f570c38202d94564bab39f75617bc87a",
    },
    // Source: pyca/cryptography (Count=3)
    GcmVector {
        key: "7e2db00321189476d144c5f27e787087302a48b5f7786cd91e93641628c2328b",
        nonce: "ea9d525bf01de7b2234b606a",
        pt: "",
        aad: "",
        ct: "",
        tag: "db9df5f14f6c9f2ae81fd421412ddbbb",
    },
    // Source: pyca/cryptography (Count=4)
    GcmVector {
        key: "a23dfb84b5976b46b1830d93bcf61941cae5e409e4f5551dc684bdcef9876480",
        nonce: "5aa345908048de10a2bd3d32",
        pt: "",
        aad: "",
        ct: "",
        tag: "f28217649230bd7a40a9a4ddabc67c43",
    },
    // Source: pyca/cryptography (Count=5)
    GcmVector {
        key: "dfe928f86430b78add7bb7696023e6153d76977e56103b180253490affb9431c",
        nonce: "1dd0785af9f58979a10bd62d",
        pt: "",
        aad: "",
        ct: "",
        tag: "a55eb09e9edef58d9f671d72207f8b3c",
    },
    // Source: pyca/cryptography (Count=6)
    GcmVector {
        key: "34048db81591ee68224956bd6989e1630fcf068d7ff726ae81e5b29f548cfcfb",
        nonce: "1621d34cff2a5b250c7b76fc",
        pt: "",
        aad: "",
        ct: "",
        tag: "4992ec3d57cccfa58fd8916c59b70b11",
    },
    // Source: pyca/cryptography (Count=7)
    GcmVector {
        key: "a1114f8749c72b8cef62e7503f1ad921d33eeede32b0b5b8e0d6807aa233d0ad",
        nonce: "a190ed3ff2e238be56f90bd6",
        pt: "",
        aad: "",
        ct: "",
        tag: "c8464d95d540fb191156fbbc1608842a",
    },
    // Source: pyca/cryptography (Count=8)
    GcmVector {
        key: "ddbb99dc3102d31102c0e14b238518605766c5b23d9bea52c7c5a771042c85a0",
        nonce: "95d15ed75c6a109aac1b1d86",
        pt: "",
        aad: "",
        ct: "",
        tag: "813d1da3775cacd78e96d86f036cff96",
    },
    // Source: pyca/cryptography (Count=9)
    GcmVector {
        key: "1faa506b8f13a2e6660af78d92915adf333658f748f4e48fa20135a29e9abe5f",
        nonce: "e50f278d3662c99d750f60d3",
        pt: "",
        aad: "",
        ct: "",
        tag: "aec7ece66b7344afd6f6cc7419cf6027",
    },
    // Source: pyca/cryptography (Count=10)
    GcmVector {
        key: "f30b5942faf57d4c13e7a82495aedf1b4e603539b2e1599317cc6e53225a2493",
        nonce: "336c388e18e6abf92bb739a9",
        pt: "",
        aad: "",
        ct: "",
        tag: "ddaf8ef4cb2f8a6d401f3be5ff0baf6a",
    },
    // Source: pyca/cryptography (Count=11)
    GcmVector {
        key: "daf4d9c12c5d29fc3fa936532c96196e56ae842e47063a4b29bfff2a35ed9280",
        nonce: "5381f21197e093b96cdac4fa",
        pt: "",
        aad: "",
        ct: "",
        tag: "7f1832c7f7cd7812a004b79c3d399473",
    },
    // Source: pyca/cryptography (Count=12)
    GcmVector {
        key: "6b524754149c81401d29a4b8a6f4a47833372806b2d4083ff17f2db3bfc17bca",
        nonce: "ac7d3d618ab690555ec24408",
        pt: "",
        aad: "",
        ct: "",
        tag: "db07a885e2bd39da74116d06c316a5c9",
    },
    // Source: pyca/cryptography (Count=13)
    GcmVector {
        key: "cff083303ff40a1f66c4aed1ac7f50628fe7e9311f5d037ebf49f4a4b9f0223f",
        nonce: "45d46e1baadcfbc8f0e922ff",
        pt: "",
        aad: "",
        ct: "",
        tag: "1687c6d459ea481bf88e4b2263227906",
    },
    // Source: pyca/cryptography (Count=14)
    GcmVector {
        key: "3954f60cddbb39d2d8b058adf545d5b82490c8ae9283afa5278689041d415a3a",
        nonce: "8fb3d98ef24fba03746ac84f",
        pt: "",
        aad: "",
        ct: "",
        tag: "7fb130855dfe7a373313361f33f55237",
    },
    // Source: pyca/cryptography (Count=1)
    GcmVector {
        key: "4457ff33683cca6ca493878bdc00373893a9763412eef8cddb54f91318e0da88",
        nonce: "699d1f29d7b8c55300bb1fd2",
        pt: "",
        aad: "6749daeea367d0e9809e2dc2f309e6e3",
        ct: "",
        tag: "d60c74d2517fde4a74e0cd4709ed43a9",
    },
    // Source: pyca/cryptography (Count=2)
    GcmVector {
        key: "4d01c96ef9d98d4fb4e9b61be5efa772c9788545b3eac39eb1cacb997a5f0792",
        nonce: "32124a4d9e576aea2589f238",
        pt: "",
        aad: "d72bad0c38495eda50d55811945ee205",
        ct: "",
        tag: "6d6397c9e2030f5b8053bfe510f3f2cf",
    },
    // Source: pyca/cryptography (Count=3)
    GcmVector {
        key: "8378193a4ce64180814bd60591d1054a04dbc4da02afde453799cd6888ee0c6c",
        nonce: "bd8b4e352c7f69878a475435",
        pt: "",
        aad: "1c6b343c4d045cbba562bae3e5ff1b18",
        ct: "",
        tag: "0833967a6a53ba24e75c0372a6a17bda",
    },
    // Source: pyca/cryptography (Count=4)
    GcmVector {
        key: "22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12",
        nonce: "880d05c5ee599e5f151e302f",
        pt: "",
        aad: "3e3eb5747e390f7bc80e748233484ffc",
        ct: "",
        tag: "2e122a478e64463286f8b489dcdd09c8",
    },
    // Source: pyca/cryptography (Count=5)
    GcmVector {
        key: "fc00960ddd698d35728c5ac607596b51b3f89741d14c25b8badac91976120d99",
        nonce: "a424a32a237f0df530f05e30",
        pt: "",
        aad: "cfb7e05e3157f0c90549d5c786506311",
        ct: "",
        tag: "dcdcb9e4004b852a0da12bdf255b4ddd",
    },
    // Source: pyca/cryptography (Count=6)
    GcmVector {
        key: "69749943092f5605bf971e185c191c618261b2c7cc1693cda1080ca2fd8d5111",
        nonce: "bd0d62c02ee682069bd1e128",
        pt: "",
        aad: "6967dce878f03b643bf5cdba596a7af3",
        ct: "",
        tag: "378f796ae543e1b29115cc18acd193f4",
    },
    // Source: pyca/cryptography (Count=7)
    GcmVector {
        key: "fc4875db84819834b1cb43828d2f0ae3473aa380111c2737e82a9ab11fea1f19",
        nonce: "da6a684d3ff63a2d109decd6",
        pt: "",
        aad: "91b6fa2ab4de44282ffc86c8cde6e7f5",
        ct: "",
        tag: "504e81d2e7877e4dad6f31cdeb07bdbd",
    },
    // Source: pyca/cryptography (Count=8)
    GcmVector {
        key: "9f9fe7d2a26dcf59d684f1c0945b5ffafe0a4746845ed317d35f3ed76c93044d",
        nonce: "13b59971cd4dd36b19ac7104",
        pt: "",
        aad: "190a6934f45f89c90067c2f62e04c53b",
        ct: "",
        tag: "4f636a294bfbf51fc0e131d694d5c222",
    },
    // Source: pyca/cryptography (Count=9)
    GcmVector {
        key: "ab9155d7d81ba6f33193695cf4566a9b6e97a3e409f57159ae6ca49655cca071",
        nonce: "26a9f8d665d163ddb92d035d",
        pt: "",
        aad: "4a203ac26b951a1f673c6605653ec02d",
        ct: "",
        tag: "437ea77a3879f010691e288d6269a996",
    },
    // Source: pyca/cryptography (Count=10)
    GcmVector {
        key: "0f1c62dd80b4a6d09ee9d787b1b04327aa361529ffa3407560414ac47b7ef7bc",
        nonce: "c87613a3b70d2a048f32cb9a",
        pt: "",
        aad: "8f23d404be2d9e888d219f1b40aa29e8",
        ct: "",
        tag: "36d8a309acbb8716c9c08c7f5de4911e",
    },
    // Source: pyca/cryptography (Count=11)
    GcmVector {
        key: "f3e954a38956df890255f01709e457b33f4bfe7ecb36d0ee50f2500471eebcde",
        nonce: "9799abd3c52110c704b0f36a",
        pt: "",
        aad: "ddb70173f44157755b6c9b7058f40cb7",
        ct: "",
        tag: "b323ae3abcb415c7f420876c980f4858",
    },
    // Source: pyca/cryptography (Count=12)
    GcmVector {
        key: "0625316534fbd82fe8fdea50fa573c462022c42f79e8b21360e5a6dce66dde28",
        nonce: "da64a674907cd6cf248f5fbb",
        pt: "",
        aad: "f24d48e04f5a0d987ba7c745b73b0364",
        ct: "",
        tag: "df360b810f27e794673a8bb2dc0d68b0",
    },
    // Source: pyca/cryptography (Count=13)
    GcmVector {
        key: "28f045ac7c4fe5d4b01a9dcd5f1ad3efff1c4f170fc8ab8758d97292868d5828",
        nonce: "5d85de95b0bdc44514143919",
        pt: "",
        aad: "601d2158f17ab3c7b4dcb6950fbdcdde",
        ct: "",
        tag: "42c3f527418cf2c3f5d5010ccba8f271",
    },
    // Source: pyca/cryptography (Count=14)
    GcmVector {
        key: "19310eed5f5f44eb47075c105eb31e36bbfd1310f741b9baa66a81138d357242",
        nonce: "a1247120138fa4f0e96c992c",
        pt: "",
        aad: "29d746414333e0f72b4c3f44ec6bfe42",
        ct: "",
        tag: "d5997e2f956df3fa2c2388e20f30c480",
    },
    // Source: pyca/cryptography (Count=0)
    GcmVector {
        key: "886cff5f3e6b8d0e1ad0a38fcdb26de97e8acbe79f6bed66959a598fa5047d65",
        nonce: "3a8efa1cd74bbab5448f9945",
        pt: "",
        aad: "519fee519d25c7a304d6c6aa1897ee1eb8c59655",
        ct: "",
        tag: "f6d47505ec96c98a42dc3ae719877b87",
    },
    // Source: pyca/cryptography (Count=1)
    GcmVector {
        key: "6937a57d35fe6dc3fc420b123bccdce874bd4c18f2e7c01ce2faf33d3944fd9d",
        nonce: "a87247797b758467b96310f3",
        pt: "",
        aad: "ead961939a33dd578f8e93db8b28a1c85362905f",
        ct: "",
        tag: "599de3ecf22cb867f03f7f6d9fd7428a",
    },
    // Source: pyca/cryptography (Count=2)
    GcmVector {
        key: "e65a331776c9dcdf5eba6c59e05ec079d97473bcdce84daf836be323456263a0",
        nonce: "ca731f768da01d02eb8e727e",
        pt: "",
        aad: "d7274586517bf1d8da866f4a47ad0bcf2948a862",
        ct: "",
        tag: "a8abe7a8085f25130a7206d37a8aaf6d",
    },
    // Source: pyca/cryptography (Count=3)
    GcmVector {
        key: "77bb1b6ef898683c981b2fc899319ffbb6000edca22566b634db3a3c804059e5",
        nonce: "354a19283769b3b991b05a4c",
        pt: "",
        aad: "b5566251a8a8bec212dc08113229ff8590168800",
        ct: "",
        tag: "e5c2dccf8fc7f296cac95d7071cb8d7d",
    },
    // Source: pyca/cryptography (Count=4)
    GcmVector {
        key: "2a43308d520a59ed51e47a3a915e1dbf20a91f0886506e481ad3de65d50975b4",
        nonce: "bcbf99733d8ec90cb23e6ce6",
        pt: "",
        aad: "eb88288729289d26fe0e757a99ad8eec96106053",
        ct: "",
        tag: "01b0196933aa49123eab4e1571250383",
    },
    // Source: pyca/cryptography (Count=5)
    GcmVector {
        key: "2379b35f85102db4e7aecc52b705bc695d4768d412e2d7bebe999236783972ff",
        nonce: "918998c4801037b1cd102faa",
        pt: "",
        aad: "b3722309e0f066225e8d1659084ebb07a93b435d",
        ct: "",
        tag: "dfb18aee99d1f67f5748d4b4843cb649",
    },
    // Source: pyca/cryptography (Count=6)
    GcmVector {
        key: "98b3cb7537167e6d14a2a8b2310fe94b715c729fdf85216568150b556d0797ba",
        nonce: "bca5e2e5a6b30f18d263c6b2",
        pt: "",
        aad: "260d3d72db70d677a4e3e1f3e11431217a2e4713",
        ct: "",
        tag: "d6b7560f8ac2f0a90bad42a6a07204bc",
    },
    // Source: pyca/cryptography (Count=7)
    GcmVector {
        key: "30341ae0f199b10a15175d00913d5029526ab7f761c0b936a7dd5f1b1583429d",
        nonce: "dbe109a8ce5f7b241e99f7af",
        pt: "",
        aad: "fe4bdee5ca9c4806fa024715fbf66ab845285fa7",
        ct: "",
        tag: "ae91daed658e26c0d126575147af9899",
    },
    // Source: pyca/cryptography (Count=8)
    GcmVector {
        key: "8232b6a1d2e367e9ce1ea8d42fcfc83a4bc8bdec465c6ba326e353ad9255f207",
        nonce: "cd2fb5ff9cf0f39868ad8685",
        pt: "",
        aad: "02418b3dde54924a9628de06004c0882ae4ec3bb",
        ct: "",
        tag: "d5308f63708675ced19b2710afd2db49",
    },
    // Source: pyca/cryptography (Count=9)
    GcmVector {
        key: "f9a132a50a508145ffd8294e68944ea436ce0f9a97e181f5e0d6c5d272311fc1",
        nonce: "892991b54e94b9d57442ccaf",
        pt: "",
        aad: "4e0fbd3799da250fa27911b7e68d7623bfe60a53",
        ct: "",
        tag: "89881d5f786e6d53e0d19c3b4e6887d8",
    },
    // Source: pyca/cryptography (Count=10)
    GcmVector {
        key: "0e3746e5064633ea9311b2b8427c536af92717de20eeb6260db1333c3d8a8114",
        nonce: "f84c3a1c94533f7f25cec0ac",
        pt: "",
        aad: "8c0d41e6135338c8d3e63e2a5fa0a9667ec9a580",
        ct: "",
        tag: "479ccfe9241de2c474f2edebbb385c09",
    },
    // Source: pyca/cryptography (Count=11)
    GcmVector {
        key: "b997e9b0746abaaed6e64b63bdf64882526ad92e24a2f5649df055c9ec0f1daa",
        nonce: "f141d8d71b033755022f0a7d",
        pt: "",
        aad: "681d6583f527b1a92f66caae9b1d4d028e2e631e",
        ct: "",
        tag: "b30442a6395ec13246c48b21ffc65509",
    },
    // Source: pyca/cryptography (Count=12)
    GcmVector {
        key: "87660ec1700d4e9f88a323a49f0b871e6aaf434a2d8448d04d4a22f6561028e0",
        nonce: "2a07b42593cd24f0a6fe406c",
        pt: "",
        aad: "1dd239b57185b7e457ced73ebba043057f049edd",
        ct: "",
        tag: "df7a501049b37a534098cb45cb9c21b7",
    },
    // Source: pyca/cryptography (Count=13)
    GcmVector {
        key: "ea4792e1f1717b77a00de4d109e627549b165c82af35f33ca7e1a6b8ed62f14f",
        nonce: "7453cc8b46fe4b93bcc48381",
        pt: "",
        aad: "46d98970a636e7cd7b76fc362ae88298436f834f",
        ct: "",
        tag: "518dbacd36be6fba5c12871678a55516",
    },
    // Source: pyca/cryptography (Count=14)
    GcmVector {
        key: "34892cdd1d48ca166f7ba73182cb97336c2c754ac160a3e37183d6fb5078cec3",
        nonce: "ed3198c5861b78c71a6a4eec",
        pt: "",
        aad: "a6fa6d0dd1e0b95b4609951bbbe714de0ae0ccfa",
        ct: "",
        tag: "c6387795096b348ecf1d1f6caaa3c813",
    },
    // Source: pyca/cryptography (Count=0)
    GcmVector {
        key: "f4069bb739d07d0cafdcbc609ca01597f985c43db63bbaaa0debbb04d384e49c",
        nonce: "d25ff30fdc3d464fe173e805",
        pt: "",
        aad: "3e1449c4837f0892f9d55127c75c4b25d69be334baf5f19394d2d8bb460cbf2120e14736d0f634aa792feca20e455f11",
        ct: "",
        tag: "805ec2931c2181e5bfb74fa0a975f0cf",
    },
    // Source: pyca/cryptography (Count=1)
    GcmVector {
        key: "62189dcc4beb97462d6c0927d8a270d39a1b07d72d0ad28840badd4f68cf9c8b",
        nonce: "859fda5247c888823a4b8032",
        pt: "",
        aad: "b28d1621ee110f4c9d709fad764bba2dd6d291bc003748faac6d901937120d41c1b7ce67633763e99e05c71363fceca8",
        ct: "",
        tag: "27330907d0002880bbb4c1a1d23c0be2",
    },
    // Source: pyca/cryptography (Count=2)
    GcmVector {
        key: "59012d85a1b90aeb0359e6384c9991e7be219319f5b891c92c384ade2f371816",
        nonce: "3c9cde00c23912cff9689c7c",
        pt: "",
        aad: "e5daf473a470860b55210a483c0d1a978d8add843c2c097f73a3cda49ac4a614c8e887d94e6692309d2ed97ebe1eaf5d",
        ct: "",
        tag: "048239e4e5c2c8b33890a7c950cda852",
    },
    // Source: pyca/cryptography (Count=3)
    GcmVector {
        key: "4be09b408ad68b890f94be5efa7fe9c917362712a3480c57cd3844935f35acb7",
        nonce: "8f350bd3b8eea173fc7370bc",
        pt: "",
        aad: "2819d65aec942198ca97d4435efd9dd4d4393b96cf5ba44f09bce4ba135fc8636e8275dcb515414b8befd32f91fc4822",
        ct: "",
        tag: "a133cb7a7d0471dbac61fb41589a2efe",
    },
    // Source: pyca/cryptography (Count=4)
    GcmVector {
        key: "13cb965a4d9d1a36efad9f6ca1ba76386a5bb160d80b0917277102357ac7afc8",
        nonce: "f313adec42a66d13c3958180",
        pt: "",
        aad: "717b48358898e5ccfea4289049adcc1bb0db3b3ebd1767ac24fb2b7d37dc80ea2316c17f14fb51b5e18cd5bb09afe414",
        ct: "",
        tag: "81b4ef7a84dc4a0b1fddbefe37f53852",
    },
    // Source: pyca/cryptography (Count=5)
    GcmVector {
        key: "d27f1bebbbdef0edca393a6261b0338abbc491262eab0737f55246458f6668cc",
        nonce: "fc062f857886e278f3a567d2",
        pt: "",
        aad: "2bae92dea64aa99189de8ea4c046745306002e02cfb46a41444ce8bfcc329bd4205963d9ab5357b026a4a34b1a861771",
        ct: "",
        tag: "5c5a6c4613f1e522596330d45f243fdd",
    },
    // Source: pyca/cryptography (Count=6)
    GcmVector {
        key: "7b4d19cd3569f74c7b5df61ab78379ee6bfa15105d21b10bf6096699539006d0",
        nonce: "fbed5695c4a739eded97b1e3",
        pt: "",
        aad: "c6f2e5d663bfaf668d014550ef2e66bf89978799a785f1f2c79a2cb3eb3f2fd4076207d5f7e1c284b4af5cffc4e46198",
        ct: "",
        tag: "7101b434fb90c7f95b9b7a0deeeb5c81",
    },
    // Source: pyca/cryptography (Count=7)
    GcmVector {
        key: "d3431488d8f048590bd76ec66e71421ef09f655d7cf8043bf32f75b4b2e7efcc",
        nonce: "cc766e98b40a81519fa46392",
        pt: "",
        aad: "93320179fdb40cbc1ccf00b872a3b4a5f6c70b56e43a84fcac5eb454a0a19a747d452042611bf3bbaafd925e806ffe8e",
        ct: "",
        tag: "3afcc336ce8b7191eab04ad679163c2a",
    },
    // Source: pyca/cryptography (Count=8)
    GcmVector {
        key: "a440948c0378561c3956813c031f81573208c7ffa815114ef2eee1eb642e74c6",
        nonce: "c1f4ffe54b8680832eed8819",
        pt: "",
        aad: "253438f132b18e8483074561898c5652b43a82cc941e8b4ae37e792a8ed6ec5ce2bcec9f1ffcf4216e46696307bb774a",
        ct: "",
        tag: "129445f0a3c979a112a3afb10a24e245",
    },
    // Source: pyca/cryptography (Count=9)
    GcmVector {
        key: "798706b651033d9e9bf2ce064fb12be7df7308cf45df44776588cd391c49ff85",
        nonce: "5a43368a39e7ffb775edfaf4",
        pt: "",
        aad: "926b74fe6381ebd35757e42e8e557601f2287bfc133a13fd86d61c01aa84f39713bf99a8dc07b812f0274c9d3280a138",
        ct: "",
        tag: "89fe481a3d95c03a0a9d4ee3e3f0ed4a",
    },
    // Source: pyca/cryptography (Count=10)
    GcmVector {
        key: "c3aa2a39a9fef4a466618d1288bb62f8da7b1cb760ccc8f1be3e99e076f08eff",
        nonce: "9965ba5e23d9453d7267ca5b",
        pt: "",
        aad: "93efb6a2affc304cb25dfd49aa3e3ccdb25ceac3d3cea90dd99e38976978217ad5f2b990d10b91725c7fd2035ecc6a30",
        ct: "",
        tag: "00a94c18a4572dcf4f9e2226a03d4c07",
    },
    // Source: pyca/cryptography (Count=11)
    GcmVector {
        key: "14e06858008f7e77186a2b3a7928a0c7fcee22136bc36f53553f20fa5c37edcd",
        nonce: "32ebe0dc9ada849b5eda7b48",
        pt: "",
        aad: "6c0152abfa485b8cd67c154a5f0411f22121379774d745f40ee577b028fd0e188297581561ae972223d75a24b488aed7",
        ct: "",
        tag: "2625b0ba6ee02b58bc529e43e2eb471b",
    },
    // Source: pyca/cryptography (Count=12)
    GcmVector {
        key: "fbb56b11c51a093ce169a6990399c4d741f62b3cc61f9e8a609a1b6ae8e7e965",
        nonce: "9c5a953247e91aceceb9defb",
        pt: "",
        aad: "46cb5c4f617916a9b1b2e03272cb0590ce716498533047d73c81e4cbe9278a3686116f5632753ea2df52efb3551aea2d",
        ct: "",
        tag: "4f3b82e6be4f08756071f2c46c31fedf",
    },
    // Source: pyca/cryptography (Count=13)
    GcmVector {
        key: "b303bf02f6a8dbb5bc4baccab0800db5ee06de648e2fae299b95f135c9b107cc",
        nonce: "906495b67ef4ce00b44422fa",
        pt: "",
        aad: "872c6c370926535c3fa1baec031e31e7c6c82808c8a060742dbef114961c314f1986b2131a9d91f30f53067ec012c6b7",
        ct: "",
        tag: "64dde37169082d181a69107f60c5c6bb",
    },
    // Source: pyca/cryptography (Count=14)
    GcmVector {
        key: "29f5f8075903063cb6d7050669b1f74e08a3f79ef566292dfdef1c06a408e1ab",
        nonce: "35f25c48b4b5355e78b9fb3a",
        pt: "",
        aad: "107e2e23159fc5c0748ca7a077e5cc053fa5c682ff5269d350ee817f8b5de4d3972041d107b1e2f2e54ca93b72cd0408",
        ct: "",
        tag: "fee5a9baebb5be0165deaa867e967a9e",
    },
    // Source: pyca/cryptography (Count=0)
    GcmVector {
        key: "03ccb7dbc7b8425465c2c3fc39ed0593929ffd02a45ff583bd89b79c6f646fe9",
        nonce: "fd119985533bd5520b301d12",
        pt: "",
        aad: "98e68c10bf4b5ae62d434928fc6405147c6301417303ef3a703dcfd2c0c339a4d0a89bd29fe61fecf1066ab06d7a5c31a48ffbfed22f749b17e9bd0dc1c6f8fbd6fd4587184db964d5456132106d782338c3f117ec05229b0899",
        ct: "",
        tag: "cf54e7141349b66f248154427810c87a",
    },
    // Source: pyca/cryptography (Count=1)
    GcmVector {
        key: "57e112cd45f2c57ddb819ea651c206763163ef016ceead5c4eae40f2bbe0e4b4",
        nonce: "188022c2125d2b1fcf9e4769",
        pt: "",
        aad: "09c8f445ce5b71465695f838c4bb2b00624a1c9185a3d552546d9d2ee4870007aaf3007008f8ae9affb7588b88d09a90e58b457f88f1e3752e3fb949ce378670b67a95f8cf7f5c7ceb650efd735dbc652cae06e546a5dbd861bd",
        ct: "",
        tag: "9efcddfa0be21582a05749f4050d29fe",
    },
    // Source: pyca/cryptography (Count=2)
    GcmVector {
        key: "a4ddf3cab7453aaefad616fd65d63d13005e9459c17d3173cd6ed7f2a86c921f",
        nonce: "06177b24c58f3be4f3dd4920",
        pt: "",
        aad: "f95b046d80485e411c56b834209d3abd5a8a9ddf72b1b916679adfdde893044315a5f4967fd0405ec297aa332f676ff0fa5bd795eb609b2e4f088db1cdf37ccff0735a5e53c4c12173a0026aea42388a7d7153a8830b8a901cf9",
        ct: "",
        tag: "9d1bd8ecb3276906138d0b03fcb8c1bb",
    },
    // Source: pyca/cryptography (Count=3)
    GcmVector {
        key: "24a92b24e85903cd4aaabfe07c310df5a4f8f459e03a63cbd1b47855b09c0be8",
        nonce: "22e756dc898d4cf122080612",
        pt: "",
        aad: "2e01b2536dbe376be144296f5c38fb099e008f962b9f0e896334b6408393bff1020a0e442477abfdb1727213b6ccc577f5e16cb057c8945a07e307264b65979aed96b5995f40250ffbaaa1a1f0eccf394015f6290f5e64dfe5ca",
        ct: "",
        tag: "0d7f1aed4708a03b0c80b2a18785c96d",
    },
    // ── Additional vectors generated and verified with pyca/cryptography (OpenSSL backend) ──
    // Source: pyca/cryptography verified – BoringSSL style short plaintext
    GcmVector {
        key: "e5ac4a32c67e425bae6c19d2e25f0e8a3d0de5502cc3b1ee4ef83f16e75f2e29",
        nonce: "5bf11a0951f0bfc81e2a0750",
        pt: "54657374696e672031203220332e",
        aad: "",
        ct: "13f9e79f7c0627be185474023c2c",
        tag: "92a13cc6519f9ee5dc0c7e11b6e29490",
    },
    // Source: pyca/cryptography verified – 256-byte zero plaintext
    GcmVector {
        key: "4c8ebfe1444ec1b2d503c6986659af2c94fafe945f0ec25f043bde6ef0fa8f0a",
        nonce: "473360e0ad24889959858282",
        pt: "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        aad: "",
        ct: "4e557bbaa946e00f5d152e5cb33aef3a06270dcc4d97bfc33a7857414140b316b6a99b0490b5bc02a2de20c7ddc02a2bc7a6a0591d2e2ff56fda74780dbc8745154314e9484dd8683b9ce893081c7306a45db1e8ca5f02bd688bb182a3e293362efca92c85c8b7cfe83cc2f70084877dbcecf57469c6fe2c990722355fab1f43530c465424ed1bf7f03b93c403b005a40ea9238f4c2c816d1d9af48ddad9c7d1e8abf11d00f38110867c720fdddcfa0769cc39daf81178c82ab960c3309200610a42e5219c920e8a5c0b30c360218e80f7418fa4a9e556f199da87636616bf27b484a4bef1db582a0a11a5f20a9b5ecdce3e703f98086b13e453cc16f3d87e51",
        tag: "b2c3a2b04f167587e821f50efce64b0c",
    },
    // Source: pyca/cryptography verified – Large AAD (128 bytes) with 32-byte plaintext
    GcmVector {
        key: "2f1e0a2fc2355b7e18b6adab7ccbe8a27e3e3faab3f06f26d3e1fe2e69ab3bff",
        nonce: "a91cd374a3a4362fd1f30e2f",
        pt: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        aad: "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        ct: "4009a0f359798995faa7f3610d7fab81a097fc171619940082b381453bc512dc",
        tag: "10add57efc77a4bcb30e94a82dbe8c1a",
    },
    // Source: pyca/cryptography verified – Single byte plaintext with AAD
    GcmVector {
        key: "3881e7be1bb3bbcbcda0f07b7a4471e6ae18e5f0e93a6cb14647c2de85ffbeef",
        nonce: "dcf5b7ae2f0f4d2684f2baae",
        pt: "42",
        aad: "deadbeef",
        ct: "d5",
        tag: "b32d586e094a7e2fa2bb86d2df7adc02",
    },
    // Source: pyca/cryptography verified – 15-byte plaintext (sub-block)
    GcmVector {
        key: "a4bc10b1a62c96d459fbaf3a5aa3face7313bb9e1253e696f96a7a8e36801088",
        nonce: "a544218dadd3c10583db49cf",
        pt: "00112233445566778899aabbccddee",
        aad: "",
        ct: "315167cf54ba0d2d37270b4b93cd66",
        tag: "b5e0791ac0672c38a845503137f797e1",
    },
    // Source: pyca/cryptography verified – 17-byte plaintext (just over one block) with AAD
    GcmVector {
        key: "8395fcf1e95bebd697bd010bc766aac3af96bdf3e4f916e6f67e5561bb8e9f96",
        nonce: "0e8d0b2d38e0f8d7a68ce2ad",
        pt: "00112233445566778899aabbccddeeff11",
        aad: "4142434445464748",
        ct: "3a810ec7d35ff484224c7327724fde4393",
        tag: "1472036169976e5a36ff68daa3078d3e",
    },
    // Source: pyca/cryptography verified – MAC-only with 128-byte AAD
    GcmVector {
        key: "014c9214862a18d2b46384e25c1a1c0030476defd1ea39d29a1cee84eca15b05",
        nonce: "0b7fdbfb25e1bc9daae8a28c",
        pt: "",
        aad: "aabbccddeeff0011aabbccddeeff0011aabbccddeeff0011aabbccddeeff0011aabbccddeeff0011aabbccddeeff0011aabbccddeeff0011aabbccddeeff0011aabbccddeeff0011aabbccddeeff0011aabbccddeeff0011aabbccddeeff0011aabbccddeeff0011aabbccddeeff0011aabbccddeeff0011aabbccddeeff0011",
        ct: "",
        tag: "4b41034df2152a1b22f3d68dee395e24",
    },
    // Source: pyca/cryptography verified – Multi-block (64-byte PT, 32-byte AAD)
    GcmVector {
        key: "b52c505a37d78eda5dd34f20c22540ea1b58963cf8e5bf8ffa85f9f2492505b4",
        nonce: "516c33929df5a3284ff463d7",
        pt: "00010203040506070001020304050607000102030405060700010203040506070001020304050607000102030405060700010203040506070001020304050607",
        aad: "0001020304050607000102030405060700010203040506070001020304050607",
        ct: "e69c8e689b8eb6386bfdcb2509d4c21797fec618d7c56b4ab1cf8c37e1db437a3f21a04732b8a2a6e5d1f2985fa06f9e848c3f5f115142580adf65e57a6c863f",
        tag: "1fa8e4f994d15d7cfff7315b487c6713",
    },
    // Source: pyca/cryptography verified – Incrementing pattern (NIST CAVP style)
    GcmVector {
        key: "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
        nonce: "000102030405060708090a0b",
        pt: "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
        aad: "000102030405060708090a0b0c0d0e0f",
        ct: "4703d418c1e0c41c85489d80bde4766293c79527e46e496b207eff9e01741ead",
        tag: "209d59e08347d37153a593a1fca88881",
    },
    // Source: pyca/cryptography verified – All-ones key, plaintext, and AAD
    GcmVector {
        key: "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        nonce: "ffffffffffffffffffffffff",
        pt: "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        aad: "ffffffffffffffffffffffffffffffff",
        ct: "42c4417ae76f276beb09973a4b9b3715e6a0fc4d53cb94e7338af461b0837bc1",
        tag: "6f1e9ac49421d9ca6e6720b4d22e0cbe",
    },
    // Source: pyca/cryptography verified – Alternating bit patterns
    GcmVector {
        key: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        nonce: "555555555555555555555555",
        pt: "aa55aa55aa55aa55aa55aa55aa55aa55aa55aa55aa55aa55aa55aa55aa55aa55",
        aad: "55aa55aa55aa55aa55aa55aa55aa55aa",
        ct: "629139940711d698f7e956430f425b1f51b3f03ef647a1d1479b7cb552616acd",
        tag: "9ead0bc82db5a46ef03ac3195ab1de05",
    },
    // Source: pyca/cryptography verified – 3-block (48-byte) plaintext
    GcmVector {
        key: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        nonce: "fedcba9876543210fedc0123",
        pt: "aabbccddeeff00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff00112233445566778899",
        aad: "",
        ct: "de7384a1c9f8fe4fa022a7ff71525ec36723d5f17b0f06e7d656911c9c39c457d1429ffd70d17430f029846ad6639781",
        tag: "7bf9dce9edcf955f0cd751bd46cdb943",
    },
    // Source: pyca/cryptography verified – 5-block (80-byte) PT with 3-block (48-byte) AAD
    GcmVector {
        key: "deadbeefcafebabedeadbeefcafebabedeadbeefcafebabedeadbeefcafebabe",
        nonce: "0bad0bad0bad0bad0bad0bad",
        pt: "48656c6c6f20576f726c642148656c6c6f20576f726c642148656c6c6f20576f726c642148656c6c6f20576f726c642148656c6c6f20576f726c642148656c6c6f20576f726c642148656c6c6f20576f",
        aad: "416464206d65746164617461416464206d65746164617461416464206d65746164617461416464206d65746164617461",
        ct: "7caf8fe2c0f2ee0fd0282c977620fa0c65cb448dae9ed89922a4dfbe387a26489265b9b8c2258f19dd2d4b29c13aa19be755220e3f0ff57dc2ac49fd1f8f1f4500fcae6fa6c102e23c1822013ff695b9",
        tag: "804e1175cb90bef1bb3ac867e1f104d3",
    },
    // Source: pyca/cryptography verified – 2-block PT with 1-byte AAD
    GcmVector {
        key: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        nonce: "bbbbbbbbbbbbbbbbbbbbbbbb",
        pt: "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
        aad: "dd",
        ct: "06dd876abad19614fc58615121a916d6794c3bf745fca14124b777efbc12e163",
        tag: "f537f82fe5c633b0bcbb74a47857ab69",
    },
    // Source: pyca/cryptography verified – 31-byte PT with 20-byte AAD
    GcmVector {
        key: "1111111111111111111111111111111122222222222222222222222222222222",
        nonce: "333333333333333333333333",
        pt: "44444444444444444444444444444444444444444444444444444444444444",
        aad: "5555555555555555555555555555555555555555",
        ct: "6d5fe7e7516f31d40a33a1c254ca67b5e95bbe82f28cc454c689b4e5efab79",
        tag: "06ac4b2c8b5bca4823c61b28bc2c27fc",
    },
    // ── BoringSSL test vectors (google/boringssl crypto/cipher/test/aes_256_gcm_tests.txt) ──
    // Source: BoringSSL – empty PT, empty AAD
    GcmVector {
        key: "e5ac4a32c67e425ac4b143c83c6f161312a97d88d634afdf9f4da5bd35223f01",
        nonce: "5bf11a0951f0bfc7ea5c9e58",
        pt: "",
        aad: "",
        ct: "",
        tag: "d7cba289d6d19a5af45dc13857016bac",
    },
    // Source: BoringSSL – 5-byte PT + 5-byte AAD
    GcmVector {
        key: "73ad7bbbbc640c845a150f67d058b279849370cd2c1f3c67c4dd6c869213e13a",
        nonce: "a330a184fc245812f4820caa",
        pt: "f0535fe211",
        aad: "e91428be04",
        ct: "e9b8a896da",
        tag: "9115ed79f26a030c14947b3e454db9e7",
    },
    // Source: BoringSSL – 15-byte PT + 15-byte AAD
    GcmVector {
        key: "881cca012ef9d6f1241b88e4364084d8c95470c6022e59b62732a1afcc02e657",
        nonce: "172ec639be736062bba5c32f",
        pt: "8ed8ef4c09360ef70bb22c716554ef",
        aad: "98c115f2c3bbe22e3a0c562e8e67ff",
        ct: "06a761987a7eb0e57a31979043747d",
        tag: "cf07239b9d40a759e0f4f8ef088f016a",
    },
    // Source: BoringSSL – 30-byte PT + 30-byte AAD
    GcmVector {
        key: "525429d45a66b9d860c83860111cc65324ab91ff77938bbc30a654220bb3e526",
        nonce: "31535d82b9b46f5ad75a1629",
        pt: "677eca74660499acf2e2fd6c7800fd6da2d0273a31906a691205b5765b85",
        aad: "513bc218acee89848e73ab108401bfc4f9c2aa70310a4e543644c37dd2f3",
        ct: "f1e6032ee3ce224b2e8f17f91055c81a480398e07fd9366ad69d84dca712",
        tag: "e39da5658f1d2994a529646d692c55d8",
    },
];
