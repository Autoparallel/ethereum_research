// You need to define Poseidon and OptimizedPoseidon structs and their methods

pub struct Poseidon {
    // fields
}

pub fn case_simple() -> (Poseidon, i32) {
    let security_level = 128;
    let input_rate = 8;
    let t = 9;
    let full_round = 8;
    let partial_round = 41;
    let alpha = 3;
    let poseidon = Poseidon::new(/* parameters */);
    (poseidon, t)
}

pub fn case_neptune() -> (OptimizedPoseidon, i32) {
    let security_level = 128;
    let input_rate = 3;
    let t = 4;
    let full_round = 8;
    let partial_round = 56;
    let alpha = 5;
    let poseidon = OptimizedPoseidon::new(/* parameters */);
    (poseidon, t)
}

const PRIME_64: u64 = 0xfffffffffffffeff;
const PRIME_254: u64 = 0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001;
const PRIME_255: u64 = 0x73EDA753299D7D483339D80809A1D80553BDA402FFFE5BFEFFFFFFFF00000001;

// Define round_constants_64 and matrix_neptune here
const round_constants_64: Vec<&'static str> = vec!["0xa874f1651194bf2", "0x8a33ea4b1d922d96", "0x11badd07168d87d1", "0xefc1185fe235753a",
                      "0x75711475cef37a6b", "0xfe2018febde04894", "0xcd5093e1c9d8c8ff", "0xd0b64f6fbd9e75c",
                      "0xbc6efcefe1fbd9e0", "0xeb0954505ee0940f", "0x68fa4d1c34c293a1", "0x4f1e19dcf109ce22",
                      "0xcec0792c731e87eb", "0x66ada74aa0b6b8ae", "0xedf1a0e8a9867747", "0xdcedd33a20b2fef2",
                      "0x2f0dc0ab74e5b75", "0xcafe1f18d4aa6d68", "0x33db89298f49a93f", "0xe05259d7010dcc26",
                      "0xb8b6fc9a3a7cade9", "0xf5d56d0a51405ec1", "0xc16bfb1759238f40", "0x75844914394afcda",
                      "0x55571ce9ee8f69c8", "0x896e6767f999172f", "0x1fe7b3146aa88997", "0x81ea9a2c7d05d99e",
                      "0x6532f588f05b211", "0xfac220d96e24707c", "0xa8b463cdf3a48b52", "0xb8269d7801be7d38",
                      "0x407507fddd8ec4e4", "0x9fd670bff9c56a62", "0x1cf21e354bccc3a7", "0xc7085c67733a84f5",
                      "0x7215df6f93a0baa9", "0xbf4020118826c754", "0xc4187574b11790ac", "0x119a2fb9f8df0279",
                      "0xb5d582747470c4ad", "0x594fa625a6aa9589", "0x943821e188348d51", "0x4a9f119da9e3a429",
                      "0xe0323f26be059812", "0x36e8d53aa2bf2a61", "0xd2d58ab39f2a78a5", "0x1f48bf3defc0b6cd",
                      "0xfd73f564888d53b2", "0xca759d577d56c0a4", "0x84d926e9a264e71b", "0x40b8786917067038",
                      "0xe975bd33ffe52d3a", "0xb1ef0dc4eb735751", "0x2a0393e34c779794", "0xe8a0b8a83ab98c17",
                      "0x6214a9663e4d7be5", "0xdc70061115d3c0b4", "0x75fbd72f66813cfd", "0x3ce69d467a48249b",
                      "0xaab2221e87f0ef4a", "0x69fa6fad79c6e15a", "0xc2f01c105acc1418", "0x85b02af15666735e",
                      "0xbb3a9b6cbc0db631", "0x4b44221ea2a6bf6a", "0x185038b190981af0", "0x91e815a3e2336406",
                      "0xdd5f1d3b9c51abd", "0x9101c68fcf3129c2", "0x7c42b49383a9cdcc", "0x169119786077e5fc",
                      "0x51c2c68d37e5d2f4", "0x5c338b02dce6bf95", "0xdf3523f74a99e74b", "0xce590fd099a855a9",
                      "0xa47552fbbcd9f062", "0x1261825026c433b7", "0xd2b788f70cba2cb9", "0x22a297f8e9b67ff4",
                      "0x792b591828eac2d1", "0xe2463a5cd8501657", "0x9becac6022d03149", "0x1daae18570ec9846",
                      "0xe769e950a2493a1d", "0x291fef31e5bef3d4", "0x4aeba4bb4ac8ed14", "0xa87b99bb1ac84fc3",
                      "0xa43cc2b3c668dec1", "0x55ba28a82e0a4874", "0x888aa9f99ee693bf", "0x414013b258b3a85e",
                      "0xb2220e916bbb415b", "0x29310fc278ee378a", "0x8866e1fd85725b1d", "0x75281c110993bb9e",
                      "0xee60d98aaba54ab2", "0xf9cc1b36043758bf", "0xcfa641d72045893c", "0xf0188b1eeea3d4c7",
                      "0x3a97faf1a0cfa9fb", "0x9ccaf67e701c0dbb", "0xaa43be6dd8ce11b0", "0x7d60bc23c93fc756",
                      "0x5321bd80be653d79", "0xa1455327f5ddfff", "0xfe04a65edea08beb", "0x7c5a457d0960e503",
                      "0xffa22cccbc50185a", "0xebba68722dfeb78c", "0xbc93cd4eb399f34c", "0xfb38ff261b853625",
                      "0x67c248e8645bd531", "0x3a10da71a4a57e2a", "0x4ea9d70a1bb2063c", "0xf2237119f88a90e6",
                      "0x99e0f3b990cf298c", "0x7a0558d4776f376c", "0x826b2605592c9d70", "0x47148ab6bfa0c0ce",
                      "0x1e1843493c4e29b7", "0xd9eabbda06578435", "0xcc4af1376ef3e7c5", "0x72e01100d6e69de5",
                      "0x568275cf88235720", "0x3bc386793b543b7e", "0xf5f34b0325bcb0d5", "0xc21117e5407dd5d4",
                      "0x7f9e88fcf6cd0a43", "0x544ce30c9e92816c", "0xc35f50a7af625622", "0x946d9b16f712499e",
                      "0x4d575443d672f0d4", "0xebb5a87f7bf53712", "0x8e3da0152a8f21ef", "0x2c0e27f416072259",
                      "0x63c2ef08b9bdb2d9", "0x41f616c48ba84212", "0x46da2deebe919b9b", "0xdb01aeb904b32ab1",
                      "0x2ffe56e398ee43f3", "0x0af38a2cf366f6", "0x971041db7a263b48", "0x2ce919b05d19aeb9",
                      "0x868f8dab348ad253", "0x149bcf6541f0a46a", "0xfc83c40c4734d4c", "0x91ec053a3d6a7fae",
                      "0x94e6134f14f5df9f", "0xcf991c5827387ef4", "0x49b75b5843420e59", "0xdc09aa0ebf6edc90",
                      "0x9ba1c8bedbcfd833", "0x38420376e9e5353d", "0xa33777164b7311e8", "0xb6921f131f9a5870",
                      "0x355426c49620d027", "0x4f4313ecac723cfc", "0xb86b5e8b18eaf8b2", "0x42e570c2874ac409",
                      "0x8841071b28d6731a", "0xdc88dbe0a9a5c07a", "0x9dca00dcf7628ea5", "0xde3f97db644bb6e9",
                      "0x12d2c3ddc4a5bc4a", "0xdb2484f79c48d6b6", "0xcf1522813044796b", "0x6aae883064912cae",
                      "0xc46070eedbf1b303", "0xf5033a90f7d15b8", "0x59e5ab30c37e0b0e", "0xf82d3148e0bacf51",
                      "0xd8ba0c59c76548fa", "0x42603cef5609fa2", "0x6c0349c0b4149032", "0x89bfcd426c9a1335",
                      "0x6db9466fc7d75720", "0x7db81f262ba357f9", "0xa8521fbf8477befd", "0x90dffe681beb8a0e",
                      "0xe9ad7fcf1ec62b88", "0xe8da0923d3c4e2af", "0x2c8beec5a52c710a", "0x50e63b09474d41af",
                      "0xbbfe24ebe0a74200", "0xc7aed3142e8cd6f2", "0xeb4d3ecac0df2e84", "0x685fd256e9ca872",
                      "0x88e9a6182df933a7", "0x2fe3f0ff336a967e", "0xfd50dd2adb2fa516", "0x7222e12f8fe9e06b",
                      "0x1ce73976d273a5e", "0xfdebb2266a29aaaf", "0x812fc5483a4d8c75", "0x5f3446183464a474",
                      "0x5177b697626bfff4", "0x357935774d0dc898", "0x274122702af1601c", "0x726b051c90366de6",
                      "0xcaeccce82a4f0249", "0xb1e541f075666f7f", "0x7d5aef9cc609e1e7", "0x7719fa1cd519afcc",
                      "0x10d3d4949e9cd28f", "0x94c64c3469978e1d", "0x8a5aaf183b47e186", "0xa5003c7841000029",
                      "0x78a3be28163cb6ea", "0xeab04e2f120f2cd7", "0x94a63e223736a667", "0x38cd07c188d91608",
                      "0x831202e4e8d74713", "0xb0bbb1474209bcc5", "0x2932d00624d6e4f4", "0x1ca2b65a46c786ab",
                      "0x5dc396a3e5565d15", "0x1b2622af53a26f3e", "0xf968f16f119d57f0", "0x9a7247d2a1e778ee",
                      "0x537c1c898955b434", "0xc1a21dc28bb73c05", "0x5a6291f854533370", "0xd02c94a5c62c37a5",
                      "0xba8045140c463afb", "0x2fb9b17556ce2a84", "0x25207866d14233ba", "0x93518201f627b080",
                      "0xd183c595ba94e7fe", "0xfe0cba8dd15b2c9e", "0xfdb7731824840ca3", "0x37171a7a96de5df9",
                      "0xd51b2624b7ce8c02", "0xdb9e655f8a5c96ee", "0xa6e242b2c610cf09", "0x804a9c6f146eccce",
                      "0x7092965b114b73e5", "0xc10c787397bfae88", "0x3bb965a1c733b1cd", "0x322cf094cbd3bd5d",
                      "0xb779264c2ac8fb73", "0x97e49fb402af65e1", "0xfe981df5fe0374f5", "0xd3981dc9d4840b97",
                      "0x2e5940d13465c8b3", "0x6dec3ff963bfa863", "0x9c91fbd842263505", "0x907639eb1754898c",
                      "0x642409a7c2e01228", "0x6336a2b8591acc31", "0xb1b7fe2d53f33bd5", "0xa037075ee301ebac",
                      "0x2a6bb2df5ad1caf7", "0xb14b6230a6883c9d", "0x8c74af2069b280c3", "0x7713fb7476805c60",
                      "0xf3d393af73ae1264", "0xd288481cb411665d", "0xa093c437c6313847", "0x94dda35a9ce12f91",
                      "0x61de153aaa7567e", "0x690baf9a1fc53347", "0x92d20b458a426610", "0x984ca2ad660f7a9a",
                      "0x2a5fb97367f2d0b3", "0xee42a2fd41eb4a09", "0xc0022a66a734a4ac", "0x1d1609d43e792aa7",
                      "0x1bfdc71e2f790e26", "0x21204199f4d73fa7", "0x22c42227ca30c539", "0xbd1e6eaeef34b5f7",
                      "0x4f8aa2de294b1653", "0xd64110ad4cffeeb3", "0xb1d31a168f743c7d", "0x8d5ed585605eba4b",
                      "0xe0b2a4e10f5313e0", "0xcc0e028f37461245", "0xae90fb77e5f6dcce", "0x1a24cc2ba4feeff7",
                      "0x1191df300e59bc86", "0x9bdb3adbf8cdb730", "0x9cefe092ca8c3ce1", "0x57d2a012ab52c3e9",
                      "0x18299906f0fcd68", "0xd7c41166100ec8bf", "0x3be2dfbedf457cc5", "0x588be88b4b360de5",
                      "0x8c879f57afa28b1", "0x6ab132cd9a81ce55", "0x20f5b5f1b7f39f52", "0x12c40089d119c07b",
                      "0x23c7d21226cbb14a", "0x22374c3017b5770d", "0x3af1639ba6720ab0", "0x3123c109fc31dc39",
                      "0x2e6f3a9ac042c46d", "0x9df0b21456dc5d6f", "0x2c6831e29a80bbe2", "0x269be2fadfe7238",
                      "0xd311822424bb14c3", "0xa1ad09ba3e367d47", "0x7f4d5d3aa7636840", "0xec5a124a8474de68",
                      "0x3fb72cde273da11d", "0x3867e9b9f1e7ed7a", "0xdb20bdf8fd26e9d2", "0xfcb07fe2d74a991d",
                      "0xcc39ba33ac3d3218", "0xcb8f1bab94327b99", "0x4ee5c6bc0fec33f3", "0x48cad5854daef5df",
                      "0x32e56ed90c0bf597", "0x7fa3d0278f1ddba7", "0x38eed6f56275bc70", "0x5c2c94f7941a7195",
                      "0xad3300c9be203d5b", "0x47f9b54e19dd70ff", "0x28f165fc3c0a38b2", "0xcd078a1e3914720d",
                      "0xb0292faa0a30ecfb", "0x8b156b2bb228b09d", "0x7f150b371350c5c8", "0x861ad9b88a8b20bc",
                      "0xbab24882bfac56f5", "0xf124cffcb0e54e27", "0x2a7ddaefd75e32e0", "0xde7900dab55b7634",
                      "0x1271d39e56eb6767", "0x5f225b70aaaf3bc3", "0xa34810cab345b5a8", "0xe992eca39be06bc5",
                      "0x5a7f7e0bd08ac9e2", "0x8ee415632900971", "0xedcd736cbec54a9d", "0x411ad3517244a0b8",
                      "0xc91bd329bd9c9158", "0xfff5474cf4b90b7f", "0x37426488827e8968", "0x5a7b524f7141ee8b",
                      "0xb21aefd0805fdb02", "0x52a492d60d748675", "0xa7f358beea287438", "0xf9778381646b03c3",
                      "0xedfc6806166e646d", "0x3c801b075d6c6a6f", "0x41b19d3de208eb55", "0x7fe6645b54dd3785",
                      "0xa51edeec7c0e76fc", "0x73172f9885641886", "0x497482771de7b930", "0x99903e2bee203eb8",
                      "0xdd5f7a2a7fb920a8", "0xef64bcc43ae9298e", "0x3a660f4058c0b2f8", "0x39e2557c3d5ba046",
                      "0x12eee1b789acb623", "0x483aee1fa5cfaa37", "0x12b64019b4f5ec91", "0x433cb4b7ebb901fc",
                      "0x5eebaa6b0e1c335f", "0x4620fd9c932525c0", "0x996f8c731cadd6a8", "0x132d6b989f39b671",
                      "0xe605299fec19ec81", "0x70880657478274a0", "0x4dd5b8045c2d0ee9", "0x24db9de73c947eae",
                      "0x11ae6d90393abf63", "0x9e21fa1151b065e1", "0x96a85cb00ec52124", "0xc1a634451c1fe521",
                      "0x226b38645f4788ad", "0x704f6ba93450113a", "0xda5bc2ce856a8426", "0xaef23748257f6d39",
                      "0x2fa3fa7bf3395310", "0xa14289df5d146896", "0x8563bb02391ce4b", "0x8a3adc02f03ae637",
                      "0xd6f5d51772384bdb", "0x8939414aabef88eb", "0xe5bf64f620b030bf", "0x4f84a20c0196e291",
                      "0xfc7ce8ff557a79d9", "0x194b62b29cf61abb", "0x456e62e4820a0bdb", "0xb6893a06f9c8e446",
                      "0xae2f3a88b7df853", "0xbba2f53a54100b6c", "0x1c7173ef369813b4", "0x912d4021dfe3d8f7",
                      "0xcf2495ab22ff91a9", "0xf1b7dfde52bd14b0", "0xad5d0bd7df283d10", "0x5bf54104d3e8946",
                      "0xee9a263c5522a2be", "0xefb8b0b00a746a16", "0x23f61607c2db7a4a", "0x18156a9c1a2129f1",
                      "0x94513ab0b7d46f74", "0x7192173e16e8d4a3", "0x91ec7e12616e6aef", "0x2e4e3736a0c0a3e5",
                      "0x461c1e78e1821dab", "0x87bef967775aff7d", "0xc164378ccbee44c7", "0xa94eda7198bff5e0",
                      "0xfa73c869f77f0c93", "0x4a631274feb61d54", "0x3e09e0b464320289", "0x38405b6240443432",
                      "0x62883949bc820ff4", "0x77f5437c21efe442", "0x6f27697fa9886957", "0xc73cb63377c256dd",
                      "0x13a1fffeb6b8a48b", "0x3a0184cbf159414c", "0xbc4a0dfb06e4c88a", "0xb656d8ae420001a5",
                      "0x2fe8957285f175c", "0xe42cf961309efc9a", "0xa29fe9b8c00efd3b", "0xb3322a91c83e951a",
                      "0xf31b2e55346c1ae0", "0x6d57ea736a5788a4", "0x7922e7364f4fd7cf", "0x528a69c4272c68b6",
                      "0x2595793290e1a3d7", "0x2177f94da2dd9e41", "0xbcde6f9d42c563ef", "0x8c53e86374394f01",
                      "0x2e05a16eb845a8a7", "0x24eab70429a820c6", "0xa8a27fb3882b2cdc", "0x7c3940f00d1c1dba",
                      "0x79282d37e0624616", "0xa1ace9ba2355213b", "0x6327ef00b0d30a0", "0x6019f164385f7e22",
                      "0xe4261e7a681dbd0c"];

const matrix_64: Vec<Vec<&'static str>>  = vec![
    ["0xd0f90449f141a03a", "0x7e786e3c01b46475", "0x9d845622e36d87fe", "0x85acd9c81446e876", "0x908500e30cb13fbf",
     "0x33510de023baef18", "0xa4f0fb9dac688d82", "0xe65b1572da0991ba", "0x70449b75736fe86d"],
    ["0xde9cd1600c961f10", "0x7b94ca547d41b687", "0xad4cc3702545d88b", "0x3434702e96566da4", "0x8fd9903afcc0ed6d",
     "0xfcca503e047419b4", "0x22ff797ef444ad3d", "0x80bd4941dfa3df44", "0x27eb8baea29fe272"],
    ["0x232f56afaa9cb1ba", "0xec6af281f7bf9997", "0x464c0d3cd39eabf5", "0xdfe4b3cc13c0fab0", "0x624db025685a69a1",
     "0x856bbba1c85ad167", "0x127b1ed684a7f918", "0xa973036074f99e27", "0xbd681100c639089e"],
    ["0xd0c067f20c9849ed", "0x237439c54b70e7a8", "0xe859e7be8d8e1f4b", "0x229f3a469b63a77d", "0xc4425b27f12f214d",
     "0xfaa979322eb88b03", "0xa7d81b02eac355c", "0x55e5956ceed0220", "0xae8f0f63a4aeec0f"],
    ["0xe4a2264e682573b8", "0x3a8e5dc1b47546a9", "0xbf504eb11b946fea", "0x5e20fe6931b3cdba", "0x45c4f15dd42d6804",
     "0x3497ff9334010f1d", "0x454247d6184396bf", "0xb92c9de37fe52d8a", "0x171cce6c939f936"],
    ["0xf42b17b28d4b6d79", "0x11fe9d657a4ba2e8", "0xeae9bc0199efd564", "0xd58d4f91f9496fb7", "0xec044b014681768",
     "0x55af9ca03d8d49b1", "0x85fbd0ab098fef6a", "0x3741bb7ffcc4f8d9", "0xb9314d7dcfc948ba"],
    ["0x89845da8e9cdcb24", "0x8874a7d8a57cdea2", "0xde562f62637aa59a", "0x6b082b03d36d4bb0", "0x68f0fe53e2518f0c",
     "0xed14f650a86d4200", "0x5e3b107979b466f7", "0x6fd02c9e5f5a8963", "0x54fecbb6233ff946"],
    ["0xef09185bea0dc3de", "0xac3bff714c0bc1dc", "0x935be70d82eabae2", "0x2a9448159d808a2f", "0xd3612efa4d8655d4",
     "0x9b4459a1a189016b", "0x52df80b7d43a1b91", "0x1ce704edbed1b251", "0xf01f0ae6812e9695"],
    ["0x62f0e2bd38b0a7ac", "0x9a9fcd1d25921649", "0xd1c281cb42b33f10", "0x67624b51bf3887e6", "0xab671059dbfd4c02",
     "0xc3da6ec38fbe8d5c", "0xe2c9499a86ab13ee", "0xda6cf9a09c55ba43", "0x589aca3c2e3b8334"]];

