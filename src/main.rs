use std::process::exit;

fn parse_args() -> String {
    let args: Vec<String> = std::env::args().collect();

    let input_file = match args.len() {
        2 => &args[1],
        _ => {
            println!("Only one input file can be given");
            exit(1);
        },
    };

    return input_file.to_owned();
}

fn main() {
    let input_file = parse_args();

    if input_file.ends_with(".qoi") {
        println!("Got QOI input file");
    } else if input_file.ends_with(".png") {
        println!("Got PNG input file");
    } else {
        println!("Unknwon file type");
        exit(1);
    }
}
