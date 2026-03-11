use distances::vectors::euclidean;
use num_complex::{Complex, Complex64};
use std::f64::consts::{PI,E};
use scilib::math::bessel::*; // hankel function
use num_complex::ComplexFloat; // complex exponents

// Kernels saved as traits for independence
pub trait Kernel<const D: usize> { 
    fn eval(&self, x: &[f64; D], y: &[f64; D]) -> Complex64; // require generic 2 point eval returning a Complex64
}

// ---------------- LAPLACE KERNEL ----------------------
pub struct Laplace; 

impl<const D: usize> Kernel<D> for Laplace {

    fn eval( &self, x: &[f64; D], y: &[f64; D]) -> Complex64 {
        
        let temp_r: f64 = euclidean(x, y);
        let r: f64 = temp_r.max(1e-15); // must find neater way of dealing with r=0

        // Laplace Green's functions for 2 and 3 dimensions
        if D == 2 { return Complex { re:- (1.0 / (2.0 * std::f64::consts::PI)) * r.ln(), im: 0.0}}
        if D == 3 { return Complex64 { re: 1.0 / (4.0 * std::f64::consts::PI * r), im: 0.0 }}
        else { panic!()} // must be better way of doing this, maybe in Nodes new impl
    }
}

// ------------------ HELMHOLTZ KERNELS ------------------

// ----- Standard ---------

pub struct Helmholtz { pub wavenumber: f64}

// "new" method for ease of setting k -- eg. HK = Helmholtz::new(3.02), then can call HK wherever down the line
impl Helmholtz { pub fn new(wavenumber: f64) -> Self { Self {wavenumber}}}

impl<const D: usize> Kernel<D> for Helmholtz {

    fn eval( &self, x: &[f64; D], y: &[f64; D]) -> Complex64 {
        
        let temp_r: f64 = euclidean(x, y);
        let r: f64 = temp_r.max(1e-15); // must find neater way of dealing with r=0

        // Helmholtz Green's functions for 2 and 3 dimensions
        if D == 2 {
            let kr: Complex<f64> = Complex64 {re: self.wavenumber * r, im: 0.0};
            let h0: Complex<f64> = h1_nu(0.0,kr); 
            return (Complex64::i()/4.0) * h0 
        }
        if D == 3 {
            let ikr: Complex<f64> = Complex64 {re: 0.0, im: self.wavenumber * r};
            let exponent: Complex<f64> = E.powc(ikr);
            return - (1.0/(4.0 * PI * r)) * exponent
        }
        else { panic!()} // must be better way of doing this, maybe in Nodes new impl
    }
}
