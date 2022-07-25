mod curves;
use ark_std::test_rng;
use crate::curves::secp256k1;

fn main() {
    let mut rng = test_rng();
    let a = secp256k1::Affine::rand(&mut rng);
    let b = secp256k1::Affine::rand(&mut rng);
    let c = a + b;
}
