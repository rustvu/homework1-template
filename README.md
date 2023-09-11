# RustVU (CS 3891) - Homework 1

This assignment contains several smaller programming exercises w.r.t basic concepts in Rust.

The source code of these exercises is in the files under the `src/` folder. You job is to make sure that each of them compiles and builds properly and passes all of the built-in tests (some exercises only need to be fixed to pass the compiler, some others are also tested against the expected functionality).

Try to solve the exercises with minimum changes and additions to the existing code. I placed `// TODO` comments in the code where I expect you make changes. The test code is clearly marked with a `// DO NOT EDIT BELOW THIS LINE` comment. This should be evident: changing the test code is a (not too intelligent) way of cheating. I will handle any such attempts accordingly. However, you are allowed and encouraged to look at the test code to better understand what is expected from you.

## Use

You can always check your work with `cargo test`.

- If you run `cargo test` in the project folder it will tell you all compilation and testing problems you still need to solve.
- You can also run `cargo test --bin <exercise>`, where `<exercise>` is one of the exercise names to focus on the results of that particular exercise, only.

Some exercises contain a _dummy_ `main()` function, where you can use this function for your own needs (you can see this in the comments I left in the relevant files). It is up to you if and how you want to use these for your development work. You can use `cargo run --bin <exercise>` to see your the execution results of you `main()` function. The only important requirement is that these should not break the compilation/build process.

## Grading

Make sure you __commit__ and __push__ your assignment repository once you manage to run `cargo test` without any errors or warnings.
The homework is graded by assignments (no partial credits are given for incomplete exercises):

|Exercise   | Points|
|-----------|-------|
|variables  |    10 |
|basic_types|    20 |
|functions  |    20 |
|ownership  |    20 |
|euler      |    30 |

Once you __push__ your solution to the repository, GitHub Classroom will run the automated test. I highly recommend to [verify your results of this CI/CD worflow](https://docs.github.com/en/education/manage-coursework-with-github-classroom/learn-with-github-classroom/view-autograding-results) - I use these results for grading your work.