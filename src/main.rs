use lib_learn_opengl_rs::run_tutorial;
use std::env;

fn main() {
    env_logger::init();

    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!(
            "Call with the number of the tutorial, e.g. `1_1_2` for _1_2_hello_window_clear.rs"
        );
        std::process::exit(1);
    }

    let tutorial_id = &args[1];
    unsafe {
        pollster::block_on(run_tutorial(tutorial_id.to_string()));
    }
}
