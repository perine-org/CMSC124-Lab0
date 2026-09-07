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