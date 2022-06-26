use enum_map::{enum_map, Enum};
use std::env;

// if an out of bound index is passed to the tag element vector
// a panic will occur. This is the expected behavior.
// tagparam is not nor will it ever be an input validation tool.
// the purpose is to retreive elements from a spiltable string
// and nothing more.

// Enum to handle the index lookups
#[derive(Debug, Enum)]
enum ArgType {
    DelimOrHelp,
    Index,
    Tag,
}

fn main() {

    // Mapped index values to enum targets
    let map = enum_map! {
        ArgType::DelimOrHelp => 1,
        ArgType::Index => 2,
        ArgType::Tag => 3,
    };

    // Validate index map
    assert_eq!(map[ArgType::DelimOrHelp], 1);
    assert_eq!(map[ArgType::Index], 2);
    assert_eq!(map[ArgType::Tag], 3);

    let version = env!("CARGO_PKG_VERSION");
    let args: Vec<String> = env::args().collect();

    if &args[map[ArgType::DelimOrHelp]] == "-h" || &args[map[ArgType::DelimOrHelp]] == "--help" {
        println!("{} {}", "TagPram Version", version);
        println!(
            "{}",
            "Usage: tagpram delimiter index tag_string\n\
    Example: tagpram - 1 dev-appX-uuid\n\
    Results in the output: dev"
        );
    } else {
        let tag = &args[map[ArgType::Tag]];
        let index: usize = args[map[ArgType::Index]].parse().unwrap();
        let delim = &args[map[ArgType::DelimOrHelp]];
        let tag_element: Vec<&str> = tag.split(delim).collect();
        print!("{}", tag_element[index - map[ArgType::DelimOrHelp]]);
    }
}
