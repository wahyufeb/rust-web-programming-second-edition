use clap::{Command, Arg};

fn main() {
    let app = Command::new("booking")
        .version("1.0")
        .about("Books in a user")
        .author("Maxwell Flitton");

    let first_name = Arg::new("first_name")
        .long("f")
        .value_name("FIRST_NAME")
        .help("first name of user")
        .required(true);

    let last_name = Arg::new("last_name")
        .long("l")
        .value_name("LAST_NAME")
        .help("last name of user")
        .required(true);

    let age = Arg::new("age")
        .long("a")
        .value_name("AGE")
        .help("age of user")
        .required(true);
    
    let app = app.arg(first_name).arg(last_name).arg(age);
    let matches = app.get_matches();

    let name = matches.get_one::<String>("first_name").expect("First name is required");
    let last_name = matches.get_one::<String>("last_name").expect("Last name is required");
    let age = matches.get_one::<String>("age").expect("Age is required");

    println!("{:?}", name);
    println!("{:?}", last_name);
    println!("{:?}", age);
}