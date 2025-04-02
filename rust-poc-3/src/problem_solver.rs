use levenberg_marquardt::{LeastSquaresProblem, LevenbergMarquardt};
use nalgebra::{DMatrix, DVector, Dyn, VecStorage, Matrix, U1};

/// A struct for solving dynamically sized least-squares problems using the Levenberg-Marquardt algorithm.
/// 
/// `Rfn` is a function that produces a vector of residuals.
/// `Jfn` is an optional function that produces a Jacobian matrix. If not provided,
/// the Jacobian will be approximated using finite differences.
#[derive(Clone)]
pub struct DynamicallySizedProblem<Rfn, Jfn>
where
    Rfn: Fn(&DVector<f64>) -> Option<DVector<f64>> + Clone,
    Jfn: Fn(&DVector<f64>) -> Option<DMatrix<f64>> + Clone,
{
    /// Parameter values
    params: DVector<f64>,
    
    /// Function to compute residuals
    residual_fn: Rfn,
    
    /// Optional function to compute Jacobian
    jacobian_fn: Option<Jfn>,
    
    /// Finite difference step size for numerical Jacobian approximation
    fd_step_size: f64,
}

impl<Rfn, Jfn> DynamicallySizedProblem<Rfn, Jfn>
where
    Rfn: Fn(&DVector<f64>) -> Option<DVector<f64>> + Clone,
    Jfn: Fn(&DVector<f64>) -> Option<DMatrix<f64>> + Clone,
{
    /// Create a new problem with initial parameters and residual function.
    /// The Jacobian function is optional.
    pub fn new(initial_params: DVector<f64>, residual_fn: Rfn, jacobian_fn: Option<Jfn>) -> Self {
        Self {
            params: initial_params,
            residual_fn,
            jacobian_fn,
            fd_step_size: 1e-6,
        }
    }
    
    /// Set the finite difference step size for numerical Jacobian approximation
    pub fn with_fd_step_size(mut self, step_size: f64) -> Self {
        self.fd_step_size = step_size;
        self
    }
    
    /// Solve the least-squares problem
    pub fn solve(&mut self) -> Result<(DVector<f64>, usize), String> {
        let lm = LevenbergMarquardt::new();
        
        // Clone self because minimize takes ownership
        let problem = self.clone();
        let (problem_result, report) = lm.minimize(problem);
        
        // Update our parameters with the solution
        self.params = problem_result.params.clone();
        
        Ok((self.params.clone(), report.number_of_evaluations))
    }
    
    /// Compute residuals for the current parameters
    pub fn compute_residuals(&self) -> Option<DVector<f64>> {
        (self.residual_fn)(&self.params)
    }
    
    /// Compute residuals for specific parameters
    fn compute_residuals_for(&self, params: &DVector<f64>) -> Option<DVector<f64>> {
        (self.residual_fn)(params)
    }
    
    /// Approximate the Jacobian numerically using finite differences
    fn approximate_jacobian(&self) -> Option<DMatrix<f64>> {
        let residuals = match self.compute_residuals() {
            Some(r) => r,
            None => return None,
        };
        
        let n_params = self.params.len();
        let n_residuals = residuals.len();
        
        let mut jacobian = DMatrix::zeros(n_residuals, n_params);
        
        for i in 0..n_params {
            let mut perturbed_params = self.params.clone();
            perturbed_params[i] += self.fd_step_size;
            
            let perturbed_residuals = match self.compute_residuals_for(&perturbed_params) {
                Some(r) => r,
                None => return None,
            };
            
            for j in 0..n_residuals {
                jacobian[(j, i)] = (perturbed_residuals[j] - residuals[j]) / self.fd_step_size;
            }
        }
        
        Some(jacobian)
    }
}

impl<Rfn, Jfn> LeastSquaresProblem<f64, Dyn, Dyn> for DynamicallySizedProblem<Rfn, Jfn>
where
    Rfn: Fn(&DVector<f64>) -> Option<DVector<f64>> + Clone,
    Jfn: Fn(&DVector<f64>) -> Option<DMatrix<f64>> + Clone,
{
    type ResidualStorage = VecStorage<f64, Dyn, U1>;
    type JacobianStorage = VecStorage<f64, Dyn, Dyn>;
    type ParameterStorage = VecStorage<f64, Dyn, U1>;
    
    fn residuals(&self) -> Option<Matrix<f64, Dyn, U1, Self::ResidualStorage>> {
        self.compute_residuals()
    }
    
    fn jacobian(&self) -> Option<Matrix<f64, Dyn, Dyn, Self::JacobianStorage>> {
        match &self.jacobian_fn {
            Some(jac_fn) => jac_fn(&self.params),
            None => self.approximate_jacobian(),
        }
    }
    
    fn params(&self) -> Matrix<f64, Dyn, U1, Self::ParameterStorage> {
        self.params.clone()
    }
    
    fn set_params(&mut self, params: &Matrix<f64, Dyn, U1, Self::ParameterStorage>) {
        self.params = params.clone();
    }
}
