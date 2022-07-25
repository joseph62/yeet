use clap::{arg, command};

const DEFAULT_EXPRESSION: &str = "yeet";

fn main() -> ! {
    let matches = command!()
        .about("Like the yes utility, but way cool!")
        .arg(arg!([expression] "Optional expression to output"))
        .get_matches();

    let expression = match matches.get_one::<String>("expression") {
        Some(expr) => expr,
        None => DEFAULT_EXPRESSION,
    };

    loop {
        println!("{}", expression);
    }
}
