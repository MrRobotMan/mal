use super::*;

fn test_runner(inp: &[&str], out: Option<&str>) {
    let mut env = env::Env::default().with_debug();
    let mut res = rep(inp[0], &mut env);
    if inp.len() > 1 {
        for line in inp.iter().skip(1) {
            res = rep(line, &mut env);
        }
    }
    if let Some(s) = out {
        if let Err(e) = &res {
            println!("{e:?}");
        }
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), s);
    } else {
        assert!(res.is_err());
    }
}

// ; -----------------------------------------------------
// ; Testing list functions
#[test]
fn test000() {
    test_runner(&["(list)"], Some("()"));
}

#[test]
fn test001() {
    test_runner(&["(list? (list))"], Some("true"));
}

#[test]
fn test002() {
    test_runner(&["(empty? (list))"], Some("true"));
}

#[test]
fn test003() {
    test_runner(&["(empty? (list 1))"], Some("false"));
}

#[test]
fn test004() {
    test_runner(&["(list 1 2 3)"], Some("(1 2 3)"));
}

#[test]
fn test005() {
    test_runner(&["(count (list 1 2 3))"], Some("3"));
}

#[test]
fn test006() {
    test_runner(&["(count (list))"], Some("0"));
}

#[test]
fn test7() {
    test_runner(&["(count nil)"], Some("0"));
}

#[test]
fn test008() {
    test_runner(
        &["(if (> (count (list 1 2 3)) 3) \"yes\" \"no\")"],
        Some("\"no\""),
    );
}

#[test]
fn test009() {
    test_runner(
        &["(if (>= (count (list 1 2 3)) 3) \"yes\" \"no\")"],
        Some("\"yes\""),
    );
}

// ; Testing if form
#[test]
fn test010() {
    test_runner(&["(if true 7 8)"], Some("7"));
}

#[test]
fn test011() {
    test_runner(&["(if false 7 8)"], Some("8"));
}

#[test]
fn test012() {
    test_runner(&["(if true (+ 1 7) (+ 1 8))"], Some("8"));
}

#[test]
fn test013() {
    test_runner(&["(if false (+ 1 7) (+ 1 8))"], Some("9"));
}

#[test]
fn test014() {
    test_runner(&["(if nil 7 8)"], Some("8"));
}

#[test]
fn test015() {
    test_runner(&["(if 0 7 8)"], Some("7"));
}

#[test]
fn test016() {
    test_runner(&["(if \"\" 7 8)"], Some("7"));
}

#[test]
fn test017() {
    test_runner(&["(if (list) 7 8)"], Some("7"));
}

#[test]
fn test018() {
    test_runner(&["(if (list 1 2 3) 7 8)"], Some("7"));
}

#[test]
fn test019() {
    test_runner(&["(= (list) nil)"], Some("false"));
}

// ; Testing 1-way if form
#[test]
fn test020() {
    test_runner(&["(if false (+ 1 7))"], Some("nil"));
}

#[test]
fn test021() {
    test_runner(&["(if nil 8 7)"], Some("7"));
}

#[test]
fn test022() {
    test_runner(&["(if true (+ 1 7))"], Some("8"));
}

// ; Testing basic conditionals
#[test]
fn test023() {
    test_runner(&["(= 2 1)"], Some("false"));
}

#[test]
fn test024() {
    test_runner(&["(= 1 1)"], Some("true"));
}

#[test]
fn test025() {
    test_runner(&["(= 1 2)"], Some("false"));
}

#[test]
fn test026() {
    test_runner(&["(= 1 (+ 1 1))"], Some("false"));
}

#[test]
fn test027() {
    test_runner(&["(= 2 (+ 1 1))"], Some("true"));
}

#[test]
fn test028() {
    test_runner(&["(= nil 1)"], Some("false"));
}

#[test]
fn test029() {
    test_runner(&["(= nil nil)"], Some("true"));
}

#[test]
fn test030() {
    test_runner(&["(> 2 1)"], Some("true"));
}

#[test]
fn test031() {
    test_runner(&["(> 1 1)"], Some("false"));
}

#[test]
fn test032() {
    test_runner(&["(> 1 2)"], Some("false"));
}

#[test]
fn test033() {
    test_runner(&["(>= 2 1)"], Some("true"));
}

#[test]
fn test034() {
    test_runner(&["(>= 1 1)"], Some("true"));
}

#[test]
fn test035() {
    test_runner(&["(>= 1 2)"], Some("false"));
}

#[test]
fn test036() {
    test_runner(&["(< 2 1)"], Some("false"));
}

#[test]
fn test037() {
    test_runner(&["(< 1 1)"], Some("false"));
}

#[test]
fn test038() {
    test_runner(&["(< 1 2)"], Some("true"));
}

#[test]
fn test039() {
    test_runner(&["(<= 2 1)"], Some("false"));
}

#[test]
fn test040() {
    test_runner(&["(<= 1 1)"], Some("true"));
}

#[test]
fn test041() {
    test_runner(&["(<= 1 2)"], Some("true"));
}

// ; Testing equality
#[test]
fn test042() {
    test_runner(&["(= 1 1)"], Some("true"));
}

#[test]
fn test043() {
    test_runner(&["(= 0 0)"], Some("true"));
}

#[test]
fn test044() {
    test_runner(&["(= 1 0)"], Some("false"));
}

#[test]
fn test045() {
    test_runner(&["(= \"\" \"\")"], Some("true"));
}

#[test]
fn test046() {
    test_runner(&["(= \"abc\" \"abc\")"], Some("true"));
}

#[test]
fn test047() {
    test_runner(&["(= \"abc\" \"\")"], Some("false"));
}

#[test]
fn test048() {
    test_runner(&["(= \"\" \"abc\")"], Some("false"));
}

#[test]
fn test049() {
    test_runner(&["(= \"abc\" \"def\")"], Some("false"));
}

#[test]
fn test050() {
    test_runner(&["(= \"abc\" \"ABC\")"], Some("false"));
}

#[test]
fn test051() {
    test_runner(&["(= true true)"], Some("true"));
}

#[test]
fn test052() {
    test_runner(&["(= false false)"], Some("true"));
}

#[test]
fn test053() {
    test_runner(&["(= nil nil)"], Some("true"));
}

#[test]
fn test054() {
    test_runner(&["(= (list) (list))"], Some("true"));
}

#[test]
fn test055() {
    test_runner(&["(= (list 1 2) (list 1 2))"], Some("true"));
}

#[test]
fn test056() {
    test_runner(&["(= (list 1) (list))"], Some("false"));
}

#[test]
fn test057() {
    test_runner(&["(= (list) (list 1))"], Some("false"));
}

#[test]
fn test058() {
    test_runner(&["(= 0 (list))"], Some("false"));
}

#[test]
fn test059() {
    test_runner(&["(= (list) 0)"], Some("false"));
}

#[test]
fn test060() {
    test_runner(&["(= (list) \"\")"], Some("false"));
}

#[test]
fn test061() {
    test_runner(&["(= \"\" (list))"], Some("false"));
}

// ; Testing builtin and user defined functions
#[test]
fn test062() {
    test_runner(&["(+ 1 2)"], Some("3"));
}

#[test]
fn test063() {
    test_runner(&["( (fn* (a b) (+ b a)) 3 4)"], Some("7"));
}

#[test]
fn test064() {
    test_runner(&["( (fn* () 4) )"], Some("4"));
}

#[test]
fn test065() {
    test_runner(&["( (fn* (f x) (f x)) (fn* (a) (+ 1 a)) 7)"], Some("8"));
}

// ; Testing closures
#[test]
fn test066() {
    test_runner(&["( ( (fn* (a) (fn* (b) (+ a b))) 5) 7)"], Some("12"));
}

#[test]
fn test067() {
    test_runner(
        &[
            "(def! gen-plus5 (fn* () (fn* (b) (+ 5 b))))",
            "(def! plus5 (gen-plus5))",
            "(plus5 7)",
        ],
        Some("12"),
    );
}

#[test]
fn test068() {
    test_runner(
        &[
            "(def! gen-plus5 (fn* () (fn* (b) (+ 5 b))))",
            "(def! plus5 (gen-plus5))",
            "(def! gen-plusX (fn* (x) (fn* (b) (+ x b))))",
            "(def! plus7 (gen-plusX 7))",
            "(plus7 8)",
        ],
        Some("15"),
    );
}

// ; Testing do form
//  "prn output1"
#[test]
fn test069() {
    test_runner(&["(do (prn \"prn output1\"))"], Some("nil"));
}

//  "prn output2"
#[test]
fn test070() {
    test_runner(&["(do (prn \"prn output2\") 7)"], Some("7"));
}

//  "prn output1"
//  "prn output2"
#[test]
fn test071() {
    test_runner(
        &["(do (prn \"prn output1\") (prn \"prn output2\") (+ 1 2))"],
        Some("3"),
    );
}

#[test]
fn test072() {
    test_runner(&["(do (def! a 6) 7 (+ a 8))"], Some("14"));
}

#[test]
fn test073() {
    test_runner(&["(do (def! a 6) 7 (+ a 8))", "a"], Some("6"));
}

// ; Testing special form case-sensitivity
#[test]
fn test074() {
    test_runner(&["(def! DO (fn* (a) 7))", "(DO 3)"], Some("7"));
}

// ; Testing recursive sumdown function
#[test]
fn test075() {
    test_runner(
        &[
            "(def! sumdown (fn* (N) (if (> N 0) (+ N (sumdown  (- N 1))) 0)))",
            "(sumdown 1)",
        ],
        Some("1"),
    );
}

#[test]
fn test076() {
    test_runner(
        &[
            "(def! sumdown (fn* (N) (if (> N 0) (+ N (sumdown  (- N 1))) 0)))",
            "(sumdown 2)",
        ],
        Some("3"),
    );
}

#[test]
fn test077() {
    test_runner(
        &[
            "(def! sumdown (fn* (N) (if (> N 0) (+ N (sumdown  (- N 1))) 0)))",
            "(sumdown 6)",
        ],
        Some("21"),
    );
}

// ; Testing recursive fibonacci function
#[test]
fn test078() {
    test_runner(
        &[
            "(def! fib (fn* (N) (if (= N 0) 1 (if (= N 1) 1 (+ (fib (- N 1)) (fib (- N 2)))))))",
            "(fib 1)",
        ],
        Some("1"),
    );
}

#[test]
fn test079() {
    test_runner(
        &[
            "(def! fib (fn* (N) (if (= N 0) 1 (if (= N 1) 1 (+ (fib (- N 1)) (fib (- N 2)))))))",
            "(fib 2)",
        ],
        Some("2"),
    );
}

#[test]
fn test080() {
    test_runner(
        &[
            "(def! fib (fn* (N) (if (= N 0) 1 (if (= N 1) 1 (+ (fib (- N 1)) (fib (- N 2)))))))",
            "(fib 4)",
        ],
        Some("5"),
    );
}

// ;; Too slow for bash, erlang, make and miniMAL
// ;;(fib 10)
// ;;;=>89
// >>> deferrable=True
// ;
// ; -------- Deferrable Functionality --------
// ; Testing variable length arguments
#[test]
fn test081() {
    test_runner(&["( (fn* (& more) (count more)) 1 2 3)"], Some("3"));
}

#[test]
fn test082() {
    test_runner(&["( (fn* (& more) (list? more)) 1 2 3)"], Some("true"));
}

#[test]
fn test083() {
    test_runner(&["( (fn* (& more) (count more)) 1)"], Some("1"));
}

#[test]
fn test084() {
    test_runner(&["( (fn* (& more) (count more)) )"], Some("0"));
}

#[test]
fn test085() {
    test_runner(&["( (fn* (& more) (list? more)) )"], Some("true"));
}

#[test]
fn test086() {
    test_runner(&["( (fn* (a & more) (count more)) 1 2 3)"], Some("2"));
}

#[test]
fn test087() {
    test_runner(&["( (fn* (a & more) (count more)) 1)"], Some("0"));
}

#[test]
fn test088() {
    test_runner(&["( (fn* (a & more) (list? more)) 1)"], Some("true"));
}

// ; Testing language defined not function
#[test]
fn test089() {
    test_runner(&["(not false)"], Some("true"));
}

#[test]
fn test090() {
    test_runner(&["(not nil)"], Some("true"));
}

#[test]
fn test091() {
    test_runner(&["(not true)"], Some("false"));
}

#[test]
fn test092() {
    test_runner(&["(not \"a\")"], Some("false"));
}

#[test]
fn test093() {
    test_runner(&["(not 0)"], Some("false"));
}

// ; -----------------------------------------------------
// ; Testing string quoting
#[test]
fn test094() {
    test_runner(&["\"\""], Some("\"\""));
}

#[test]
fn test095() {
    test_runner(&["\"abc\""], Some("\"abc\""));
}

#[test]
fn test096() {
    test_runner(&["\"abc  def\""], Some("\"abc  def\""));
}

#[test]
fn test097() {
    test_runner(&["\"\\\"\""], Some("\"\\\"\""));
}

#[test]
fn test098() {
    test_runner(&["\"abc\\ndef\\nghi\""], Some("\"abc\\ndef\\nghi\""));
}

#[test]
fn test099() {
    test_runner(&["\"abc\\\\def\\\\ghi\""], Some("\"abc\\\\def\\\\ghi\""));
}

#[test]
fn test100() {
    test_runner(&["\"\\\\n\""], Some("\"\\\\n\""));
}

// ; Testing pr-str
#[test]
fn test101() {
    test_runner(&["(pr-str)"], Some("\"\""));
}

#[test]
fn test102() {
    test_runner(&["(pr-str \"\")"], Some("\"\\\"\\\"\""));
}

#[test]
fn test103() {
    test_runner(&["(pr-str \"abc\")"], Some("\"\\\"abc\\\"\""));
}

#[test]
fn test104() {
    test_runner(
        &["(pr-str \"abc  def\" \"ghi jkl\")"],
        Some("\"\\\"abc  def\\\" \\\"ghi jkl\\\"\""),
    );
}

#[test]
fn test105() {
    test_runner(&["(pr-str \"\\\"\")"], Some("\"\\\"\\\\\\\"\\\"\""));
}

#[test]
fn test106() {
    test_runner(
        &["(pr-str (list 1 2 \"abc\" \"\\\"\") \"def\")"],
        Some("\"(1 2 \\\"abc\\\" \\\"\\\\\\\"\\\") \\\"def\\\"\""),
    );
}

#[test]
fn test107() {
    test_runner(
        &["(pr-str \"abc\\ndef\\nghi\")"],
        Some("\"\\\"abc\\\\ndef\\\\nghi\\\"\""),
    );
}

#[test]
fn test108() {
    test_runner(
        &["(pr-str \"abc\\\\def\\\\ghi\")"],
        Some("\"\\\"abc\\\\\\\\def\\\\\\\\ghi\\\"\""),
    );
}

#[test]
fn test109() {
    test_runner(&["(pr-str (list))"], Some("\"()\""));
}

// ; Testing str
#[test]
fn test110() {
    test_runner(&["(str)"], Some("\"\""));
}

#[test]
fn test111() {
    test_runner(&["(str \"\")"], Some("\"\""));
}

#[test]
fn test112() {
    test_runner(&["(str \"abc\")"], Some("\"abc\""));
}

#[test]
fn test113() {
    test_runner(&["(str \"\\\"\")"], Some("\"\\\"\""));
}

#[test]
fn test114() {
    test_runner(&["(str 1 \"abc\" 3)"], Some("\"1abc3\""));
}

#[test]
fn test115() {
    test_runner(
        &["(str \"abc  def\" \"ghi jkl\")"],
        Some("\"abc  defghi jkl\""),
    );
}

#[test]
fn test116() {
    test_runner(&["(str \"abc\\ndef\\nghi\")"], Some("\"abc\\ndef\\nghi\""));
}

#[test]
fn test117() {
    test_runner(
        &["(str \"abc\\\\def\\\\ghi\")"],
        Some("\"abc\\\\def\\\\ghi\""),
    );
}

#[test]
fn test118() {
    test_runner(
        &["(str (list 1 2 \"abc\" \"\\\"\") \"def\")"],
        Some("\"(1 2 abc \\\")def\""),
    );
}

#[test]
fn test119() {
    test_runner(&["(str (list))"], Some("\"()\""));
}

// ; Testing prn
//
#[test]
fn test120() {
    test_runner(&["(prn)"], Some("nil"));
}

//  ""
#[test]
fn test121() {
    test_runner(&["(prn \"\")"], Some("nil"));
}

//  "abc"
#[test]
fn test122() {
    test_runner(&["(prn \"abc\")"], Some("nil"));
}

//  "abc  def" "ghi jkl"
//  "\""
#[test]
fn test123() {
    test_runner(&["(prn \"\\\"\")"], Some("nil"));
}

//  "abc\ndef\nghi"
#[test]
fn test124() {
    test_runner(&["(prn \"abc\\ndef\\nghi\")"], Some("nil"));
}

//  "abc\\def\\ghi"
//  (1 2 "abc" "\"") "def"
#[test]
fn test125() {
    test_runner(&["(prn (list 1 2 \"abc\" \"\\\"\") \"def\")"], Some("nil"));
}

// ; Testing println
//
#[test]
fn test126() {
    test_runner(&["(println)"], Some("nil"));
}

//
#[test]
fn test127() {
    test_runner(&["(println \"\")"], Some("nil"));
}

//  abc
#[test]
fn test128() {
    test_runner(&["(println \"abc\")"], Some("nil"));
}

//  abc  def ghi jkl
//  "
#[test]
fn test129() {
    test_runner(&["(println \"\\\"\")"], Some("nil"));
}

//  abc
//  def
//  ghi
#[test]
fn test130() {
    test_runner(&["(println \"abc\\ndef\\nghi\")"], Some("nil"));
}

//  abc\def\ghi
#[test]
fn test131() {
    test_runner(&["(println \"abc\\\\def\\\\ghi\")"], Some("nil"));
}

//  (1 2 abc ") def
#[test]
fn test132() {
    test_runner(
        &["(println (list 1 2 \"abc\" \"\\\"\") \"def\")"],
        Some("nil"),
    );
}

// >>> optional=True
// ;
// ; -------- Optional Functionality --------
// ; Testing keywords
#[test]
fn test133() {
    test_runner(&["(= :abc :abc)"], Some("true"));
}

#[test]
fn test134() {
    test_runner(&["(= :abc :def)"], Some("false"));
}

#[test]
fn test135() {
    test_runner(&["(= :abc \":abc\")"], Some("false"));
}

// ; Testing vector truthiness
#[test]
fn test136() {
    test_runner(&["(if [] 7 8)"], Some("7"));
}

// ; Testing vector printing
#[test]
fn test137() {
    test_runner(
        &["(pr-str [1 2 \"abc\" \"\\\"\"] \"def\")"],
        Some("\"[1 2 \\\"abc\\\" \\\"\\\\\\\"\\\"] \\\"def\\\"\""),
    );
}

#[test]
fn test138() {
    test_runner(&["(pr-str [])"], Some("\"[]\""));
}

#[test]
fn test139() {
    test_runner(
        &["(str [1 2 \"abc\" \"\\\"\"] \"def\")"],
        Some("\"[1 2 abc \\\"]def\""),
    );
}

#[test]
fn test140() {
    test_runner(&["(str [])"], Some("\"[]\""));
}

// ; Testing vector functions
#[test]
fn test141() {
    test_runner(&["(count [1 2 3])"], Some("3"));
}

#[test]
fn test142() {
    test_runner(&["(empty? [1 2 3])"], Some("false"));
}

#[test]
fn test143() {
    test_runner(&["(empty? [])"], Some("true"));
}

#[test]
fn test144() {
    test_runner(&["(list? [4 5 6])"], Some("false"));
}

// ; Testing vector equality
#[test]
fn test145() {
    test_runner(&["(= [] (list))"], Some("true"));
}

#[test]
fn test146() {
    test_runner(&["(= [7 8] [7 8])"], Some("true"));
}

#[test]
fn test147() {
    test_runner(&["(= (list 1 2) [1 2])"], Some("true"));
}

#[test]
fn test148() {
    test_runner(&["(= (list 1) [])"], Some("false"));
}

#[test]
fn test149() {
    test_runner(&["(= [] [1])"], Some("false"));
}

#[test]
fn test150() {
    test_runner(&["(= 0 [])"], Some("false"));
}

#[test]
fn test151() {
    test_runner(&["(= [] 0)"], Some("false"));
}

#[test]
fn test152() {
    test_runner(&["(= [] \"\")"], Some("false"));
}

#[test]
fn test153() {
    test_runner(&["(= \"\" [])"], Some("false"));
}

// ; Testing vector parameter lists
#[test]
fn test154() {
    test_runner(&["( (fn* [] 4) )"], Some("4"));
}

#[test]
fn test155() {
    test_runner(&["( (fn* [f x] (f x)) (fn* [a] (+ 1 a)) 7)"], Some("8"));
}

// ; Nested vector/list equality
#[test]
fn test156() {
    test_runner(&["(= [(list)] (list []))"], Some("true"));
}

#[test]
fn test157() {
    test_runner(
        &["(= [1 2 (list 3 4 [5 6])] (list 1 2 [3 4 (list 5 6)]))"],
        Some("true"),
    );
}
