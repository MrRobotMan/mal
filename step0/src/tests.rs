use super::*;

fn test_runner(inp: &str, out: &str) {
    let res = {
        let a = read(inp.into());
        let a = eval(a);
        print(a)
    };
    assert_eq!(res, out);
}

//  Testing basic string
#[test]
fn test0() {
    test_runner("abcABC123", "abcABC123");
}

//  Testing string containing spaces
#[test]
fn test1() {
    test_runner("hello mal world", "hello mal world");
}

//  Testing string containing symbols
#[test]
fn test2() {
    test_runner("[]{}\"'* ;:()", "[]{}\"'* ;:()");
}

//  Test long string
#[test]
fn test3() {
    test_runner(
        "hello world abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ 0123456789 (;:() []{}\"'* ;:() []{}\"'* ;:() []{}\"'*)",
        "hello world abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ 0123456789 (;:() []{}\"'* ;:() []{}\"'* ;:() []{}\"'*)",
    );
}
