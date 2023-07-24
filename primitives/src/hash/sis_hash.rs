use std::marker::PhantomData;

use lark_algebra::{Field, NTTField, PolynomialRing};
use rand::RngCore;

use super::Hash;

/// A hash function is instantiated with k polynomial ring element.
pub struct SISHash<F: NTTField, R: PolynomialRing<F>, const k: usize> {
    pub(crate) state: Option<R>,
    _phantom: PhantomData<F>,
}

pub struct SISHashParam<F: NTTField, R: PolynomialRing<F>, const k: usize> {
    pub(crate) param: [R; k],
    _phantom: PhantomData<F>,
}

impl<F: NTTField, R: PolynomialRing<F>, const k: usize> Hash for SISHash<F, R, k> {
    type Parameters = SISHashParam<F, R, k>;
    type Input = R;
    type Output = R;

    fn setup(mut rng: impl RngCore) -> Self::Parameters {
        let param = (0..k)
            .map(|_| <R as PolynomialRing<F>>::random(&mut rng))
            .collect::<Vec<_>>();
        SISHashParam {
            param: param.try_into().unwrap(),
            _phantom: PhantomData,
        }
    }

    fn init() -> Self {
        SISHash {
            state: None,
            _phantom: PhantomData,
        }
    }

    fn update(&mut self, param: &Self::Parameters, input: &[Self::Input]) {
        todo!()
    }

    fn finalize(&self) -> Self::Output {
        todo!()
    }
}

#[inline]
fn get_num_rounds(len: usize, k: usize) -> usize {
    if len <= k {
        return 1;
    }
    1 + (len - k) / (k - 1)
}

#[inline]
/// Compute \sum_{i=0}^{n-1} a_i * b_i where n = min(a.len(), b.len())
/// Allow a_i and b_i to have variable length
fn inner_product<F: NTTField, R: PolynomialRing<F>>(a: &[R], b: &[R]) -> R {
    a.iter()
        .zip(b.iter())
        .fold(R::default(), |acc, (&a, &b)| acc + a * b)
}
