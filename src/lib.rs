mod camera;
mod shader;
mod window;

mod _1_getting_started;
mod _2_lighting;

use _1_getting_started::*;
use _2_lighting::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

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
        "1_3_2" => main_1_3_2(),
        "1_3_3" => main_1_3_3(),
        "1_3_4" => main_1_3_4(),
        "1_3_5" => main_1_3_5(),
        "1_3_6" => main_1_3_6(),
        "1_4_1" => main_1_4_1(),
        "1_4_2" => main_1_4_2(),
        "1_4_3" => main_1_4_3(),
        "1_4_4" => main_1_4_4(),
        "1_4_5" => main_1_4_5(),
        "1_4_6" => main_1_4_6(),
        "1_5_1" => main_1_5_1(),
        "1_5_2" => main_1_5_2(),
        "1_5_3" => main_1_5_3(),
        "1_6_1" => main_1_6_1(),
        "1_6_2" => main_1_6_2(),
        "1_6_3" => main_1_6_3(),
        "1_6_4" => main_1_6_4(),
        "1_7_1" => main_1_7_1(),
        "1_7_2" => main_1_7_2(),
        "1_7_3" => main_1_7_3(),
        "1_7_4" => main_1_7_4(),
        "1_7_5" => main_1_7_5(),
        "1_7_6" => main_1_7_6(),
        "2_1_1" => main_2_1_1(),
        "2_2_1" => main_2_2_1(),
        "2_2_2" => main_2_2_2(),
        "2_2_3" => main_2_2_3(),
        "2_2_4" => main_2_2_4(),
        _ => log::error!("Unknown tutorial id: {}", tutorial_id),
    }
}
