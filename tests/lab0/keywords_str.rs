use perine_lab0::scanner::Scanner;
use perine_lab0::token::TokenType;

// checks that every keyword scans to the right token, in order
#[test]
fn recognizes_all_keywords() {
    let input = "set deal call flush fold bet bust round bluff draw raise show";

    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens();

    // one token per word, plus eof at the end
    let expected_types = vec![
        TokenType::Set,
        TokenType::Deal,
        TokenType::Call,
        TokenType::Flush,
        TokenType::Fold,
        TokenType::Bet,
        TokenType::Bust,
        TokenType::Round,
        TokenType::Bluff,
        TokenType::Draw,
        TokenType::Raise,
        TokenType::Show,
        TokenType::Eof,
    ];

    assert_eq!(tokens.len(), expected_types.len());

    for i in 0..tokens.len() {
        assert_eq!(tokens[i].token_type, expected_types[i]);
    }
}

// "Set" with a capital s is not the same as the keyword "set"
#[test]
fn keywords_are_case_sensitive() {
    let mut scanner = Scanner::new("Set");
    let tokens = scanner.scan_tokens();

    assert_eq!(tokens[0].token_type, TokenType::Identifier);
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

// "setter" should stay one identifier, not split into "set" + "ter"
#[test]
fn identifier_not_shadowed_by_keyword_prefix() {
    let mut scanner = Scanner::new("setter");
    let tokens = scanner.scan_tokens();

    assert_eq!(tokens[0].token_type, TokenType::Identifier);
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

// a simple quoted string should scan as one string token
#[test]
fn basic_string_literal() {
    let mut scanner = Scanner::new("\"hello\"");
    let tokens = scanner.scan_tokens();

    assert_eq!(tokens[0].token_type, TokenType::Str);
    assert_eq!(tokens[0].lexeme, "\"hello\"");
}

// line count should keep updating even inside a multi-line string
#[test]
fn string_spanning_multiple_lines_tracks_line_number() {
    let mut scanner = Scanner::new("\"line one\nline two\"\nx");
    let tokens = scanner.scan_tokens();

    assert_eq!(tokens[0].token_type, TokenType::Str);

    // "x" comes after 2 newlines, so it should be on line 3
    assert_eq!(tokens[1].line, 3);
}

// an unclosed string should flag an error, not crash or get ignored
#[test]
fn unterminated_string_sets_had_error() {
    let mut scanner = Scanner::new("\"never closed");
    scanner.scan_tokens();

    assert!(scanner.had_error);
}

// matches the worked example from the language spec doc
#[test]
fn set_variable_declaration_example_from_spec() {
    let mut scanner = Scanner::new("set spade = 5");
    let tokens = scanner.scan_tokens();

    assert_eq!(tokens[0].token_type, TokenType::Set);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[2].token_type, TokenType::Assign);
    assert_eq!(tokens[3].token_type, TokenType::Number);
    assert_eq!(tokens[4].token_type, TokenType::Eof);
}