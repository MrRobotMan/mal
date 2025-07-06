use super::*;

fn test_runner(inp: &str, out: Option<&str>) {
    let res = rep(inp);
    if let Some(s) = out {
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), s);
    } else {
        assert!(res.is_err());
    }
}

// ; Testing read of numbers
#[test]
fn test0() {
    test_runner("1", Some("1"));
}

#[test]
fn test1() {
    test_runner("7", Some("7"));
}

#[test]
fn test2() {
    test_runner("  7   ", Some("7"));
}

#[test]
fn test3() {
    test_runner("-123", Some("-123"));
}

// ; Testing read of symbols
#[test]
fn test4() {
    test_runner("+", Some("+"));
}

#[test]
fn test5() {
    test_runner("abc", Some("abc"));
}

#[test]
fn test6() {
    test_runner("   abc   ", Some("abc"));
}

#[test]
fn test7() {
    test_runner("abc5", Some("abc5"));
}

#[test]
fn test8() {
    test_runner("abc-def", Some("abc-def"));
}

// ; Testing read of lists
#[test]
fn test9() {
    test_runner("(+ 1 2)", Some("(+ 1 2)"));
}

#[test]
fn test10() {
    test_runner("()", Some("()"));
}

#[test]
fn test11() {
    test_runner("(nil)", Some("(nil)"));
}

#[test]
fn test12() {
    test_runner("((3 4))", Some("((3 4))"));
}

#[test]
fn test13() {
    test_runner("(+ 1 (+ 2 3))", Some("(+ 1 (+ 2 3))"));
}

#[test]
fn test14() {
    test_runner("  ( +   1   (+   2 3   )   )  ", Some("(+ 1 (+ 2 3))"));
}

#[test]
fn test15() {
    test_runner("(* 1 2)", Some("(* 1 2)"));
}

#[test]
fn test16() {
    test_runner("(** 1 2)", Some("(** 1 2)"));
}

#[test]
fn test17() {
    test_runner("(* -3 6)", Some("(* -3 6)"));
}

// ; Test commas as whitespace
#[test]
fn test18() {
    test_runner("(1 2, 3,,,,),,", Some("(1 2 3)"));
}

// >>> deferrable=True
// ;
// ; -------- Deferrable Functionality --------
// ; Testing read of nil/true/false
#[test]
fn test19() {
    test_runner("nil", Some("nil"));
}

#[test]
fn test20() {
    test_runner("true", Some("true"));
}

#[test]
fn test21() {
    test_runner("false", Some("false"));
}

// ; Testing read of strings
#[test]
fn test22() {
    test_runner("\"abc\"", Some("\"abc\""));
}

#[test]
fn test23() {
    test_runner("   \"abc\"   ", Some("\"abc\""));
}

#[test]
fn test24() {
    test_runner("\"abc (with parens)\"", Some("\"abc (with parens)\""));
}

#[test]
fn test25() {
    test_runner("\"abc\\\"def\"", Some("\"abc\\\"def\""));
}

// ;;"abc\ndef"
// ;;;=>"abc\ndef"
#[test]
fn test26() {
    test_runner("\"\"", Some("\"\""));
}

// ; Testing reader errors

#[test]
fn testf1() {
    test_runner("(1 2", None)
}

#[test]
fn testf2() {
    test_runner("\"abc", None)
}

#[test]
fn testf3() {
    test_runner("[1 2", None)
}

#[test]
fn testf4() {
    test_runner("(1 \"abc", None)
}

// ; Testing read of quoting
#[test]
fn test27() {
    test_runner("'1", Some("(quote 1)"));
}

#[test]
fn test28() {
    test_runner("'(1 2 3)", Some("(quote (1 2 3))"));
}

#[test]
fn test29() {
    test_runner("`1", Some("(quasiquote 1)"));
}

#[test]
fn test30() {
    test_runner("`(1 2 3)", Some("(quasiquote (1 2 3))"));
}

#[test]
fn test31() {
    test_runner("~1", Some("(unquote 1)"));
}

#[test]
fn test32() {
    test_runner("~(1 2 3)", Some("(unquote (1 2 3))"));
}

#[test]
fn test33() {
    test_runner("`(1 ~a 3)", Some("(quasiquote (1 (unquote a) 3))"));
}

#[test]
fn test34() {
    test_runner("~@(1 2 3)", Some("(splice-unquote (1 2 3))"));
}

// >>> optional=True
// ;
// ; -------- Optional Functionality --------
// ; Testing keywords
#[test]
fn test35() {
    test_runner(":kw", Some(":kw"));
}

#[test]
fn test36() {
    test_runner("(:kw1 :kw2 :kw3)", Some("(:kw1 :kw2 :kw3)"));
}

// ; Testing read of vectors
#[test]
fn test37() {
    test_runner("[+ 1 2]", Some("[+ 1 2]"));
}

#[test]
fn test38() {
    test_runner("[]", Some("[]"));
}

#[test]
fn test39() {
    test_runner("[[3 4]]", Some("[[3 4]]"));
}

#[test]
fn test40() {
    test_runner("[+ 1 [+ 2 3]]", Some("[+ 1 [+ 2 3]]"));
}

#[test]
fn test41() {
    test_runner("  [ +   1   [+   2 3   ]   ]  ", Some("[+ 1 [+ 2 3]]"));
}

// ; Testing read of hash maps
#[test]
fn test42() {
    test_runner("{\"abc\" 1}", Some("{\"abc\" 1}"));
}

#[test]
fn test43() {
    test_runner("{\"a\" {\"b\" 2}}", Some("{\"a\" {\"b\" 2}}"));
}

#[test]
fn test44() {
    test_runner(
        "{\"a\" {\"b\" {\"c\" 3}}}",
        Some("{\"a\" {\"b\" {\"c\" 3}}}"),
    );
}

#[test]
fn test45() {
    test_runner(
        "{  \"a\"  {\"b\"   {  \"cde\"     3   }  }}",
        Some("{\"a\" {\"b\" {\"cde\" 3}}}"),
    );
}

#[test]
fn test46() {
    test_runner(
        "{  :a  {:b   {  :cde     3   }  }}",
        Some("{:a {:b {:cde 3}}}"),
    );
}

// ; Testing read of comments
#[test]
fn test47() {
    test_runner("1 ; comment after expression", Some("1"));
}

#[test]
fn test48() {
    test_runner("1; comment after expression", Some("1"));
}

// ; Testing read of ^/metadata
#[test]
fn test49() {
    test_runner("^{\"a\" 1} [1 2 3]", Some("(with-meta [1 2 3] {\"a\" 1})"));
}

// ; Testing read of @/deref
#[test]
fn test50() {
    test_runner("@a", Some("(deref a)"));
}
