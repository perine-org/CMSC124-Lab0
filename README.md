# PokerScript

## Creators

- Erine Lourdes L. Medalla (e-rine)
- Percie Louise Y. Samaniego (perky-boo25)

## Overview

**PokerScript** is a beginner-friendly, general-purpose programming language that turns basic programming concepts into poker terms. Variables are **dealt and set**, functions are **called**, returns **flush**, loops **go round**, and errors **bust**.
It is designed for beginners who may find familiar, playful terms easier to understand than traditional keywords like `var`, `return`, or `for`. Instead of feeling like memorizing syntax, writing PokerScript feels more like **narrating a hand of poker**.
Despite the poker theme, the language focuses on **deterministic logic and decision-making, not chance**.

## Host language and build

- Host language: Rust
- Version metadata: 1.98.0
- Build: `./build.sh`
- [Anything a fresh clone needs to know.]

## Running it

| Command | What it does |
| --- | --- |
| `./run <file>` | [Executes a program. Available from Lab 4.] |
| `./run --tokenize <file>` | [Prints the token stream.] |
| `./run --parse <file>` | [Prints the parsed tree.] |
| `./run --eval <file>` | [Evaluates each expression in the file and prints its value, one per line.] |
| `./run` | [Starts the REPL.] |

Exit codes: `0` on success,  `64` for incorrect command usage (wrong flags or number of arguments) . `65` when input is invalid (lexical or any syntax error). `70` when a runtime error happens while evaluating (file mode)

## File extension

`.pkr` 

## Lexical structure

### Keywords

| Keyword | Purpose |
|---|---|
| `set` | Variable declaration |
| `deal` | Immutable variable declaration |
| `call` | Function call |
| `flush` | Return |
| `fold` | Break |
| `bet` | Try |
| `bust` | Catch / exceptions |
| `round` | For loop |
| `show` | Print statement |
| `true` | Boolean literal (true) |
| `false` | Boolean literal (false) |
| `and` | Logical AND |
| `or` | Logical OR |
| `not` | Logical NOT |

### Operators

| Operator | Category | Operands | Associativity | Precedence |
| --- | --- | --- | --- | --- |
| `+` | arithmetic | binary | left | 4 |
| `-` | arithmetic | binary | left | 4 |
| `*` | arithmetic | binary | left | 5 |
| `/` | arithmetic | binary | left | 5 |
| `%` | arithmetic | binary | left | 5 |
| `<` | comparison | binary | left | 3 |
| `<=` | comparison | binary | left | 3 |
| `>` | comparison | binary | left | 3 |
| `>=` | comparison | binary | left | 3 |
| `==` | comparison | binary | left | 2 |
| `!=` | comparison | binary | left | 2 |
| `=` | assignment | binary | right | Not defined |
| `and` | logical | binary | Not defined | Not defined |
| `or` | logical | binary | Not defined | Not defined |
| `not` | logical | unary | Not defined | Not defined |

**Note:** The precedence and associativity for +, -, *, /, %, comparison, and equality operators are based on the grammar implemented in parser.rs. However, =, and, or, and not do not have their precedence or associativity defined in the current grammar.

### Literals

| Kind | Syntax | Produces |
|---|---|---|
| Number| 42, 3.14 | Number |
| String | "hello", escapes not supported | String |
| Booleon | [true, false] | Boolean |
| [nil] | [spelling] | [what runtime value] |

### Identifiers

- Start characters: Alphabetic characters (`a-z`, `A-Z`, and other alphabetic Unicode characters) or `~`
- Continue characters: Alphanumeric characters or `~`
- Case-sensitive: Yes
- Reserved words: `set`, `deal`, `call`, `flush`, `fold`, `bet`, `bust`, `round`, `bluff`, `draw`, `raise`, `show`, `true`, `false`, `and`, `or`, `not`
- No identifier length limit is defined in the Scanner.
- A word matching one of the reserved words is tokenized as its corresponding keyword rather than as an `IDENTIFIER`.
- Identifiers may contain letters, numbers, and `~`, but they cannot begin with a number.
- The language's `IDENTIFIER` token is distinct from its reserved keyword tokens. :contentReference[oaicite:3]{index=3}

### Comments

- Line comments: !!
- Block comments: !!!
- Nesting: [supported or not]
- comment_prefix in tests/lab*/manifest.json is set to !!

## Whitespace and termination

- Whitespace significant: **No.** Spaces, carriage returns (`\r`), and tabs (`\t`) are ignored by the Scanner. Newlines are also not treated as tokens, but they increment the line counter for error reporting.
- Statement terminator: **No explicit statement terminator is defined in the Parser grammar.** A semicolon token exists in the Scanner, but the current expression grammar does not use it.
- Block delimiters: `{` and `}` are recognized as `LEFT_BRACE` and `RIGHT_BRACE`, but they are not currently used by the Parser grammar.
- Grouping delimiters: `(` and `)`. Parentheses are used to group expressions in the grammar: `"(" expression ")"`.
- End of input: The Scanner automatically adds an `EOF` token after scanning the entire source.

## Token output format

```
[Token(type=, lexeme=, literal=, line=)]
```

Fields: token type, lexeme, literal value (or empty), line number.


## Grammar

```
program    → ( expression ";" )* EOF
expression → equality
equality   → comparison ( ( "==" | "!=" ) comparison )*
comparison → term ( ( "<" | "<=" | ">" | ">=" ) term )*
term       → factor ( ( "+" | "-" ) factor )*
factor     → unary ( ( "*" | "/" | "%" ) unary )*
unary      → ( "not" | "-" ) unary | primary
primary    → NUMBER | STRING | "true" | "false" | "(" expression ")"
```

## Parse output format

```
[one line of real --parse output, e.g. (+ 1.0 (* 2.0 3.0))]
```

- Groupings print as: [form]
- Numbers print as: [form]

## Semantics

### Values and types

A runtime value is a tagged union (`enum Value` in `value.rs`) with four cases:
 
| Case | Host representation | Notes |
|---|---|---|
| Number | `f64` | One numeric type. There are no separate integers. |
| String | `String` | |
| Boolean | `bool` | |
| Nil | unit variant | No literal yet. |
 
`Value` is separate from the scanner's `Literal`: a literal is what the source text said, a value is what the program computed.

### Value printing

- Numbers: shortest form, so `5` rather than `5.0`. Both `0` and `-0` print as `0`. `NaN`, `Infinity` and `-Infinity` print by those names. 
- Nil: `nil`
- Strings: without quotes. `"hi"` prints as `hi`.

### Truthiness

Only `false` and `nil` are falsy. Everything else is truthy, including `0`, `0.0` and the empty string `""`. Because there is no `nil` literal yet, truthiness can only be observed through `not`: `not x` is `true` exactly when `x` is falsy.

### Operator semantics

- Arithmetic: `-`, `*`, `/`, `%` accept numbers only. `%` keeps the sign of the left operand (`-7 % 3` is `-1`, `7 % -3` is `1`). Unary `-` accepts a number only.
- `+` on strings: concatenation. `+` accepts two numbers or two strings.
- Mixed types: a runtime error, with no automatic conversion. `"a" + 1` fails with `Operands must be two numbers or two strings.` and `"a" - 1` fails with `Operands must be numbers.`
- Comparison: `<`, `<=`, `>`, `>=` accept numbers only. 
- Equality across types: `false`. `==` and `!=` accept any two values. `1 == "1"` is `false` and `0 == false` is `false`. Same-type values compare by content.
- Division by zero: a runtime error (`Division by zero.`), not `Infinity`. `x % 0` is also a runtime error (`Modulo by zero.`), not `NaN`.
- Evaluation order: left operand, then right operand, and an error in the left is reported first. 

### Scope and bindings

- Redeclaration in the same scope: [allowed or an error]
- Uninitialized variable holds: [value]
- Shadowing: [behavior]
- Undefined name: [static error with exit 65, or runtime error with exit 70]

### Control flow and functions

- Logical operators return: [booleans, or the operand]
- Dangling else binds to: [which if]
- Closure capture of a loop variable: [per iteration, or shared]
- Function with no return statement produces: [value]
- Arity mismatch: [message and exit code]

## Native functions

| Name | Arguments | Returns | Notes |
|---|---|---|---|
| [name] | [count and types] | [type] | [caveats] |

## Errors and diagnostics

Message format:

```
[line 1] Error at ';': Expect ')' after expression.
[line 1] Runtime error: Division by zero.
```

| Failure | Exit code |
| --- | --- |
| lexical error | 65 |
| syntax error | 65 |
| runtime error (file mode) | 70 |
|Incorrect command usage|64|

## Testing conventions

| Folder | Activity | Mode | Flag |
| --- | --- | --- | --- |
| tests/lab1 | Scanner | sidecar | `--tokenize` |
| tests/lab2 | Parser | sidecar | `--parse` |
| tests/lab3 | Evaluator | inline | `--eval` |
| tests/lab4 | Context | inline | none |
| tests/lab5 | Functions | inline | none |

```
Lab 3 tests are inline: the program, its expected output, and its expected exit code live in one `.pkr` file, using `!!` comments such as `!! expect: 7`, `!! expect runtime error: Division by zero.`, and `!! expect error: Expect ')' after expression.`. Every error test holds exactly one error annotation.
```

Run locally with:

```bash
curl -sSL https://raw.githubusercontent.com/WhiteLicorice/cmsc-124-harness/v1.1/run_tests.py -o run_tests.py
./build.sh
python3 run_tests.py tests/lab1
python3 run_tests.py tests/lab2
python3 run_tests.py tests/lab3
```

## Scanner Example

```
set spade = 5
show spade
```

Output:
Output (`./run --tokenize`):
```
Token(type=SET, lexeme=set, literal=null, line=1)
Token(type=IDENTIFIER, lexeme=spade, literal=null, line=1)
Token(type=ASSIGN, lexeme==, literal=null, line=1)
Token(type=NUMBER, lexeme=5, literal=5, line=1)
Token(type=SHOW, lexeme=show, literal=null, line=2)
Token(type=IDENTIFIER, lexeme=spade, literal=null, line=2)
Token(type=EOF, lexeme=, literal=null, line=2)
```

## Parser Example

Input:

```text
2 + 3 * 4;
(2 + 3) * 4;
1 - 2 - 3;
-(4 + 5);
```

Output:

```text
(+ 2.0 (* 3.0 4.0))
(* (group (+ 2.0 3.0)) 4.0)
(- (- 1.0 2.0) 3.0)
(- (group (+ 4.0 5.0)))
```

These outputs show that multiplication happens before addition, parentheses change the grouping, subtraction is left-associative, and unary `-` can be applied to a group.

## Evaluator Example

Input:

```text
2 + 3 * 4;
(2 + 3) * 4;
10 / 4;
"a" + "b";
1 == "1";
not 0;
```

Output (`./run --eval`):

```text
14
20
2.5
ab
false
false
```

A runtime error (`1 / 0;`) prints `[line 1] Runtime error: Division by zero.` on stderr and exits with code 70.

## Design rationale

- **Poker vocabulary over generic keywords:** Uses poker terms like `deal`, `fold`, `bust`, and `round` so the code feels like a poker game.

- **`!!!` for block comments instead of `/* */`:** Avoids conflict with `/` for division and keeps comments easy to recognize.

- **Errors accumulate instead of stopping the Scanner:** The Scanner continues after an error so it can report multiple errors at once.

- **Non-nestable block comments:** The first `!!!` closes the comment, keeping the Scanner simple.

- **`~` allowed in identifiers:** Adds a small stylistic feature to make PokerScript names more unique.

- **Poker terms:** We use poker-related words to make the language more fun and easier to remember.

- **`;` for expressions:** Each expression ends with `;`, which also gives the Parser a clear point to continue from after an error.

- **Prefix parse output:** The operator comes first so the structure of an expression is easy to see.

- **`not` and `-`:** Both are handled by the same unary rule, including repeated operators like `not not true`.
- **Rust enum for the AST:** We use one `Expr` enum for `Literal`, `Grouping`, `Binary`, and `Unary` nodes.

- **Error recovery:** When parsing fails, the Parser skips to the next `;` and continues instead of stopping immediately.

- **Tagged union for values:** `Value` is a Rust enum with one variant per runtime type (number, string, boolean, nil). Matching on the variant is how the evaluator checks types, and the compiler warns about missed cases. We avoided `Box<dyn Any>` with downcasting, which Rust does not need.

- **Division by zero is an error:** Dividing by zero is most likely a mistake. `Infinity` or `NaN` would let it spread silently through later calculations, so we report it where it happens.

- **No automatic conversion in `+`:** `"a" + 1` is an error, because silent conversion hides type mistakes. Strings only concatenate with strings.

- **Errors as `Result`, not panics:** The evaluator returns `Result<Value, RuntimeError>` and uses `?` to unwind. The error carries the operator token so the message includes a line number, and it is reported in one place (`main.rs` for files, `repl.rs` for the REPL).

- **Left-to-right evaluation:** The left operand is always evaluated before the right.

- **Semantics written once:** Truthiness, equality, and number formatting each live in `value.rs` and are shared by file mode and the REPL.

## Known limitations

- `./run <file>` is a placeholder until Lab 4. Use `./run --eval <file>` to evaluate expressions.
- `--parse` only supports a single expression. Statements like `set`, `deal`, `call`, `flush`, `fold`, `bet`, `bust`, `round`, and `show` are not parsed yet.
- Assignment (`=`) and logical operators (`and`, `or`) are recognized but cannot be parsed yet. (`not` works.)
- Block comments cannot be nested.

## Changelog

| Activity | What changed in the language |
|---|---|
| Lab 1 | Implemented the Scanner: operators, strings, numbers, identifiers and keywords, `!!`/`!!!` comments, line tracking, `--tokenize` mode, and error reporting with exit code 65. |
| Lab 2 | Added the Parser, AST, grammar, expression parsing, error recovery, `--parse`, and REPL parsing. |
| Lab 3 | Added the evaluator and `Value` enum, runtime type checking, runtime errors with line numbers and exit code 70, `--eval` mode, and REPL evaluation. Added the inline test folder `tests/lab3`. |