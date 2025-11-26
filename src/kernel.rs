use num_complex::{Complex, Complex64};
use std::f64::consts::PI;

// Kernels saved as traits for independence
// all kernels will return Complex64 because is easier than going through a general scalar
pub trait Kernel<const D: usize> { 
    fn eval(&self, x: &[f64; D], y: &[f64; D]) -> Complex64; // generic 2 point eval returning a Complex64
} // traits mean i can just call .eval(x,y) no matter the kernel or dimension
// as is, requires eval functions to return Complex64

// ---------------- LAPLACE KERNEL ----------------------

// main kernel public structures (keeping it 2D for now)
pub struct Laplace; // simple 2D Laplace

// implementing Kernel trait for 2D laplace struct
impl Kernel<2> for Laplace {

    // Green function eval method
    fn eval( &self, x: &[f64; 2], y: &[f64; 2]) -> Complex64 {
        let dx = x[0] - y[0];
        let dy = x[1] - y[1];
        let r2 = dx*dx + dy*dy;
        // will need better x=y handling but for now call it e-15
        let r = r2.max(1e-15).sqrt();
        Complex { re: (- (1.0 / (2.0 * std::f64::consts::PI)) * r.ln() ), im: 0.0 }
    }
}

// implementing Kernel trait for 3D laplace struct
impl Kernel<3> for Laplace {

    // Green function eval method
    fn eval( &self, x: &[f64; 3], y: &[f64; 3]) -> Complex64 {
        let dx: f64 = x[0] - y[0];
        let dy: f64 = x[1] - y[1];
        let r2: f64 = dx*dx + dy*dy;
        // will need better x=y handling but for now call it e-15
        let r = r2.max(1e-15).sqrt();
        Complex { re: (- (1.0 / (2.0 * std::f64::consts::PI)) * r.ln() ), im: 0.0 } // fix this
    }
}

// ------------------ HELMHOLTZ KERNELS ------------------

//copy and pasted but 2 eval methods for the 2 separate kernels now

// ----- Standard ---------

pub struct Helmholtz { pub wavenumber: f64}

// new method for ease of setting k -- eg. Helmholtz::new(3.02)
impl Helmholtz {
    pub fn new(wavenumber: f64) -> Self { Self {wavenumber}}
    }

// implementing Kernel trait for Helmholtz
impl Kernel<2> for Helmholtz {

    // Green function eval method -- probably need to add new trait about G or dG 
    fn eval( &self, x: &[f64; 2], y: &[f64; 2]) -> Complex64 {
        // bog standard
        let dx = x[0] - y[0];
        let dy = x[1] - y[1];
        let r2 = dx*dx + dy*dy;
        let r = r2.max(1e-15).sqrt();

        // kr and hankel stuff needed
        // let kr = Self.k * r;
        // let h0 = hankel0_1(kr); find the fast hankel crate and implement 

        Complex64::i() * PI * r * 0.25 
    }
}

// ------- Normal Derivative ----------

pub struct HelmholtzNormal { pub wavenumber: f64} // 2D normal derivative 

// new method for ease of setting k -- eg. Helmholtz::new(3.02)
impl HelmholtzNormal {
    pub fn new(wavenumber: f64) -> Self { Self {wavenumber}}
    } // only really important for multiple calls per k

// implementing Kernel trait for Helmholtz
impl Kernel<2> for HelmholtzNormal {

    // Green function eval method -- probably need to add new trait about G or dG 
    fn eval( &self, x: &[f64; 2], y: &[f64; 2]) -> Complex64 {
        // bog standard
        let dx = x[0] - y[0];
        let dy = x[1] - y[1];
        let r2 = dx*dx + dy*dy;
        let r = r2.max(1e-15).sqrt();

        // kr and hankel stuff needed
        // let kr = Self.k * r;
        // let h0 = hankel0_1(kr); find the fast hankel crate and implement 

        Complex64::i() * PI * r * 0.25 
        
    }
}

// Generalised greens function stuff for calling 
pub enum GreensFunction {
    Laplace,
    Helmholtz,
    HelmholtzNormal
} 

