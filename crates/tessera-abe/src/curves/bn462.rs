use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::random::{miracl::MiraclRng, Random};

use super::{
    Field, FieldWithOrder, GroupG1, GroupG2, GroupGt, Inv, PairingCurve, Pow, RefAdd, RefDiv, RefMul, RefNeg, RefPow,
    RefSub,
};
use lazy_static::lazy_static;

use rand_core::RngCore as _;
use serde::{Deserialize, Serialize};
use tessera_miracl::{
    bn462::{
        big::{BIG, MODBYTES, NLEN},
        ecp::ECP,
        ecp2::ECP2,
        fp12::FP12,
        pair,
        rom::CURVE_ORDER,
    },
    hash256::HASH256,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Bn462Field {
    inner: BIG,
}

const MODULUS: [i64; NLEN] = CURVE_ORDER;
const MSG_SIZE: usize = 48 * MODBYTES;

lazy_static! {
    pub static ref MODULUS_BIG: BIG = BIG::new_ints(&MODULUS);
}

impl Field for Bn462Field {
    type Chunk = i64;

    #[inline]
    fn new() -> Self {
        Self { inner: BIG::new() }
    }

    #[inline]
    fn one() -> Self {
        Self { inner: BIG::new_int(1) }
    }

    #[inline]
    fn new_int(x: Self::Chunk) -> Self {
        Self { inner: BIG::new_int(x as isize) }
    }

    #[inline]
    fn new_ints(x: &[Self::Chunk]) -> Self {
        Self { inner: BIG::new_ints(x) }
    }
}

impl From<u64> for Bn462Field {
    #[inline]
    fn from(x: u64) -> Self {
        Self { inner: BIG::new_int(x as isize) }
    }
}

impl Random for Bn462Field {
    type Rng = MiraclRng;

    #[inline]
    fn random(rng: &mut Self::Rng) -> Self {
        Self { inner: BIG::random(&mut rng.inner) }
    }
}

impl FieldWithOrder for Bn462Field {
    #[inline]
    fn order() -> Self {
        Self { inner: *MODULUS_BIG }
    }
    #[inline]
    fn random_within_order(rng: &mut <Self as Random>::Rng) -> Self {
        let mut r = BIG::random(&mut rng.inner);
        r.rmod(&MODULUS_BIG);
        Self { inner: r }
    }
}

impl Sum<Bn462Field> for Bn462Field {
    fn sum<I: Iterator<Item = Bn462Field>>(iter: I) -> Self {
        iter.fold(Self::new(), |acc, x| acc + x)
    }
}

impl Add for Bn462Field {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        self.ref_add(&other)
    }
}

impl RefAdd for Bn462Field {
    type Output = Self;

    #[inline]
    fn ref_add(&self, other: &Self) -> Self {
        Self { inner: BIG::modadd(&self.inner, &other.inner, &MODULUS_BIG) }
    }
}

impl Div for Bn462Field {
    type Output = Self;

    #[inline]
    fn div(self, mut other: Self) -> Self {
        other.inner.invmodp(&MODULUS_BIG);
        Self { inner: BIG::modmul(&self.inner, &other.inner, &MODULUS_BIG) }
    }
}

impl RefDiv for Bn462Field {
    type Output = Self;

    #[inline]
    fn ref_div(&self, other: &Self) -> Self {
        let mut other = other.inner;
        other.invmodp(&MODULUS_BIG);
        Self { inner: BIG::modmul(&self.inner, &other, &MODULUS_BIG) }
    }
}

impl Mul for Bn462Field {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        self.ref_mul(&other)
    }
}

impl RefMul for Bn462Field {
    type Output = Self;

    #[inline]
    fn ref_mul(&self, other: &Self) -> Self {
        Self { inner: BIG::modmul(&self.inner, &other.inner, &MODULUS_BIG) }
    }
}

impl Sub for Bn462Field {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        self.ref_sub(&other)
    }
}

impl RefSub for Bn462Field {
    type Output = Self;

    #[inline]
    fn ref_sub(&self, other: &Self) -> Self {
        let neg_other = BIG::modneg(&other.inner, &MODULUS_BIG);
        Self { inner: BIG::modadd(&self.inner, &neg_other, &MODULUS_BIG) }
    }
}

impl Neg for Bn462Field {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        self.ref_neg()
    }
}

impl RefNeg for Bn462Field {
    type Output = Self;

    #[inline]
    fn ref_neg(&self) -> Self {
        Self { inner: BIG::modneg(&self.inner, &MODULUS_BIG) }
    }
}

impl Pow for Bn462Field {
    type Output = Self;

    #[inline]
    fn pow(mut self, e: &Self) -> Self {
        Self { inner: self.inner.powmod(&e.inner, &MODULUS_BIG) }
    }
}

impl RefPow for Bn462Field {
    type Output = Self;

    #[inline]
    fn ref_pow(&self, e: &Self) -> Self {
        self.clone().pow(e)
    }
}

impl PartialEq for Bn462Field {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        BIG::comp(&self.inner, &other.inner) == 0
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct G1 {
    inner: ECP,
}

impl GroupG1 for G1 {
    type Field = Bn462Field;

    #[inline]
    fn new(x: &Self::Field) -> Self {
        Self::generator() * x
    }

    #[inline]
    fn zero() -> Self {
        Self { inner: ECP::new() }
    }

    #[inline]
    fn generator() -> Self {
        Self { inner: ECP::generator() }
    }
}

impl Mul<Bn462Field> for G1 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Bn462Field) -> Self {
        self.ref_mul(&rhs)
    }
}

impl Mul<&Bn462Field> for G1 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: &Bn462Field) -> Self {
        self.ref_mul(rhs)
    }
}

impl RefMul<Bn462Field> for G1 {
    type Output = Self;

    #[inline]
    fn ref_mul(&self, rhs: &Bn462Field) -> Self {
        Self { inner: pair::g1mul(&self.inner, &rhs.inner) }
    }
}

impl Add for G1 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        self + &other
    }
}

impl Add<&G1> for G1 {
    type Output = Self;

    #[inline]
    fn add(mut self, other: &Self) -> Self {
        self.inner.add(&other.inner);
        self
    }
}

impl RefAdd for G1 {
    type Output = Self;

    #[inline]
    fn ref_add(&self, other: &Self) -> Self {
        self.clone() + other
    }
}

impl Neg for G1 {
    type Output = Self;

    #[inline]
    fn neg(mut self) -> Self {
        self.inner.neg();
        self
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct G2 {
    inner: ECP2,
}

impl GroupG2 for G2 {
    type Field = Bn462Field;

    fn new(x: &Self::Field) -> Self {
        Self::generator() * x
    }

    fn generator() -> Self {
        Self { inner: ECP2::generator() }
    }
}

impl Mul<Bn462Field> for G2 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Bn462Field) -> Self {
        self.ref_mul(&rhs)
    }
}

impl Mul<&Bn462Field> for G2 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: &Bn462Field) -> Self {
        self.ref_mul(rhs)
    }
}

impl RefMul<Bn462Field> for G2 {
    type Output = Self;

    #[inline]
    fn ref_mul(&self, rhs: &Bn462Field) -> Self {
        Self { inner: pair::g2mul(&self.inner, &rhs.inner) }
    }
}

impl Add for G2 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        self + &other
    }
}

impl Add<&G2> for G2 {
    type Output = Self;

    #[inline]
    fn add(mut self, other: &Self) -> Self {
        self.inner.add(&other.inner);
        self
    }
}

impl RefAdd for G2 {
    type Output = Self;

    #[inline]
    fn ref_add(&self, other: &Self) -> Self {
        self.clone() + other
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Gt {
    inner: FP12,
}

impl GroupGt for Gt {
    type Field = Bn462Field;

    #[inline]
    fn one() -> Self {
        let mut r = FP12::new();
        r.one();
        Self { inner: r }
    }
}

impl From<Gt> for Vec<u8> {
    #[inline]
    fn from(gt: Gt) -> Self {
        let mut bytes = vec![0u8; MSG_SIZE];
        gt.inner.tobytes(&mut bytes);
        bytes
    }
}

impl<'a> From<&'a [u8]> for Gt {
    #[inline]
    fn from(bytes: &'a [u8]) -> Self {
        Self { inner: FP12::frombytes(bytes) }
    }
}

impl Random for Gt {
    type Rng = MiraclRng;
    fn random(rng: &mut Self::Rng) -> Self {
        let mut rand_bytes = [0u8; MSG_SIZE];
        rng.fill_bytes(&mut rand_bytes);
        let r = FP12::frombytes(&rand_bytes);
        Self { inner: r }
    }
}

impl Mul for Gt {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        self * &other
    }
}

impl Mul<&Self> for Gt {
    type Output = Self;

    #[inline]
    fn mul(mut self, rhs: &Self) -> Self {
        self.inner.mul(&rhs.inner);
        self
    }
}

impl RefMul for Gt {
    type Output = Self;

    #[inline]
    fn ref_mul(&self, rhs: &Self) -> Self {
        self.clone() * rhs
    }
}

impl Pow<Bn462Field> for Gt {
    type Output = Self;

    #[inline]
    fn pow(self, rhs: &Bn462Field) -> Self {
        self.ref_pow(rhs)
    }
}

impl RefPow<Bn462Field> for Gt {
    type Output = Self;

    #[inline]
    fn ref_pow(&self, rhs: &Bn462Field) -> Self {
        Self { inner: pair::gtpow(&self.inner, &rhs.inner) }
    }
}

impl Inv for Gt {
    type Output = Self;

    fn inverse(mut self) -> Self {
        self.inner.inverse();
        self
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Bn462Curve;

impl PairingCurve for Bn462Curve {
    type Rng = MiraclRng;
    type Field = Bn462Field;
    type G1 = G1;
    type G2 = G2;
    type Gt = Gt;

    fn pair(e1: &Self::G1, e2: &Self::G2) -> Self::Gt {
        Self::Gt { inner: pair::fexp(&pair::ate(&e2.inner, &e1.inner)) }
    }

    fn hash_to_g1(msg: &[u8]) -> Self::G1 {
        let mut hash = HASH256::new();
        hash.process_array(msg);
        let h = hash.hash();
        Self::G1 { inner: ECP::mapit(&h) }
    }

    fn hash_to_g2(msg: &[u8]) -> Self::G2 {
        let mut hash = HASH256::new();
        hash.process_array(msg);
        let h = hash.hash();
        Self::G2 { inner: ECP2::mapit(&h) }
    }
}