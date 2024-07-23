use crate::util::*;
use lapack_array2::prelude::*;
use ndarray::prelude::*;

#[cfg(test)]
mod test {
    use super::*;

    fn test_generic<F>()
    where
        F: SYEVNum + TestFloat + 'static,
        <F as LapackFloat>::RealFloat: TestFloat,
    {
        let a_orig = random_matrix::<F>(4, 4, 'C'.into());
        let (v_orig, w_orig) = SYEV::<F>::default().a(a_orig.view()).run().unwrap().into_owned();
        // valid original result
        {
            let w_orig_diag = Array2::from_diag(&w_orig.mapv(F::from_real));
            let a_retrive = v_orig.dot(&w_orig_diag).dot(&v_orig.t().mapv(F::conj));
            let a_orig_symm = hermitianize(&a_orig.view(), 'U');
            allclose_epsilon(&a_retrive.view(), &a_orig_symm.view(), 1.0e-10);
        }
        // valid row-major
        {
            let a = a_orig.as_standard_layout().clone();
            let (v, w) = SYEV::<F>::default().a(a.view()).run().unwrap().into_owned();
            let w_diag = Array2::from_diag(&w.mapv(F::from_real));
            let a_orig_symm = hermitianize(&a_orig.view(), 'U');
            let a_retrive = v.dot(&w_diag).dot(&v.t().mapv(F::conj));
            allclose_epsilon(&a_retrive.view(), &a_orig_symm.view(), 1.0e-10);
            allclose_epsilon(&v.mapv(F::abs).view(), &v_orig.mapv(F::abs).view(), 1.0e-10);
            allclose_epsilon(&w.view(), &w_orig.view(), 1.0e-10);
        }
        // valid row-major mut
        {
            let mut a = a_orig.as_standard_layout().clone();
            let mut w = Array1::zeros(4);
            let _ = SYEV::<F>::default().a(a.view_mut()).w(w.view_mut()).run().unwrap().into_owned();
            allclose_epsilon(&a.mapv(F::abs).view(), &v_orig.mapv(F::abs).view(), 1.0e-10);
            allclose_epsilon(&w.view(), &w_orig.view(), 1.0e-10);
        }
        // valid row-major mut
        {
            let mut a = a_orig.as_standard_layout().clone();
            let mut w = Array1::zeros(4);
            let _ = SYEV::<F>::default().a(a.view_mut()).w(w.view_mut()).run().unwrap().into_owned();
            allclose_epsilon(&a.mapv(F::abs).view(), &v_orig.mapv(F::abs).view(), 1.0e-10);
            allclose_epsilon(&w.view(), &w_orig.view(), 1.0e-10);
        }
    }

    #[test]
    fn test() {
        test_generic::<f64>();
        test_generic::<c64>();
    }
}
