use parser::ArgumentParser;
use generic::Store;
use test_parser::{check_ok,check_err};

#[test]
fn test_argument() {
    let mut ap = ArgumentParser::new();
    let mut val = 0;
    ap.refer(&mut val).add_argument("value", ~Store::<int>, "The value");
    check_ok(&ap, ~[~"./argparse_test", ~"10"]);
    assert_eq!(val, 10);
    check_err(&ap, ~[~"./argparse_test", ~"10", ~"20"]);
    check_err(&ap, ~[~"./argparse_test", ~"test", ~"20"]);
    check_err(&ap, ~[~"./argparse_test", ~"1.5"]);
}

#[test]
fn test_two() {
    let mut ap = ArgumentParser::new();
    let mut val1 = 1;
    let mut val2 = 2;
    ap.refer(&mut val1).add_argument("v1", ~Store::<int>, "The value 1");
    ap.refer(&mut val2).add_argument("v2", ~Store::<int>, "The value 2");
    check_ok(&ap, ~[~"./argparse_test", ~"10"]);
    assert_eq!(val1, 10);
    check_ok(&ap, ~[~"./argparse_test", ~"11", ~"21"]);
    assert_eq!(val1, 11);
    assert_eq!(val2, 21);
    check_err(&ap, ~[~"./argparse_test", ~"10", ~"20", ~"30"]);
    check_err(&ap, ~[~"./argparse_test", ~"test", ~"20"]);
    check_err(&ap, ~[~"./argparse_test", ~"1.5"]);
}

#[test]
fn test_positional_optional() {
    let mut ap = ArgumentParser::new();
    let mut val1 = 1;
    let mut val2 = 2;
    ap.refer(&mut val1)
        .add_option(~["--v1"], ~Store::<int>, "The value 1")
        .add_argument("v1", ~Store::<int>, "The value 1");
    ap.refer(&mut val2).add_argument("v2", ~Store::<int>, "The value 2");
    check_ok(&ap, ~[~"./argparse_test", ~"10"]);
    assert_eq!(val1, 10);
    check_ok(&ap, ~[~"./argparse_test", ~"11", ~"21"]);
    assert_eq!(val1, 11);
    assert_eq!(val2, 21);
    println!("THIS TEST");
    check_ok(&ap, ~[~"./argparse_test", ~"--v1=7", ~"8"]);
    assert_eq!(val1, 7);
    assert_eq!(val2, 8);
    println!("END TEST");
    check_ok(&ap, ~[~"./argparse_test", ~"10", ~"--v1=9"]);
    assert_eq!(val1, 9);
    assert_eq!(val2, 10);
    check_err(&ap, ~[~"./argparse_test", ~"--v1=10", ~"20", ~"30"]);
}
