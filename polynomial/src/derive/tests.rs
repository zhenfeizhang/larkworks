#[macro_export]
macro_rules! impl_poly_tests {
    ($poly:ident, $modulus:ident) => {
        #[test]
        fn test_normalization() {
            let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
            for _ in 0..10 {
                let mut poly = $poly::rand_poly(&mut rng);
                let mut poly2 = poly.clone();

                poly.lift();
                poly2
                    .coeffs
                    .iter_mut()
                    .for_each(|x| *x = *x + ((rng.next_u32() % 100) * $modulus as u32) as i32);
                poly2.lift();
                for (e, f) in poly.coeffs.iter().zip(poly2.coeffs.iter()) {
                    assert_eq!(e, f)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_ntt_poly_tests {
    ($poly:ident, $poly_ntt:ident, $modulus:ident) => {
        #[test]
        fn test_conversion() {
            let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
            for _ in 0..10 {
                let poly = $poly::rand_poly(&mut rng);
                let poly_ntt: $poly_ntt = (&poly).into();
                let poly_rec: $poly = (&poly_ntt).into();

                assert_eq!(poly, poly_rec)
            }
        }

        #[test]
        fn test_arithmetic() {
            let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
            for _ in 0..10 {
                let a = $poly::rand_poly(&mut rng);
                let a_ntt: $poly_ntt = (&a).into();
                let b = $poly::rand_poly(&mut rng);
                let b_ntt: $poly_ntt = (&b).into();

                {
                    // test correctness of ntt multiplications
                    let c_ntt = a_ntt * b_ntt;
                    let c: $poly = (&c_ntt).into();
                    let c_rec = $poly::schoolbook(&a, &b);

                    assert_eq!(c, c_rec);
                }
                {
                    // test correctness of ntt additions
                    let d_ntt = a_ntt + b_ntt;
                    let d: $poly = (&d_ntt).into();
                    let d_rec = a + b;

                    assert_eq!(d, d_rec)
                }
            }
        }

        #[test]
        fn bench_fft() {
            let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
            let size = 10000;
            let mut p: Vec<_> = (0..size)
                .map(|_| $poly::rand_poly(&mut rng).coeffs)
                .collect();
            let p_rec = p.clone();
            let timer = start_timer!(|| format!("start {} fft", size));
            for e in p.iter_mut() {
                ntt(e)
            }
            end_timer!(timer);

            let timer = start_timer!(|| format!("start {} ifft", size));
            for e in p.iter_mut() {
                inv_ntt(e)
            }
            end_timer!(timer);

            for (x, y) in p.iter().zip(p_rec.iter()) {
                for (xx, yy) in x.iter().zip(y.iter()) {
                    assert_eq!(lift(*xx, $modulus), lift(*yy, $modulus))
                }
            }
        }
    };
}
