### java-poc-1

Spring, maven, native query with grouping sets.
Created with Windsurf on openai o3-mini medium. Didn't work, quickly threw it away.

### java-poc-2

Created with Windsurf on Gemini 2.5 Pro, worked a lot better. Used the same initial prompt.

Make a new Java project in this empty directory with Java 23, Maven and Spring Boot that connects to a Postgres database at localhost:5432 (database=postgres, username=postgres, password=pocpoc) and then automatically creates a table for a Product entity if it doesn't exist; the entity is as follows:

- A Product class with a UUID, three String properties called categoryA, categoryB and categoryC, and a price that's internally stored as an integer amount of cents with getters for obtaining doubles in Java code.

Follow best practices when writing the application, with room in the project for controllers, services and repositories properly separated.

Make a new controller for products for listing products in a GET endpoint, and also add a POST endpoint at "/products/populate" with the following behavior:

- Calling the /products/populate endpoint makes the appliation randomly generate one thousand Product objects and insert them to the database.
- Each randomly generated product has a random price between 1.00 and 100.00; the distribution is not critical (for example, it does not matter much if the distribution isn't perfectly homogeneous, because the dataset is for testing purposes).
- The randomly generated values of categoryA, categoryB and categoryC are all one-character Strings chosen from pre-determined pools:
  - categoryA is a random choice of an uppercase letter of the Latin alphabet, such as "B" or "X".
  - categoryB is a random choice of an uppercase letter of the Greek alphabet, such as "Δ" or "Ψ".
  - categoryC is a random choice of a letter from the Hebrew alphabet, including letters with dagesh, such as "בּ", "שׁ" and "צ".
- As with the random price, the exact rules for the random letters is not critical, as their main purpose is to serve as test data.
- The POST request does not need to return the newly inserted products. It only needs to return 200 when successful and 5xx otherwise.

Considering these rules, the controller should have two endpoints, a GET for listing all products (preferably with pagination) and a POST for generating 1000 entries in the database.

> Did not use dagesh, but that wasn't important.

Now implement a new endpoint at GET /products/by-category that returns the values in the following format:

- One JSON object with four properties:
  - "by_category_a", contaning an array of objects in the form { "category_a": "string", "count": 0, "value": 0 }. Each object in the array groups all entries in the table that have that value of categoryA, so the array is expected to have at most as many elements as there are distinct values of categoryA.
  - Likewise for "by_category_b" and "by_category_c", each containing an array with objects that contain its category (and not the other categories).
  - "total", containing only one object in the form { count: 0, "value": 0 } aggregating over all entries.

Considering that the database is in Postgres, I recommend using grouping sets. You should create any DTOs that are needed, but keep the database DTOs separate from the controller DTOs.

> Results in server error.

It seems a server error is thrown when using the /products/by-category endpoint. It says "[ERROR: relation "product" does not exist
Position: 151] [n/a]; SQL [n/a]] with root cause".
In order to verify the behavior more reliably, please write a new Junit test for the service, testing the getProductStats() function. It's not necessary to verify the exact response, but the test should ensure that the function runs successfully. Less critically (but ideally) the Junit test should verify that the JSON response is created and returned as expected.

> Error. Needed to reset the chat.
> Then, it wrote a unit test that didn't find the problem in the ProductRepository. That's fine, I suppose, since I didn't explicitly ask for an integration test.

Sometimes it just says "done."

### rust-poc-3

Using levenberg-marquadt for non-linear regression.
Gemini 2.5 Pro was really struggling. Tried three times, it always gives up with "done".

Trying with Claude 3.7 (regular). Worked much better, loops correctly attempting to solve the problem and outputs a working solution with one prompt:

Create a new rust project in this directory with the following objective:

**Use the levenberg_marquardt library to write a program that solves non-linear regression problems using least-squares.**

To validate the program, we will use an example non-regression problem defined by:

`f(x, y, z) = a*x + b*y + cat`

For this example, we will use the following observations:

```rust
let observed_x: Vec<f64> = vec![2., 3., 4., 5., 6., 6., 7., 7., 8., 8., 9., 9.];
let observed_y: Vec<f64> = vec![3., 5., 8., 1., 2., 6., 1., 9., 2., 0., 2., 16.];
let observed_f: Vec<f64> = vec![32.2364, 37.6842, 45.0243, 33.4443, 37.0006, 44.5683, 36.7723, 51.9083, 40.3283, 36.5443,41.9923, 68.4803,];
```

When ran, the program should find and output the coefficients a, b, and c that reproduces the observed values of x, y and f, as well as print the residual for each observation.

It's important that the DynamicallySizedProblem struct should be able to solve arbitrary least-squares problems, not just the regression we're using as an example.

I used a second prompt telling it to separate the generic part of the program into a separate file, which it did successfully, and then a third prompt to introduce a categorical variable:

Let's use a different example in the main file now. Instead of the current objective function and observations, we will use the following problem:

f(x, y, z) = (a*x + b*y + c) \* m(z)

Where a, b, c, x and y continue to be numbers, but now we have a categorical variable named z, whose values are ["A", "B", "C" ...]. m(z) produces a multiplier for the rest of the expression, such that there is one unknown multiplier associated with each possible value of the categorical variable z. It is known that the multipliers are values equal to or larger than 1.

The observations are as follows:

- **x**: 3, 4, 4, 3, 5, 4, 5, 6, 6, 7, 8, 6, 7, 10, 15, 12, 14
- **y**: 4, 2, 1, 3, 5, 4, 3, 2, 4, 6, 7, 8, 9, 10, 14, 15, 16
- **z**: B, B, B, A, A, B, C, D, E, E, D, C, D, A, B, C, D
- **f**: 11.5, 9.3, 7.6, 11.76, 18.72, 12.7, 18.3, 11.7, 16.61, 21.67, 22.6, 32.85, 24.8, 36.12, 42.9, 61.5, 45.1

The unknowns are the coefficients a, b and c, as well as each modifier associated with each value of z that appears in the observations. There is an issue with identifiability, which we address with the constraint that the smallest multiplier must be adjusted to 1 after the regression is done.

Replace the current example in main.rs with the one described in this message. Because the code in problem_solver.rs uses least squares to solve arbitrary non-linear problems expressed with an array of residuals, modifications to it are not expected, but fixes may be necessary if you identify unexpected behavior. Let me know if you need more information to proceed.

The code works better than Excel's solver.
