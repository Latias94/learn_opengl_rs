pub mod _1_getting_started;
mod window;

pub use _1_getting_started::*;
use std::env;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn run_tutorial(tutorial_id: String) {
    log::info!("Running tutorial {}", tutorial_id);
    match tutorial_id.as_str() {
        "1_1_1" => main_1_1_1(),
        "1_1_2" => main_1_1_2(),
        "1_2_1" => main_1_2_1(),
        "1_2_2" => main_1_2_2(),
        "1_2_3" => main_1_2_3(),
        "1_2_4" => main_1_2_4(),
        "1_2_5" => main_1_2_5(),
        "1_3_1" => main_1_3_1(),
        _ => log::error!("Unknown tutorial id: {}", tutorial_id),
    }
}
