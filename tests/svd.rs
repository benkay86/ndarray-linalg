
extern crate ndarray;
extern crate ndarray_rand;
extern crate ndarray_linalg;
extern crate ndarray_numtest;

use std::cmp::min;
use ndarray::prelude::*;
use ndarray_linalg::prelude::*;
use ndarray_numtest::prelude::*;
use ndarray_rand::RandomExt;

#[test]
fn svd_square() {
    let r_dist = RealNormal::new(0., 1.);
    let a = Array::<f64, _>::random((3, 3), r_dist);
    let (u, s, vt) = a.clone().svd().unwrap();
    let mut sm = Array::eye(3);
    for i in 0..3 {
        sm[(i, i)] = s[i];
    }
    u.dot(&sm).dot(&vt).assert_allclose_l2(&a, 1e-7);
}
#[test]
fn svd_square_t() {
    let r_dist = RealNormal::new(0., 1.);
    let a = Array::<f64, _>::random((3, 3), r_dist).reversed_axes();
    let (u, s, vt) = a.clone().svd().unwrap();
    let mut sm = Array::eye(3);
    for i in 0..3 {
        sm[(i, i)] = s[i];
    }
    u.dot(&sm).dot(&vt).assert_allclose_l2(&a, 1e-7);
}

#[test]
fn svd_4x3() {
    let r_dist = RealNormal::new(0., 1.);
    let a = Array::<f64, _>::random((4, 3), r_dist);
    let (u, s, vt) = a.clone().svd().unwrap();
    let mut sm = Array::zeros((4, 3));
    for i in 0..3 {
        sm[(i, i)] = s[i];
    }
    u.dot(&sm).dot(&vt).assert_allclose_l2(&a, 1e-7);
}
#[test]
fn svd_4x3_t() {
    let r_dist = RealNormal::new(0., 1.);
    let a = Array::<f64, _>::random((3, 4), r_dist).reversed_axes();
    let (u, s, vt) = a.clone().svd().unwrap();
    let mut sm = Array::zeros((4, 3));
    for i in 0..3 {
        sm[(i, i)] = s[i];
    }
    u.dot(&sm).dot(&vt).assert_allclose_l2(&a, 1e-7);
}

#[test]
fn svd_3x4() {
    let r_dist = RealNormal::new(0., 1.);
    let a = Array::<f64, _>::random((3, 4), r_dist);
    let (u, s, vt) = a.clone().svd().unwrap();
    let mut sm = Array::zeros((3, 4));
    for i in 0..3 {
        sm[(i, i)] = s[i];
    }
    u.dot(&sm).dot(&vt).assert_allclose_l2(&a, 1e-7);
}
#[test]
fn svd_3x4_t() {
    let r_dist = RealNormal::new(0., 1.);
    let a = Array::<f64, _>::random((4, 3), r_dist).reversed_axes();
    let (u, s, vt) = a.clone().svd().unwrap();
    let mut sm = Array::zeros((3, 4));
    for i in 0..3 {
        sm[(i, i)] = s[i];
    }
    u.dot(&sm).dot(&vt).assert_allclose_l2(&a, 1e-7);
}

#[test]
#[ignore]
fn svd_large() {
    let n = 2480;
    let m = 4280;
    let r_dist = RealNormal::new(0., 1.);
    let a = Array::<f64, _>::random((n, m), r_dist);
    let (u, s, vt) = a.clone().svd().unwrap();
    let mut sm = Array::zeros((n, m));
    for i in 0..min(n, m) {
        sm[(i, i)] = s[i];
    }
    u.dot(&sm).dot(&vt).assert_allclose_l2(&a, 1e-7);
}
