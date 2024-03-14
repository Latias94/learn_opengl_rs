pub mod _1_getting_started;
mod window;

pub use _1_getting_started::*;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("args: {:?}", args);
        println!(
            "Call with the number of the tutorial, e.g. `1_1_2` for _1_2_hello_window_clear.rs"
        );
        std::process::exit(1);
    }

    let tutorial_id = &args[1];
    match tutorial_id.as_str() {
        "1_1_1" => main_1_1_1(),
        "1_1_2" => main_1_1_2(),
        "1_2_1" => main_1_2_1(),
        _ => println!("Unknown tutorial id: {}", tutorial_id),
    }
}
