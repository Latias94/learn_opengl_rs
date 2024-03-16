use learn_opengl_rs::run_tutorial;
use std::env;

fn main() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Info).expect("Cannot init logger");
        } else {
            env_logger::init();
        }
    }
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            // run_tutorial("1_2_1".to_string());
        } else {
            let args = env::args().collect::<Vec<String>>();
            if args.len() != 2 {
                println!(
                    "Call with the number of the tutorial, e.g. `1_1_2` for _1_2_hello_window_clear.rs"
                );
                std::process::exit(1);
            }

            let tutorial_id = &args[1];
            run_tutorial(tutorial_id.to_string());
        }
    }
}
