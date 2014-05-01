use parser::ArgumentParser;
use generic::Store;
use test_parser::{check_ok,check_err};

#[test]
fn test_str() {
    let mut val: ~str = ~"";
    let mut ap = ArgumentParser::new();
    ap.refer(&mut val)
      .add_option(~["-s", "--set"], ~Store::<~str>,
        "Set string value");
    check_ok(&ap, ~[~"./argparse_test", ~"-s", ~"10"]);
    assert!(val.eq(&~"10"));
    check_ok(&ap, ~[~"./argparse_test", ~"--set", ~"value"]);
    assert!(val.eq(&~"value"));
    check_err(&ap, ~[~"./argparse_test", ~"--set"]);
}
