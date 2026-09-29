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
| `./run --eval <file>` | [Evaluates each expression and prints its value.  **Not yet implemented** — scoped for a later lab. ] |
| `./run` | [Starts the REPL.] |

Exit codes: `0` on success,  `64` for incorrect command usage (wrong flags or number of arguments) . `65` when input is invalid (lexical or any syntax error)

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
expression → equality
equality   → comparison ( ( "==" | "!=" ) comparison )*
comparison → term ( ( "<" | "<=" | ">" | ">=" ) term )*
term       → factor ( ( "+" | "-" ) factor )*
factor     → primary ( ( "*" | "/" | "%" ) primary )*
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

[What runtime values exist, and how they are represented in the host
language.]

### Value printing

- Numbers: [e.g. 5 rather than 5.0]
- Nil: [spelling]
- Strings: [with or without quotes]

### Truthiness

[The complete rule. Which values are false in a condition; everything else is
true.]

### Operator semantics

- Arithmetic: [accepted operand types]
- `+` on strings: [concatenation, error, or coercion]
- Mixed types: [what happens]
- Comparison: [accepted operand types]
- Equality across types: [false, or an error]
- Division by zero: [value produced, or runtime error]

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
[one real static error]
[one real runtime error]
```

| Failure | Exit code |
| --- | --- |
| lexical error | 65 |
| syntax error | 65 |
|Incorrect command usage|64|

## Testing conventions

| Folder | Activity | Mode | Flag |
| --- | --- | --- | --- |
| tests/lab1 | Scanner | sidecar | `--tokenize` |
| tests/lab2 | Parser | sidecar | `--parse` |
| tests/lab3 | Evaluator | inline | none |
| tests/lab4 | Context | inline | none |
| tests/lab5 | Functions | inline | none |

```
[specific tests]...
```

Run locally with:

```bash
curl -sSL https://raw.githubusercontent.com/WhiteLicorice/cmsc-124-harness/v1.1/run_tests.py -o run_tests.py
./build.sh
python3 run_tests.py tests/lab1
```

## Sample code

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

## Design rationale

- **Poker vocabulary over generic keywords:** Uses poker terms like `deal`, `fold`, `bust`, and `round` so the code feels like a poker game.

- **`!!!` for block comments instead of `/* */`:** Avoids conflict with `/` for division and keeps comments easy to recognize.

- **Errors accumulate instead of stopping the Scanner:** The Scanner continues after an error so it can report multiple errors at once.

- **Non-nestable block comments:** The first `!!!` closes the comment, keeping the Scanner simple.

- **`~` allowed in identifiers:** Adds a small stylistic feature to make PokerScript names more unique.

## Known limitations

- `./run <file>` and `./run --eval` are not implemented yet, so programs cannot be executed or evaluated.
- `--parse` only supports a single expression. Statements like `set`, `deal`, `call`, `flush`, `fold`, `bet`, `bust`, `round`, and `show` are not parsed yet.
- Assignment (`=`) and logical operators (`and`, `or`, `not`) are recognized but cannot be parsed yet.
- Block comments cannot be nested.

## Changelog

| Activity | What changed in the language |
|---|---|
| Lab 1 | Implemented the Scanner: operators, strings, numbers, identifiers and keywords, `!!`/`!!!` comments, line tracking, `--tokenize` mode, and error reporting with exit code 65. |
