# Non-Linear Regression Solver using Levenberg-Marquardt

This Rust project implements a flexible non-linear regression solver using the Levenberg-Marquardt algorithm. It provides a generic `DynamicallySizedProblem` struct that can solve arbitrary least-squares problems by expressing them in terms of residuals and an optional Jacobian matrix.

## Features

- Generic implementation that can handle any non-linear regression problem
- Support for both analytical and numerical Jacobian computation
- Example implementation of a linear regression problem

## Example

The project includes an example that fits the function `f(x, y) = a*x + b*y + c` to a set of observed data points. It demonstrates:

1. How to define a residual function
2. How to define an analytical Jacobian function (optional)
3. How to solve the problem with both analytical and numerical Jacobians

## Usage

To run the example:

```bash
cargo run
```

## Dependencies

- levenberg-marquardt: Implementation of the Levenberg-Marquardt algorithm
- nalgebra: Linear algebra library for matrix operations
- num-traits: Traits for numeric types
- rand: Random number generation
