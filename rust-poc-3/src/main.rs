use levenberg_marquardt::{LeastSquaresProblem, LevenbergMarquardt};
use nalgebra::{DMatrix, DVector, Dyn, VecStorage, Matrix, U1};

/// A struct for solving dynamically sized least-squares problems using the Levenberg-Marquardt algorithm.
/// 
/// `Rfn` is a function that produces a vector of residuals.
/// `Jfn` is an optional function that produces a Jacobian matrix. If not provided,
/// the Jacobian will be approximated using finite differences.
#[derive(Clone)]
struct DynamicallySizedProblem<Rfn, Jfn>
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
    fn compute_residuals(&self) -> Option<DVector<f64>> {
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

/// Evaluation function for our example problem
fn eval(x: f64, y: f64, a: f64, b: f64, c: f64) -> f64 {
    a * x + b * y + c
}

fn main() {
    // Example observations
    let observed_x: Vec<f64> = vec![2., 3., 4., 5., 6., 6., 7., 7., 8., 8., 9., 9.];
    let observed_y: Vec<f64> = vec![3., 5., 8., 1., 2., 6., 1., 9., 2., 0., 2., 16.];
    let observed_f: Vec<f64> = vec![32.2364, 37.6842, 45.0243, 33.4443, 37.0006, 44.5683, 36.7723, 51.9083, 40.3283, 36.5443, 41.9923, 68.4803];
    
    // Initial guess for parameters [a, b, c]
    let initial_params = DVector::from_vec(vec![1.0, 1.0, 1.0]);
    
    // Define the residual function
    let residual_fn = |params: &DVector<f64>| {
        let a = params[0];
        let b = params[1];
        let c = params[2];
        
        let mut residuals = DVector::zeros(observed_x.len());
        
        for i in 0..observed_x.len() {
            let x = observed_x[i];
            let y = observed_y[i];
            let f_observed = observed_f[i];
            
            // Compute the predicted value
            let f_predicted = eval(x, y, a, b, c);
            
            // Compute the residual (observed - predicted)
            residuals[i] = f_observed - f_predicted;
        }
        
        Some(residuals)
    };
    
    // Define the Jacobian function (optional, but provided for better performance)
    let jacobian_fn = |params: &DVector<f64>| {
        let n_observations = observed_x.len();
        let n_params = 3; // a, b, c
        
        let mut jacobian = DMatrix::zeros(n_observations, n_params);
        
        for i in 0..n_observations {
            let x = observed_x[i];
            let y = observed_y[i];
            
            // Partial derivatives of the residual with respect to each parameter
            // residual = f_observed - (a*x + b*y + c)
            // d(residual)/da = -x
            // d(residual)/db = -y
            // d(residual)/dc = -1
            jacobian[(i, 0)] = -x;
            jacobian[(i, 1)] = -y;
            jacobian[(i, 2)] = -1.0;
        }
        
        Some(jacobian)
    };
    
    // Create the problem with the Jacobian function
    let mut problem_with_jacobian = DynamicallySizedProblem::new(
        initial_params.clone(),
        residual_fn.clone(),
        Some(jacobian_fn.clone()),
    );
    
    // Solve the problem
    println!("----- Solving with provided Jacobian -----");
    match problem_with_jacobian.solve() {
        Ok((params, evaluations)) => {
            println!("Solution found after {} evaluations:", evaluations);
            println!("a = {:.6}", params[0]);
            println!("b = {:.6}", params[1]);
            println!("c = {:.6}", params[2]);
            
            // Compute final residuals
            if let Some(residuals) = problem_with_jacobian.compute_residuals() {
                println!("\nResiduals:");
                for i in 0..residuals.len() {
                    println!("Observation {}: x = {}, y = {}, f_observed = {}, residual = {:.6}",
                             i, observed_x[i], observed_y[i], observed_f[i], residuals[i]);
                }
                
                // Compute sum of squared residuals
                let ssr: f64 = residuals.iter().map(|r| r * r).sum();
                println!("\nSum of squared residuals: {:.6}", ssr);
                
                // Verify the solution by computing predicted values
                println!("\nVerifying solution:");
                for i in 0..observed_x.len() {
                    let x = observed_x[i];
                    let y = observed_y[i];
                    let f_observed = observed_f[i];
                    let f_predicted = eval(x, y, params[0], params[1], params[2]);
                    println!("Observation {}: x = {}, y = {}, f_observed = {}, f_predicted = {:.6}, diff = {:.6}",
                             i, x, y, f_observed, f_predicted, f_observed - f_predicted);
                }
            }
        },
        Err(e) => println!("Error solving the problem: {}", e),
    }
    
    // Create the problem without the Jacobian function (using numerical approximation)
    let mut problem_without_jacobian = DynamicallySizedProblem::new(
        initial_params,
        residual_fn,
        None::<fn(&DVector<f64>) -> Option<DMatrix<f64>>>,
    ).with_fd_step_size(1e-8);
    
    // Solve the problem
    println!("\n----- Solving with numerical Jacobian approximation -----");
    match problem_without_jacobian.solve() {
        Ok((params, evaluations)) => {
            println!("Solution found after {} evaluations:", evaluations);
            println!("a = {:.6}", params[0]);
            println!("b = {:.6}", params[1]);
            println!("c = {:.6}", params[2]);
            
            // Compute final residuals
            if let Some(residuals) = problem_without_jacobian.compute_residuals() {
                println!("\nResiduals:");
                for i in 0..residuals.len() {
                    println!("Observation {}: x = {}, y = {}, f_observed = {}, residual = {:.6}",
                             i, observed_x[i], observed_y[i], observed_f[i], residuals[i]);
                }
                
                // Compute sum of squared residuals
                let ssr: f64 = residuals.iter().map(|r| r * r).sum();
                println!("\nSum of squared residuals: {:.6}", ssr);
                
                // Verify the solution by computing predicted values
                println!("\nVerifying solution:");
                for i in 0..observed_x.len() {
                    let x = observed_x[i];
                    let y = observed_y[i];
                    let f_observed = observed_f[i];
                    let f_predicted = eval(x, y, params[0], params[1], params[2]);
                    println!("Observation {}: x = {}, y = {}, f_observed = {}, f_predicted = {:.6}, diff = {:.6}",
                             i, x, y, f_observed, f_predicted, f_observed - f_predicted);
                }
            }
        },
        Err(e) => println!("Error solving the problem: {}", e),
    }
}
