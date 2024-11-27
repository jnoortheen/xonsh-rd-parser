# Development Notes

- check test at [test](./src/parser.rs:73) and update the source code
- By analysing the incomplete parse tree, token stream, and the errors thrown by the parser,
  we can identify which `parse_*` method fails.
- This is a recursive descent parser, so the `parse_*` methods are called in a top-down manner.
- The `parse_*` methods are implemented in the same order as the grammar rules in the [Python grammar].
