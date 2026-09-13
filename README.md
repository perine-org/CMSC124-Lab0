# PokerScript
 Members: *Samaniego, PL & Medalla, EL*

## Overview

PokerScript is a beginner-friendly, general-purpose programming language designed to make programming feel like a strategic game of poker. It introduces a card-based vocabulary where programming concepts are represented through familiar gambling terms, such as `call`, `deal`, `raise`, `fold`, `bust`, and `flush`. Variables and data are handled as cards, while control flow and program execution are expressed through actions commonly associated with gameplay.

Despite its gambling-inspired theme, the language emphasizes strategy and calculated decision-making rather than chance, reflecting how programmers build logic and determine outcomes through their choices. Its goal is to provide a playful yet understandable programming experience while maintaining core programming concepts and functionality.

## Language Profile

| Property | Value |
|---|---|
| Name | PokerScript |
| File Extension | `.pkr` |

## Token List

### Recognition Rules

| First Character | Continue While | Token Type |
|---|---|---|
| Letter or `~` | Letter, digit, or `~` | `IDENTIFIER` |
| Digit | Digit | `NUMBER` |
| `"` | Until next `"` | `STRING` |
| `(` | No continuation | `LEFT_PAREN` |
| `)` | No continuation | `RIGHT_PAREN` |
| `{` | No continuation | `LEFT_BRACE` |
| `}` | No continuation | `RIGHT_BRACE` |
| `;` | No continuation | `SEMICOLON` |
| `<` | Take a following `=` when present | `LESS_EQUAL` for `<=`, otherwise `LESS` |
| `>` | Take a following `=` when present | `GREATER_EQUAL` for `>=`, otherwise `GREATER` |
| `+` | No continuation | `PLUS` |
| `-` | No continuation | `MINUS` |
| `/` | No continuation | `DIVIDE` |
| `=` | Take a following `=` when present | `EQUAL` for `==`, otherwise `ASSIGN` |
| `*` | No continuation | `MULTIPLY` |
| `,` | No continuation | `COMMA` |

### Keywords

| Keyword | Description |
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
| `bluff` | |
| `draw` |  |
| `raise` |  |


## Example: Turning Code into Tokens

**Input:**
```
set spade = 5
```

**Scanner output:**
```
Token(type=SET, lexeme=set, line=1)
Token(type=IDENTIFIER, lexeme=spade, line=1)
Token(type=ASSIGN, lexeme==, line=1)
Token(type=NUMBER, lexeme=5, line=1)
Token(type=EOF, lexeme=, line=1)
```

## Error Handling

1. When a character isn't recognized, stop with the exit code.

## Lexical Structure

- **Case sensitivity:** Identifiers and keywords are case sensitive (`set` is different from `Set`).
- **Whitespace:** Indentation has no meaning; braces `{}` are used to indicate a block.
- **Variable declaration:** `set <identifier> = <expression>`. Variable declarations are mutable by default; adding `deal` makes them immutable.
- **Strings:** Characters enclosed in double quotes (`"`) are treated strictly as string literals.
- **Comments:** Anything after `!!!` is treated as a comment, until closed (block comment).

## Estimated Timeline

### Week 1
- Token type list (single-character, multi-character, literals, keywords) and prototype
- **Sept 7:** Progress Report

### Week 2
- **Sept 9–13:** Implementation of multi-char ops, strings, numbers, identifiers/keywords, comments, EOF; writing tests for Lab 1
- **Sept 13:** Book slot
- **Sept 14:** Set up `tests/lab1/`, write first tests, commit
- **Sept 15:** Progress Report

### Week 3
- **Sept 16–17:** Implement error reporting (stderr, keep scanning, exit codes) and write tests for every required token category
- **Sept 18–19:** Write rejection tests for 3 failure cases and wire `tests/lab1` into CI workflow; confirm green
- **Sept 20:** Book slot; run harness locally before pushing
- **Sept 21:** Finish README (lexical structure, errors, rationale)
- **Sept 22:** Progress Report

### Week 4
- **Sept 27:** Confirm green CI on defense commit; book slot
- **Sept 28–29:** Laboratory Defense
