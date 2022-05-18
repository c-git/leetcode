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

I have created a top level folder for each language in which I have solutions. I am learning rust and intend to use this
as a way to explore the language. Then under each language there are some helper files to provide commonly reused code,
these are liable to change as I discover more of what is common among LeetCode problems.
Other than the helper files there is one file per problem in the folder. I tried to keep naming of the files obvious.
The format may need to be language specific. For python the name cannot start with a number so an it is prefixed with an
underscore then the problem number then an underscore followed by the name with all spaces removed.

# How to run code

I've tried to keep it as simple as possible but also wanted to minimize required repeated work, as a result some steps
are designed with IDE support in mind, like dependency importing. All commands are expected to be run from the root
folder of the repository where "README.md" is found.

Language:

- [Python](#python-how-to-run)

### Python (How to run)

Assumption: Both pip and python 3.6 or greater are already installed.

1. Install dependencies: `pip install -r requirements.txt`
2. Link main.py to problem
    1. Uncomment "tester" function call
    2. Add import for tester from desired problem e.g. `from python3._1_TwoSum import tester`
3. Execute run.py `python3 run.py` Should show output that it was run and how long it took

# How to add solutions

Language

- [Python](#python-how-to-add)

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