mod curves;
use ark_std::test_rng;
use ark_ff::UniformRand;
use crate::curves::secp256k1;

fn main() {
    let mut rng = test_rng();
    let a = secp256k1::Affine::rand(&mut rng);
    let b = secp256k1::Affine::rand(&mut rng);
    let _c = a + b;
}
