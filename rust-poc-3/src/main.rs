mod problem_solver;

use std::collections::HashMap;
use nalgebra::DVector;
use problem_solver::DynamicallySizedProblem;

/// Evaluation function for our example problem
/// f(x, y, z) = (a*x + b*y + c) * m(z)
/// where m(z) is a multiplier associated with the categorical variable z
fn eval(x: f64, y: f64, z: &str, a: f64, b: f64, c: f64, multipliers: &HashMap<String, f64>) -> f64 {
    let base_value = a * x + b * y + c;
    let multiplier = multipliers.get(z).unwrap_or(&1.0);
    base_value * multiplier
}

fn main() {
    // Example observations with categorical variable z
    let observed_x: Vec<f64> = vec![3., 4., 4., 3., 5., 4., 5., 6., 6., 7., 8., 6., 7., 10., 15., 12., 14.];
    let observed_y: Vec<f64> = vec![4., 2., 1., 3., 5., 4., 3., 2., 4., 6., 7., 8., 9., 10., 14., 15., 16.];
    let observed_z: Vec<String> = vec![
        "B".to_string(), "B".to_string(), "B".to_string(), "A".to_string(), "A".to_string(),
        "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string(), "E".to_string(),
        "D".to_string(), "C".to_string(), "D".to_string(), "A".to_string(), "B".to_string(),
        "C".to_string(), "D".to_string()
    ];
    let observed_f: Vec<f64> = vec![
        11.5, 9.3, 7.6, 11.76, 18.72, 12.7, 18.3, 11.7, 16.61, 21.67,
        22.6, 32.85, 24.8, 36.12, 42.9, 61.5, 45.1
    ];
    
    // Get unique categories and create parameter indices for them
    let mut unique_categories: Vec<String> = observed_z.clone();
    unique_categories.sort();
    unique_categories.dedup();
    
    // Create a mapping from category to parameter index
    let category_indices: HashMap<String, usize> = unique_categories.iter()
        .enumerate()
        .map(|(i, cat)| (cat.clone(), i + 3)) // +3 because first 3 params are a, b, c
        .collect();
    
    // Initial guess for parameters [a, b, c, multiplier_A, multiplier_B, ...]
    // We start with all multipliers at 1.0
    let mut initial_params_vec = vec![1.0, 1.0, 1.0]; // a, b, c
    for _ in 0..unique_categories.len() {
        initial_params_vec.push(1.0); // Initial multipliers
    }
    let initial_params = DVector::from_vec(initial_params_vec);
    
    println!("Unique categories: {:?}", unique_categories);
    println!("Category indices: {:?}", category_indices);
    println!("Initial parameters: {:?}", initial_params);
    
    // Clone data for closures
    let observed_x_clone1 = observed_x.clone();
    let observed_y_clone1 = observed_y.clone();
    let observed_z_clone1 = observed_z.clone();
    let observed_f_clone1 = observed_f.clone();
    let category_indices_clone1 = category_indices.clone();
    
    // Define the residual function
    let residual_fn = move |params: &DVector<f64>| {
        let a = params[0];
        let b = params[1];
        let c = params[2];
        
        // Create multipliers map from parameters
        let mut multipliers = HashMap::new();
        for (cat, &idx) in &category_indices_clone1 {
            multipliers.insert(cat.clone(), params[idx]);
        }
        
        let mut residuals = DVector::zeros(observed_x_clone1.len());
        
        for i in 0..observed_x_clone1.len() {
            let x = observed_x_clone1[i];
            let y = observed_y_clone1[i];
            let z = &observed_z_clone1[i];
            let f_observed = observed_f_clone1[i];
            
            // Compute the predicted value
            let f_predicted = eval(x, y, z, a, b, c, &multipliers);
            
            // Compute the residual (observed - predicted)
            residuals[i] = f_observed - f_predicted;
        }
        
        Some(residuals)
    };
    
    // Clone data for Jacobian function
    let observed_x_clone2 = observed_x.clone();
    let observed_y_clone2 = observed_y.clone();
    let observed_z_clone2 = observed_z.clone();
    let category_indices_clone2 = category_indices.clone();
    
    // Define the Jacobian function
    let jacobian_fn = move |params: &DVector<f64>| {
        let a = params[0];
        let b = params[1];
        let c = params[2];
        
        // Create multipliers map from parameters
        let mut multipliers = HashMap::new();
        for (cat, &idx) in &category_indices_clone2 {
            multipliers.insert(cat.clone(), params[idx]);
        }
        
        let n_observations = observed_x_clone2.len();
        let n_params = params.len();
        
        let mut jacobian = nalgebra::DMatrix::zeros(n_observations, n_params);
        
        for i in 0..n_observations {
            let x = observed_x_clone2[i];
            let y = observed_y_clone2[i];
            let z = &observed_z_clone2[i];
            let multiplier = *multipliers.get(z).unwrap_or(&1.0);
            
            // Partial derivatives of the residual with respect to each parameter
            // residual = f_observed - ((a*x + b*y + c) * multiplier(z))
            // d(residual)/da = -x * multiplier(z)
            // d(residual)/db = -y * multiplier(z)
            // d(residual)/dc = -1 * multiplier(z)
            jacobian[(i, 0)] = -x * multiplier;
            jacobian[(i, 1)] = -y * multiplier;
            jacobian[(i, 2)] = -multiplier;
            
            // Partial derivatives with respect to the multipliers
            // For the multiplier corresponding to the current z value:
            // d(residual)/d(multiplier(z)) = -(a*x + b*y + c)
            if let Some(&idx) = category_indices_clone2.get(z) {
                let base_value = a * x + b * y + c;
                jacobian[(i, idx)] = -base_value;
            }
        }
        
        Some(jacobian)
    };
    
    // Create the problem with the Jacobian function
    let mut problem = DynamicallySizedProblem::new(
        initial_params.clone(),
        residual_fn,
        Some(jacobian_fn),
    );
    
    // Solve the problem
    println!("\n----- Solving with provided Jacobian -----");
    match problem.solve() {
        Ok((params, evaluations)) => {
            println!("Solution found after {} evaluations:", evaluations);
            println!("a = {:.6}", params[0]);
            println!("b = {:.6}", params[1]);
            println!("c = {:.6}", params[2]);
            
            // Extract multipliers
            let mut multipliers = HashMap::new();
            for (cat, &idx) in &category_indices {
                multipliers.insert(cat.clone(), params[idx]);
            }
            
            // Find the smallest multiplier for normalization
            let min_multiplier = multipliers.values().fold(f64::INFINITY, |a, &b| a.min(b));
            
            // Print normalized multipliers
            println!("\nMultipliers (normalized so smallest = 1.0):");
            for cat in &unique_categories {
                let multiplier = multipliers.get(cat).unwrap_or(&1.0);
                let normalized = multiplier / min_multiplier;
                println!("{}: {:.6}", cat, normalized);
            }
            
            // Compute final residuals
            if let Some(residuals) = problem.compute_residuals() {
                println!("\nResiduals:");
                for i in 0..residuals.len() {
                    println!("Observation {}: x = {}, y = {}, z = {}, f_observed = {}, residual = {:.6}",
                             i, observed_x[i], observed_y[i], observed_z[i], observed_f[i], residuals[i]);
                }
                
                // Compute sum of squared residuals
                let ssr: f64 = residuals.iter().map(|r| r * r).sum();
                println!("\nSum of squared residuals: {:.6}", ssr);
                
                // Verify the solution by computing predicted values
                println!("\nVerifying solution with the same parameters used in optimization:");
                
                for i in 0..observed_x.len() {
                    let x = observed_x[i];
                    let y = observed_y[i];
                    let z = &observed_z[i];
                    let f_observed = observed_f[i];
                    
                    // Use the same multipliers as in optimization
                    let f_predicted = eval(x, y, z, params[0], params[1], params[2], &multipliers);
                    
                    println!("Observation {}: x = {}, y = {}, z = {}, f_observed = {}, f_predicted = {:.6}, diff = {:.6}",
                             i, x, y, z, f_observed, f_predicted, f_observed - f_predicted);
                }
                
                // Now verify with normalized multipliers
                println!("\nVerifying solution with normalized multipliers (smallest = 1.0):");
                
                // Normalize multipliers for verification
                let mut normalized_multipliers = HashMap::new();
                for (cat, &val) in &multipliers {
                    normalized_multipliers.insert(cat.clone(), val / min_multiplier);
                }
                
                // Adjust a, b, c to compensate for the normalization
                let normalized_a = params[0] * min_multiplier;
                let normalized_b = params[1] * min_multiplier;
                let normalized_c = params[2] * min_multiplier;
                
                for i in 0..observed_x.len() {
                    let x = observed_x[i];
                    let y = observed_y[i];
                    let z = &observed_z[i];
                    let f_observed = observed_f[i];
                    
                    // Use normalized multipliers and adjusted coefficients
                    let f_predicted = eval(x, y, z, normalized_a, normalized_b, normalized_c, &normalized_multipliers);
                    
                    println!("Observation {}: x = {}, y = {}, z = {}, f_observed = {}, f_predicted = {:.6}, diff = {:.6}",
                             i, x, y, z, f_observed, f_predicted, f_observed - f_predicted);
                }
                
                println!("\nNormalized coefficients (adjusted for multiplier normalization):");
                println!("a = {:.6}", normalized_a);
                println!("b = {:.6}", normalized_b);
                println!("c = {:.6}", normalized_c);
            }
        },
        Err(e) => println!("Error solving the problem: {}", e),
    }
}
