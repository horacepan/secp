use ark_ff::fields::{Fp256, MontBackend, MontConfig};
use ark_ff::{MontFp};
use ark_ec::{
    //short_weierstrass::{SWCurveConfig, Affine},
    models::{SWModelParameters, ModelParameters},
    short_weierstrass_jacobian::{GroupAffine},
};

//use ark_bls12_381::{G1Projective as G, G1Affine as GAffine, Fr as ScalarField};

// Fq for Secp256k1
#[derive(MontConfig)]
#[modulus = "115792089237316195423570985008687907853269984665640564039457584007908834671663"]
#[generator = "3"]
pub struct FqConfig;
pub type Fq = Fp256<MontBackend<FqConfig, 4>>;

// Fq for Secp256k1
#[derive(MontConfig)]
#[modulus = "115792089237316195423570985008687907852837564279074904382605163141518161494337"]
#[generator = "7"]
pub struct FrConfig;
pub type Fr = Fp256<MontBackend<FrConfig, 4>>;


#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Secp256k1Config;
pub const GENERATOR_X: Fq = MontFp!("46061211218676436338764683880818785966081188935017876647816831813008620600064");
pub const GENERATOR_Y: Fq = MontFp!("104357758288673182274090059068278833057796518081823672618493975389967095627647");

impl ModelParameters for Secp256k1Config {
    type BaseField = Fq;
    type ScalarField = Fr;
}

pub type Affine = GroupAffine<Secp256k1Config>;

impl SWModelParameters for Secp256k1Config {
    const COEFF_A: Fq = 0;
    const COEFF_B: Fq = 7;
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) = (GENERATOR_X, GENERATOR_Y);
    const COFACTOR: &'static [u64] = &[0x1];
    const COFACTOR_INV: Fr = MontFp!("1");
    // type BaseField = Fq;
    // type BaseScalar = Fr;
}
