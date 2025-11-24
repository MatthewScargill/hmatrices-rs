use num_complex::Complex64;
use std::f64::consts::PI;

// Kernels saved as traits for independence
// D is for dimension of the Kernel, will probably stick to 2 but want to get used to this conceptually
pub trait Kernel<const D: usize> { 
    type Scalar; // return type
    fn eval(&self, x: &[f64; D], y: &[f64; D]) -> Self::Scalar; // generic 2 point eval returning a Scalar
} // traits mean i can just call ::eval(x,y) no matter the kernel or dimension


// ---------------- LAPLACE KERNEL ----------------------

// main kernel public structures (keeping it 2D for now)
pub struct Laplace2D; // simple 2D Laplace

// implementing Kernel trait for Laplace2D struct
impl Kernel<2> for Laplace2D {
    type Scalar = f64; // Laplace stays real so f64

    // Green function eval method
    fn eval( &self, x: &[f64; 2], y: &[f64; 2]) -> f64 {
        let dx = x[0] - y[0];
        let dy = x[1] - y[1];
        let r2 = dx*dx + dy*dy;
        // will need better x=y handling but for now call it e-15
        let r = r2.max(1e-15).sqrt();
        - (1.0 / (2.0 * std::f64::consts::PI)) * r.ln() 
    }
}


// ------------------ HELMHOLTZ KERNELS ------------------

//copy and pasted but 2 eval methods for the 2 separate kernels now

// ----- Standard ---------

pub struct Helmholtz2D { pub wavenumber: f64} // standard 2D Greens function

// new method for ease of setting k -- eg. Helmholtz2D::new(3.02)
impl Helmholtz2D {
    pub fn new(wavenumber: f64) -> Self { Self {wavenumber}}
    }


// implementing Kernel trait for Helmholtz2D
impl Kernel<2> for Helmholtz2D {
    type Scalar = Complex64; // This one needs to be complex

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

pub struct HelmholtzNormal2D { pub wavenumber: f64} // 2D normal derivative 

// new method for ease of setting k -- eg. Helmholtz2D::new(3.02)
impl HelmholtzNormal2D {
    pub fn new(wavenumber: f64) -> Self { Self {wavenumber}}
    }

// implementing Kernel trait for Helmholtz2D
impl Kernel<2> for HelmholtzNormal2D {
    type Scalar = Complex64; // This one needs to be complex

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
    Laplace2D,
    Helmholtz2D,
} 

impl Kernel<2> for GreensFunction {
    match type {
        Laplace2D => type f64;
        Helmholtz2D => type Complex64;
    }

    fn eval(&self, x: &[f64; 2], y: &[f64; 2]) {
        match self {
            GreensFunction::Laplace2D => 0 ;
            GreensFunction::Helmholtz2D => 1;
        }
    }
}
