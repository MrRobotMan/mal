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

// ; Testing evaluation of arithmetic operations
#[test]
fn test0() {
    test_runner("(+ 1 2)", Some("3"));
}

#[test]
fn test1() {
    test_runner("(+ 5 (* 2 3))", Some("11"));
}

#[test]
fn test2() {
    test_runner("(- (+ 5 (* 2 3)) 3)", Some("8"));
}

#[test]
fn test3() {
    test_runner("(/ (- (+ 5 (* 2 3)) 3) 4)", Some("2"));
}

#[test]
fn test4() {
    test_runner("(/ (- (+ 515 (* 87 311)) 302) 27)", Some("1010"));
}

#[test]
fn test5() {
    test_runner("(* -3 6)", Some("-18"));
}

#[test]
fn test6() {
    test_runner("(/ (- (+ 515 (* -87 311)) 296) 27)", Some("-994"));
}

//; .*\'abc\' not found.*
#[test]
fn testf1() {
    test_runner("(abc 1 2 3)", None);
}

// ; Testing empty list
#[test]
fn test7() {
    test_runner("()", Some("()"));
}

// >>> deferrable=True
// >>> optional=True
// ;
// ; -------- Deferrable/Optional Functionality --------
// ; Testing evaluation within collection literals
#[test]
fn test8() {
    test_runner("[1 2 (+ 1 2)]", Some("[1 2 3]"));
}

#[test]
fn test9() {
    test_runner("{\"a\" (+ 7 8)}", Some("{\"a\" 15}"));
}

#[test]
fn test10() {
    test_runner("{:a (+ 7 8)}", Some("{:a 15}"));
}
