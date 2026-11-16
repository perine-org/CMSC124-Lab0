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
- A fresh clone needs the Rust toolchain (`cargo`) installed. Run `./build.sh` once to produce the `./run` executable. 

## Running it

| Command | What it does |
| --- | --- |
| `./run <file>` | Executes a program. Statements run in order and only `show` produces output. |
| `./run --tokenize <file>` | Prints the token stream. |
| `./run --parse <file>` | Prints the parsed tree, one line per statement. |
| `./run --eval <file>` | Runs the program and also prints the value of every bare expression statement, one per line. |
| `./run` | Starts the REPL. Variables persist across lines and bare expressions print their value. |

Exit codes: `0` on success. `64` for incorrect command usage (wrong flags or number of arguments). `65` when input is rejected before running (unreadable file, lexical error, or any syntax error), and nothing is executed. `70` when a runtime error happens while running a file. In the REPL a runtime error is printed and the prompt returns; the session's variables survive.

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
| `bluff`, `draw`, `raise` | Reserved for future use (not parsed yet) |
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
| `=` | assignment | binary | right | 1 |
| `and` | logical | binary | Not defined | Not defined |
| `or` | logical | binary | Not defined | Not defined |
| `not` | logical | unary | right | 6 |
| `-` (unary) | arithmetic | unary | right | 6 |

**Note:** Precedence and associativity for `+`, `-`, `*`, `/`, `%`, comparison and equality come from the grammar in parser.rs. `not` is unary, at the same level as unary `-` (right-associative, binds tighter than `*`). `=` is parsed as right-associative assignment, so `a = b = 5;` works, and its left side must be a variable (otherwise `Invalid assignment target.`). 

### Literals

| Kind | Syntax | Produces |
|---|---|---|
| Number| 42, 3.14 | Number |
| String | "hello", escapes not supported | String |
| Boolean | true, false | Boolean |
| nil | no literal | Nil (produced by `set a;`, which holds `nil` until assigned) |

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
- Nesting: not supported. The first `!!!` after an opening `!!!` closes the comment.
- A lone `!` (not `!!`, `!!!` or `!=`) is a lexical error.

## Whitespace and termination

- Whitespace significant: **No.** Spaces, carriage returns (`\r`), and tabs (`\t`) are ignored by the Scanner. Newlines are also not treated as tokens, but they increment the line counter for error reporting.
- Statement terminator: `;`. Every statement ends with `;`. A missing one gives `Expect ';' after expression.` (or `... after value.` / `... after variable declaration.`), and the parser uses `;` as one of its recovery points after an error.
- Block delimiters: `{` and `}` group declarations and statements into a nested scope. Blocks can nest to any depth. A missing `}` gives `Expect '}' after block.`
- Grouping delimiters: `(` and `)`. Parentheses are used to group expressions in the grammar: `"(" expression ")"`.
- End of input: The Scanner automatically adds an `EOF` token after scanning the entire source.

## Token output format

```
Token(type=NUMBER, lexeme=5, literal=5, line=1)
```

Fields: token type, lexeme, literal value (`null` when the token carries none), line number.


## Grammar

```
program     → declaration* EOF
declaration → setDecl | dealDecl | statement
setDecl     → "set" IDENTIFIER ( "=" expression )? ";"
dealDecl    → "deal" IDENTIFIER "=" expression ";"
statement   → showStmt | exprStmt | block
showStmt    → "show" expression ";"
exprStmt    → expression ";"
block       → "{" declaration* "}"

expression  → assignment
assignment  → IDENTIFIER "=" assignment | equality
equality    → comparison ( ( "==" | "!=" ) comparison )*
comparison  → term ( ( "<" | "<=" | ">" | ">=" ) term )*
term        → factor ( ( "+" | "-" ) factor )*
factor      → unary ( ( "*" | "/" | "%" ) unary )*
unary       → ( "not" | "-" ) unary | primary
primary     → NUMBER | STRING | "true" | "false" | IDENTIFIER | "(" expression ")"
```

- **Where bindings and forms are legal:** Variable declarations (`set` and `deal`) can be used at the top level of the program or directly inside `{ ... }` blocks. `set` creates a mutable binding, and its `= expression` initializer is optional, so it defaults to `nil` if omitted. `deal` creates an immutable binding, so it needs an `= expression` initializer when declared. Executable statements like `show`, expression statements, and nested `{ ... }` blocks can be used wherever a `declaration` is expected. Assignments (`=`) and identifier lookups are expressions, so they can also be used anywhere an expression is allowed.

- `--tokenize`, `--parse`, `--eval` and plain `./run` all use the same `program` rule. The Lab 2 and Lab 3 test files still pass because a file of bare `expression ";"` lines is a valid program.

## Parse output format


```
(+ 1.0 (* 2.0 3.0))
```

- Groupings print as: `(group <expr>)`
- Numbers print with a decimal point: `1` prints as `1.0`
- Binary and unary operators print prefix: `(- 1.0 2.0)`, `(- 4.0)`
- Assignment prints as: `(= a 1.0)`
- `show` prints as: `(show <expr>)`
- `set` and `deal` print as: `(set a 1.0)`, `(deal a 1.0)`, and `(set a)` when there is no initializer
- Blocks print as: `(block <stmt> <stmt> ...)`
- A bare expression statement prints as the expression itself.

## Semantics

### Values and types

A runtime value is a tagged union (`enum Value` in `value.rs`) with four cases:
 
| Case | Host representation | Notes |
|---|---|---|
| Number | `f64` | One numeric type. There are no separate integers. |
| String | `String` | |
| Boolean | `bool` | |
| Nil | unit variant | No literal. Produced by `set a;`. |
 
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

- **Scope rule: lexical (static).** Each `{ }` block gets a new environment whose `enclosing` link points to the scope it is written inside. Lookup walks that chain from the innermost scope outward, and the innermost match wins. Blocks alone cannot tell lexical from dynamic scope; the difference appears with Lab 5 function calls.
- **Declaring vs. assigning:** `set` and `deal` always create a binding in the *current* scope and never look outward. Assignment (`=`) never creates anything. It updates the nearest existing binding, searching outward.
- Redeclaration in the same scope: allowed. The new binding replaces the old one, so `set a = 1; set a = 2;` leaves `a` as `2`. A name can also be redeclared as `set` or `deal`.
- Initializer order: the initializer is evaluated before the name is defined, so `set a = a;` reads the outer `a` (or is an undefined-variable error if none exists).
- Uninitialized variable holds: `nil`. `set a;` creates `a`, and `show a;` prints `nil`. A `deal` must have an initializer, and omitting it is a syntax error (exit 65).
- Shadowing: a `set` or `deal` inside a block creates a new variable that hides the outer one only inside that block. The outer variable is unchanged and visible again when the block ends.
- Immutable bindings: assigning to a `deal` binding is rejected **at runtime** (not at parse time) with `Cannot assign to immutable binding 'x'.` and exit 70.
- Assignment is an expression: it produces the assigned value, so `set b = (a = 5);` sets both.
- Undefined name: runtime error with exit 70, `Undefined variable 'x'.` It applies to reading and to assigning, and it is found when the line runs, not at parse time. We chose runtime rather than static checking because it is simpler and keeps recursive functions workable in Lab 5 without a separate resolver pass.
- Scope restore: after a block ends, the previous scope is restored even if a statement inside it raised a runtime error, so the REPL never gets stuck inside a dead block.

### Scope, lifetime and binding time

- **Visibility:** a binding becomes visible when its declaration executes (nothing is hoisted) and stays visible until its block ends. Top-level bindings stay visible for the whole program run or REPL session.
- **Lifetime (storage):** storage for a binding is created when its declaration runs. Top-level bindings live as long as the program or REPL session. Block bindings are stack-dynamic: they live for one execution of the block and are released when the block exits, since nothing in the language can capture them yet. Lab 5 closures may change this. The host language uses a heap-allocated `HashMap` for each environment, but that is an implementation detail and not what the guest language promises.
- **Binding time:** names are bound to storage at runtime, when the declaration executes. Types are bound to values dynamically, so a `set` variable can change type: `set a = 1; a = "x";` is legal. A `deal` binding cannot change value or type.

### Trace

```
set x = "outer";
{
  set x = "inner";
  show x;
}
show x;
```

While `show x;` runs inside the block, two environments are chained: `[block: x = "inner"] -> [global: x = "outer"]`. The lookup finds `"inner"` in the block and stops. After the block ends, only the global environment remains, so the second `show x;` prints `outer`.


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

Messages format:

| Phase | Message |
| --- | --- |
| lexical (65) | `Unexpected character 'c'.`, `Unterminated string.`, `Unterminated block comment.` |
| syntax (65) | `Expect expression.`, `Expect ';' after expression.`, `Expect ';' after value.`, `Expect ';' after variable declaration.`, `Expect variable name.`, `Expect '=' after immutable variable name.`, `Expect '}' after block.`, `Expect ')' after expression.`, `Invalid assignment target.` |
| runtime (70) | `Undefined variable 'x'.`, `Cannot assign to immutable binding 'x'.`, `Operand must be a number.`, `Operands must be numbers.`, `Operands must be two numbers or two strings.`, `Division by zero.`, `Modulo by zero.` |

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
python3 run_tests.py tests/lab0
python3 run_tests.py tests/lab1
python3 run_tests.py tests/lab2
python3 run_tests.py tests/lab3
python3 run_tests.py tests/lab4
```
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

## Statements and State Example

Input:

```text
set a = 1;
{
  set b = 2;
  {
    a = a + b;
    show a;
  }
  show b;
}
show a;
```

Output (`./run <file>`):

```text
3
2
3
```

The last line is `3` because `a = a + b` updated the outer `a` two scopes up instead of creating a new local one.

`./run --parse` on the same shape of program prints one line per statement, for example `(set x outer)`, `(block (set x inner) (show x))`, `(show x)`.

A `deal` binding cannot be reassigned:

```text
deal d = 1;
d = 2;
```

This prints `[line 2] Runtime error: Cannot assign to immutable binding 'd'.` on stderr and exits with code 70.




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

- **Rust enums for the AST:** `Expr` has `Literal`, `Grouping`, `Binary`, `Unary`, `Variable` and `Assign` nodes. `Stmt` (separate from `Expr`) has `Expression`, `Show`, `VarDecl` and `Block`. `Program` owns the top-level statements.

- **Error recovery:** When parsing fails, the Parser skips tokens until just after a `;`, or until a keyword (`set`, `deal`, `show`, ...) or brace that can start or end a statement, and then continues. It always consumes at least one token first, so it can never loop forever. A missing `;` right before `}` does not eat the closing brace.

- **Tagged union for values:** `Value` is a Rust enum with one variant per runtime type (number, string, boolean, nil). Matching on the variant is how the evaluator checks types, and the compiler warns about missed cases. We avoided `Box<dyn Any>` with downcasting, which Rust does not need.

- **Division by zero is an error:** Dividing by zero is most likely a mistake. `Infinity` or `NaN` would let it spread silently through later calculations, so we report it where it happens.

- **No automatic conversion in `+`:** `"a" + 1` is an error, because silent conversion hides type mistakes. Strings only concatenate with strings.

- **Errors as `Result`, not panics:** The evaluator returns `Result<Value, RuntimeError>` and uses `?` to unwind. The error carries the operator token so the message includes a line number, and it is reported in one place (`main.rs` for files, `repl.rs` for the REPL).

- **Left-to-right evaluation:** The left operand is always evaluated before the right.

- **Semantics written once:** Truthiness, equality, and number formatting each live in `value.rs` and are shared by file mode and the REPL.

- **`set` / `deal` instead of `var`:** `set` is mutable and `deal` is immutable, enforced when assignment runs. This lets the language show both rebinding (redeclare) and mutation (assign).

- **Statements separate from expressions:** `show`, declarations and blocks are `Stmt` nodes, so they cannot appear where a value is needed.

- **Environment chain with `Rc<RefCell<...>>`:** each scope owns a map and a link to its enclosing scope. Shared ownership leaves room for Lab 5 closures to keep a scope alive after its block ends.

- **Scope restored on error:** `execute_block` does not use `?` inside its loop, so it always restores the previous scope before returning. A runtime error in a block cannot leave the REPL inside a dead scope.

- **Declaration vs. assignment:** declaring always defines in the current scope, and assigning never creates a binding. This keeps shadowing and mutation from being confused, and means a misspelled name is an error instead of a new variable.

- **One interpreter, two output rules:** file mode and the REPL share one evaluator. A single `echo` flag decides whether bare expressions print (`--eval` and REPL) or only `show` prints (`./run <file>`).

## Known limitations

- `call`, `flush`, `fold`, `bet`, `bust`, `round`, `bluff`, `draw` and `raise` are scanned but not parsed. There are no functions or control flow yet.
- `and`, `or` and `,` are scanned but not parsed.
- There is no `nil` literal, no string escapes, and block comments cannot be nested.
- Assigning to a `deal` binding is caught at runtime, not at parse time.
- The REPL reads one line at a time, so blocks, strings and `!!!` comments that span several lines cannot be entered there.
- Undefined variables are detected at runtime, not before execution.

## Changelog

| Activity | What changed in the language |
|---|---|
| Lab 1 | Implemented the Scanner: operators, strings, numbers, identifiers and keywords, `!!`/`!!!` comments, line tracking, `--tokenize` mode, and error reporting with exit code 65. |
| Lab 2 | Added the Parser, AST, grammar, expression parsing, error recovery, `--parse`, and REPL parsing. |
| Lab 3 | Added the evaluator and `Value` enum, runtime type checking, runtime errors with line numbers and exit code 70, `--eval` mode, and REPL evaluation. Added the inline test folder `tests/lab3`. |
| Lab 4 | Added statements and state: `show`, `set` (mutable, optional initializer), `deal` (immutable), assignment as a right-associative expression, nested `{ }` blocks with lexical scope, the `Environment` chain, and the `Stmt`/`Program` AST. Plain `./run <file>` now executes programs, the REPL keeps variables across lines, new runtime errors for undefined variables and immutable assignment, and the inline test folder `tests/lab4`. |