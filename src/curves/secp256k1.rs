use ark_ff::fields::{Fp320, MontBackend};
use ark_ec::{
    models::short_weierstrass::SWCurveConfig as SWModelParameters,
    models::CurveConfig as ModelParameters,
    //short_weierstrass::{SWCurveConfig, Affine},
    short_weierstrass::Affine as GroupAffine,
};

//use ark_bls12_381::{G1Projective as G, G1Affine as GAffine, Fr as ScalarField};

// Fq for Secp256k1
#[derive(ark_ff::MontConfig)]
#[modulus = "115792089237316195423570985008687907853269984665640564039457584007908834671663"]
#[generator = "3"]
pub struct FqConfig;
pub type Fq = Fp320<MontBackend<FqConfig, 5>>;

// Fq for Secp256k1
#[derive(ark_ff::MontConfig)]
#[modulus = "115792089237316195423570985008687907852837564279074904382605163141518161494337"]
#[generator = "7"]
pub struct FrConfig;
pub type Fr = Fp320<MontBackend<FrConfig, 5>>;


#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Secp256k1Config;

impl ModelParameters for Secp256k1Config {
    type BaseField = Fq;
    type ScalarField = Fr;

    const COFACTOR: &'static [u64] = &[0x1];
    const COFACTOR_INV: Fr = ark_ff::MontFp!("1");
}

pub type Affine = GroupAffine<Secp256k1Config>;

impl SWModelParameters for Secp256k1Config {
    const COEFF_A: Fq = ark_ff::MontFp!("0");
    const COEFF_B: Fq = ark_ff::MontFp!("7");
    const GENERATOR: Affine = Affine::new_unchecked(GENERATOR_X, GENERATOR_Y);
    // type BaseField = Fq;
    // type BaseScalar = Fr;
}


pub const GENERATOR_X: Fq = ark_ff::MontFp!("46061211676436338764683880818785966081188935017876647816831813008620600064");
pub const GENERATOR_Y: Fq = ark_ff::MontFp!("104358288673182274090059068278833057796518081823672618493975389967095627647");
