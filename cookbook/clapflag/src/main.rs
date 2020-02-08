use clap::{Arg, App};

fn main() {
    let matches = App::new("Test program")
        .version("0.1.0")
        .arg(Arg::with_name("file")
                .short("f").long("file").takes_value(true).help("a file"))
        .arg(Arg::with_name("num").short("n").long("number").takes_value(true).help("a number"))
        .get_matches();
    let my_file = matches.value_of("file").unwrap_or("input.txt");
    println!("{}", my_file);
}
