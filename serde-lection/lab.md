# Assigment

## Serde

Write a program that deserializes [the following JSON](https://github.com/TheBestTvarynka/trash-code/blob/serde-lection/serde-lection/request.json) into a static `Request` type and prints out its serialization in _YAML_ and _TOML_ formats. Consider to choose **correct types for data representation**.

Prove your implementation correctness with tests.

## Parsing

1. Create a grammar for parsing arithmetic expressions. The grammar should support the following:
  - Integer numbers.
  - Ariphmetic operator: `-`, `+`, `*`, `/`, `^` (exponentiation).
  - Parentheses: `(`, `)`.
2. Implement this grammar. You can use parser generators or parser combinators. Pick any Rust library you want.
3. Wrap the grammar implementation into a small program that accepts arithmetic expression as the input and prints the calculated result. Exampe:
  ```bash
  > ./my_app "4+5*2"
  14
  >
  ```
  ```bash
  > ./my_app "3+2^(4+2)-6"
  61
  >
  ```

Important notes:

- You are **not allowed** to hardcode grammar rules using tons of `if-else` statements.
- You are **not allowed** to use [Reverse Polish Notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation).
