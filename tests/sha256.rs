use streamsha::*;
use streamsha::traits::{
    StreamHasher, Resumable
};
use hex_literal::hex;


#[allow(non_upper_case_globals)]
const data: &[u8] = &hex!("3082062130820509a0030201020204012d57ca300d06092a864886f70d01010b0500308182310b3009060355040613024a50310d300b060355040a0c044a504b4931253023060355040b0c1c4a504b4920666f7220757365722061757468656e7469636174696f6e313d303b060355040b0c344a6170616e204167656e637920666f72204c6f63616c20417574686f7269747920496e666f726d6174696f6e2053797374656d73301e170d3139303732353135323830365a170d3234303530323134353935395a302f310b3009060355040613024a503120301e06035504030c17383430343434453337504146484e30383232303030334130820122300d06092a864886f70d01010105000382010f003082010a0282010100c2e48c45c07363e246be44407c8af5317cbccd3aa8be5d26129224525ac9fd73bc65296102d48744600952f0493c397657c966e2564ff9ef5175357eec9628036096326107a90bd538f67390aaecbcd85672bdc66f088b3f1fa0657009c146dbec38111c50757358e3016803cf5ece665927b377afdf058432a624b372d2e39cf534ab9ed449da12ba239fe0dd96f65c72ccea6b6bfd9733c41e90edee1f842078ac5cde7c95c6242a322516ef22927f35abb8afe8327633d7ded0959384d205853b84726fabed29182f0213b6a74f118651d2c4c415b8253d3ac2d339c8775361b6201849fe99626f591f558c5c916a79182c856bb1599ad12be5d33748e7990203010001a38202ef308202eb300e0603551d0f0101ff04040302078030130603551d25040c300a06082b0601050507030230490603551d200101ff043f303d303b060b2a83088c9b55080501031e302c302a06082b06010505070201161e687474703a2f2f7777772e6a706b692e676f2e6a702f6370732e68746d6c3081b70603551d120481af3081aca481a93081a6310b3009060355040613024a5031273025060355040a0c1ee585ace79a84e5808be4babae8aa8de8a8bce382b5e383bce38393e382b931393037060355040b0c30e585ace79a84e5808be4babae8aa8de8a8bce382b5e383bce38393e382b9e588a9e794a8e88085e8a8bce6988ee794a831333031060355040b0c2ae59cb0e696b9e585ace585b1e59ba3e4bd93e68385e5a0b1e382b7e382b9e38386e383a0e6a99fe6a78b3081b10603551d1f0481a93081a63081a3a081a0a0819da4819a308197310b3009060355040613024a50310d300b060355040a0c044a504b4931253023060355040b0c1c4a504b4920666f7220757365722061757468656e7469636174696f6e3120301e060355040b0c1743524c20446973747269627574696f6e20506f696e747331143012060355040b0c0b49626172616b692d6b656e311a301806035504030c115473756b7562612d7368692043524c4450303a06082b06010505070101042e302c302a06082b06010505073001861e687474703a2f2f6f637370617574686e6f726d2e6a706b692e676f2e6a703081af0603551d230481a73081a480149567951b5ca70d84a0fff1d85a87f1aab1340385a18188a48185308182310b3009060355040613024a50310d300b060355040a0c044a504b4931253023060355040b0c1c4a504b4920666f7220757365722061757468656e7469636174696f6e313d303b060355040b0c344a6170616e204167656e637920666f72204c6f63616c20417574686f7269747920496e666f726d6174696f6e2053797374656d73820101301d0603551d0e0416041477f6c4d716d8cde22a27eed3d3af496e1fb0eff5300d06092a864886f70d01010b050003820101002addf5bce542900c6f93ab3ccfce694bc20fbf94d6096342c217cff14658047f4c1e40db2368267842081093b80a8a1cb9d0925efe110240a7115fb9831ecbb5f70e1fa38bb97842ad68204f411a938ac7fb316bb86dd0e32ea248d780bf8bf4e130dbf156a336ede2c0a1a52f4c46f25c59843973c19e910a11a72b802a55fe4a98d202003f287ab62f90bbf83f577c74a499561ee005ad9bed1056977a529a4f3c8cd395a37e7f5b3c9e7f98c113a091ab75525589e91dc5f152d35ad209f6c066c0b69bc1193b92c6eb8781d5cccbc353f6d521cc37af3cac600c61df67a7117c8dfc5b33446276e2cc0515e859bea1dfd37aa4c238e665f655d1b14f5fd3");

const vectors: &[(&[u8], [u8;32])] = &[
    (&[0xbd], hex!("68325720 aabd7c82 f30f554b 313d0570 c95accbb 7dc4b5aa e11204c0 8ffe732b")),
    (&hex!("c98c8e55"), hex!("7abc22c0 ae5af26c e93dbb94 433a0e0b 2e119d01 4f8e7f65 bd56c61c cccd9504")),
    (&[0;55], hex!("02779466 cdec1638 11d07881 5c633f21 90141308 1449002f 24aa3e80 f0b88ef7")),
    (&[0; 56], hex!("d4817aa5 497628e7 c77e6b60 6107042b bba31308 88c5f47a 375e6179 be789fbb")),
    (&[0; 57], hex!("65a16cb7 861335d5 ace3c607 18b5052e 44660726 da4cd13b b745381b 235a1785")),
    (&[0; 64], hex!("f5a5fd42 d16a2030 2798ef6e d309979b 43003d23 20d9f0e8 ea9831a9 2759fb4b")),
    (&[0; 1000], hex!("541b3e9d aa09b20b f85fa273 e5cbd3e8 0185aa4e c298e765 db87742b 70138a53")),
    (&[0x41; 1000], hex!("c2e68682 3489ced2 017f6059 b8b23931 8b6364f6 dcd835d0 a519105a 1eadd6e4")),
    (&[0x55; 1005], hex!("f4d62dde c0f3dd90 ea1380fa 16a5ff8d c4c54b21 740650f2 4afc4120 903552b0"))
];
#[test]
fn it_hashes_hash_null() {
    let mut hasher = Sha256::new();
    let hash = hasher.finish();
    assert_eq!(
        hash,
        hex!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
    )
}
#[test]
fn it_hashes_hash_abc() {
    let mut hasher = Sha256::new();
    let written_bytes = hasher.update(b"abc");
    assert_eq!(written_bytes, 3);
    let hash = hasher.finish();
    assert_eq!(
        hash,
        hex!("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")
    )
}
#[test]
fn it_hashes_long_data() {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finish();
    assert_eq!(
        hash,
        hex!("449ef3f93be0c5425330af2b41c63769190aa75a713223d96839f6537104007e")
    )
}
#[test]
fn it_hashes_splitted_data() {
    let mut hasher = Sha256::new();
    hasher.update(&data[0..64]);
    hasher.update(&data[64..345]);
    hasher.update(&data[345..356]);
    hasher.update(&data[356..356]);
    hasher.update(&data[356..357]);
    hasher.update(&data[357..]);
    let hash = hasher.finish();
    assert_eq!(
        hash,
        hex!("449ef3f93be0c5425330af2b41c63769190aa75a713223d96839f6537104007e")
    )
}
#[test]
fn it_can_resume() {
    let mut hasher = Sha256::new();
    hasher.update(&data[0..195]);
    let state = hasher.pause();
    let mut hasher2 = Sha256::resume(state).unwrap();
    hasher2.update(&data[195..]);
    let hash = hasher2.finish();
    assert_eq!(
        hash,
        hex!("449ef3f93be0c5425330af2b41c63769190aa75a713223d96839f6537104007e")
    )
}
#[test]
fn it_can_hash_vectors() {
    for i in vectors.iter(){
        let mut hasher = Sha256::new();
        hasher.update(i.0);
        let hash = hasher.finish();
        assert_eq!(hash, i.1)
    }
}
#[test]
fn it_can_hash_1000000_zeros() {
    let mut hasher = Sha256::new();
    for _ in 0..1000 {
        hasher.update(&[0; 1000]);
    }
    let hash = hasher.finish();
    assert_eq!(hash, hex!("d29751f2 649b32ff 572b5e0a 9f541ea6 60a50f94 ff0beedf b0b692b9 24cc8025"))
}
#[test]
fn it_can_hash_0x20000000_z() {
    let mut hasher = Sha256::new();
    for _ in 0..0x100000 {
        hasher.update(&[0x5a; 0x200]);
    }
    let hash = hasher.finish();
    assert_eq!(hash, hex!("15a1868c 12cc5395 1e182344 277447cd 0979536b adcc512a d24c67e9 b2d4f3dd "))
}
#[test]
fn it_can_hash_0x41000000_zeros() {
    let mut hasher = Sha256::new();
    for _ in 0..0x100000 {
        hasher.update(&[0; 0x410]);
    }
    let hash = hasher.finish();
    assert_eq!(hash, hex!("461c19a9 3bd4344f 9215f5ec 64357090 342bc66b 15a14831 7d276e31 cbc20b53"))
}
#[test]
fn it_can_hash_0x6000003e_b() {
    let mut hasher = Sha256::new();
    for _ in 0..0x100000 {
        hasher.update(&[0x42; 0x600]);
    }
    hasher.update(&[0x42; 0x3e]);
    let hash = hasher.finish();
    assert_eq!(hash, hex!("c23ce8a7 895f4b21 ec0daf37 920ac0a2 62a22004 5a03eb2d fed48ef9 b05aabea"))
}