use super::*;
#[test]
fn test_basic_string() {
    assert_eq!(rep("abcABC123"), "abcABC123");
}

#[test]
fn string_containing_spaces() {
    assert_eq!(rep("hello mal world"), "hello mal world");
}

#[test]
fn string_containing_symbols() {
    assert_eq!(rep(r#"[]{}"'* ;:()"#), r#"[]{}"'* ;:()"#);
}

#[test]
fn long_string() {
    assert_eq!(
        rep(
            r#"hello world abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ 0123456789 (;:() []{}"'* ;:() []{}"'* ;:() []{}"'*)"#
        ),
        r#"hello world abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ 0123456789 (;:() []{}"'* ;:() []{}"'* ;:() []{}"'*)"#
    );
}

#[test]
fn non_alphanumeric_characters() {
    assert_eq!(rep("!"), "!");
    assert_eq!(rep("&"), "&");
    assert_eq!(rep("+"), "+");
    assert_eq!(rep(","), ",");
    assert_eq!(rep("-"), "-");
    assert_eq!(rep("/"), "/");
    assert_eq!(rep("<"), "<");
    assert_eq!(rep("="), "=");
    assert_eq!(rep(">"), ">");
    assert_eq!(rep("?"), "?");
    assert_eq!(rep("@"), "@");
    assert_eq!(rep("^"), "^");
    assert_eq!(rep("_"), "_");
    assert_eq!(rep("`"), "`");
    assert_eq!(rep("~"), "~");
    assert_eq!(rep("#"), "#");
    assert_eq!(rep("$"), "$");
    assert_eq!(rep("%"), "%");
    assert_eq!(rep("."), ".");
    assert_eq!(rep("|"), "|");
}
