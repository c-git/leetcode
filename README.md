# LeetCode Practice

This repo stores solutions to questions on LeetCode that I did for practice. If you are only looking for "How to Run"
instructions please feel free to skip ahead to the [How to run code](#how-to-run-code) section. Original questions,
examples and constraints can be found on the LeetCode [website](https://leetcode.com). Solutions based on other people's
solutions are cited in the same file usually at the top of the function in a comment.

# Table of Contents

- [Layout](#layout)
- [How to run code](#how-to-run-code)
- [How to add solutions](#how-to-add-solutions)

# Layout

I have created a top level folder for each language that I have solutions for. Then under each language there are some
helper files to provide commonly reused code, these are liable to change as I discover more of what is common among
LeetCode problems. Other than the helper files there is one file per problem in the folder. I tried to keep naming of
the files obvious. Python and Rust do not allow identifiers to start with a number, so filenames start with an
underscore.

# How to run code

I've tried to keep it as simple as possible but also wanted to minimize required repeated work, as a result some steps
are designed with IDE support in mind, like dependency importing (works in PyCharm but auto complete didn't work for me
in VS Code). All python commands are expected to be run from the root folder of the repository where "README.md" is
found. Rust commands are expected to be run from inside the rust folder where the `Cargo.toml` is.

Language:

- [Python](#python-how-to-run)
- [Rust](#rust-how-to-run)

### Python (How to run)

Assumption: Both pip and python 3.6 or greater are already installed.

1. Install dependencies: `pip install -r requirements.txt`
2. Link main.py to problem
    1. Uncomment "tester" function call
    2. Add import for tester from desired problem e.g. `from python3._1_TwoSum import tester`
3. Execute run.py `python3 run.py` Should show output that it was run and how long it took

### Rust (How to run)

Assumption: Latest version of Rust is installed. Should still work with fairly old versions as leet code doesn't upgrade
their rust version often.

1. Link lib.rs to problem (Add file as module. eg `mod _1_two_sum;`).
2. Run tests `cargo test` Should show output that it was run and how long it took

# How to add solutions

Language

- [Python](#python-how-to-add)
- [Rust](#rust-how-to-add)

### Python (How to add)

1. Create file for problem in python3 folder
2. Copy starter code from LeetCode into file (optionally add pass to clear errors)
3. Copy starter tester code into newly created file
   from [main](https://github.com/c-git/leetcode/blob/849c517017586ab0aacd461eaa705395ae1fa58d/python3/main.py#L10) (can
   be identified as commented out as a string at bottom of the file)
4. Import [helper library](https://github.com/c-git/leetcode/blob/main/python3/helper.py)
5. Fill in starter code with problem number and other required parameters (parameters can be seen in helper file).
6. Fill in example test cases
7. Some problems may require input transformation, simply call a function to transform the input so that your solution
   gets the input in the right format e.g. convert an integer list to tree as seen in
   [problem 1302](https://github.com/c-git/leetcode/blob/849c517017586ab0aacd461eaa705395ae1fa58d/python3/_1302_DeepestLeavesSum.py#L37)
8. Some problems allow multiple solutions those often need to be handled on a case by case basis but in general if the
   output is an unordered list you can find an example
   in [problem 47](https://github.com/c-git/leetcode/blob/849c517017586ab0aacd461eaa705395ae1fa58d/python3/_47_PermutationsII.py#L18)
9. Now you can implement your solution and test and debug locally.

### Rust (How to add)

1. Create file for problem in rust folder
2. Copy starter code from LeetCode into file. Add `todo!()` in function body so code compiles.
3. Optionally create a test module at the bottom of the file to put the tests into (usually only useful if the tests
   require imports).
4. Create a test for each example. I usually call them Case1, Case2, Case3 and so on.
5. Some problems may require input transformation, transformations for implemented types usually have a wrapper type and then an into from that wrapper type see example in [problem 2](https://github.com/c-git/leetcode/blob/main/rust/src/_2_add_two_numbers.rs#L46)
6. Some problems allow multiple solutions. I haven't done any of those yet but should be evaluated using a custom evaluation method.
7. Now you can implement your solution and test and debug locally.