use clap::{arg, Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("vendedor")
        .version("0.0.1")
        .author("Helio Frota")
        .about("Reverse echo")
        .arg(
            Arg::new("text")
                .action(ArgAction::Append)
                .value_name("TEXT")
                .help("Input text")
                .required(true),
        )
        .arg(arg!(-n --nonewline "No new line").action(ArgAction::SetTrue))
        .get_matches();

    let args = matches
        .get_many::<String>("text")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>()
        .join(" ");

    let args_reversed: String = args.chars().rev().collect();

    if matches.get_flag("nonewline") {
        print!("{}", args_reversed);
    } else {
        println!("{}", args_reversed);
    }
}
