mod camera;
mod mesh;
mod model;
mod resources;
mod shader;
mod texture;
mod window;

mod _1_getting_started;
mod _2_lighting;
mod _3_model_loading;
mod _4_advanced_opengl;

use _1_getting_started::*;
use _2_lighting::*;
use _3_model_loading::*;
use _4_advanced_opengl::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[allow(clippy::missing_safety_doc)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub async unsafe fn run_tutorial(tutorial_id: String) {
    log::info!("Running tutorial {}", tutorial_id);
    match tutorial_id.as_str() {
        "1_1_1" => main_1_1_1().await,
        "1_1_2" => main_1_1_2().await,
        "1_2_1" => main_1_2_1().await,
        "1_2_2" => main_1_2_2().await,
        "1_2_3" => main_1_2_3().await,
        "1_2_4" => main_1_2_4().await,
        "1_2_5" => main_1_2_5().await,
        "1_3_1" => main_1_3_1().await,
        "1_3_2" => main_1_3_2().await,
        "1_3_3" => main_1_3_3().await,
        "1_3_4" => main_1_3_4().await,
        "1_3_5" => main_1_3_5().await,
        "1_3_6" => main_1_3_6().await,
        "1_4_1" => main_1_4_1().await,
        "1_4_2" => main_1_4_2().await,
        "1_4_3" => main_1_4_3().await,
        "1_4_4" => main_1_4_4().await,
        "1_4_5" => main_1_4_5().await,
        "1_4_6" => main_1_4_6().await,
        "1_5_1" => main_1_5_1().await,
        "1_5_2" => main_1_5_2().await,
        "1_5_3" => main_1_5_3().await,
        "1_6_1" => main_1_6_1().await,
        "1_6_2" => main_1_6_2().await,
        "1_6_3" => main_1_6_3().await,
        "1_6_4" => main_1_6_4().await,
        "1_7_1" => main_1_7_1().await,
        "1_7_2" => main_1_7_2().await,
        "1_7_3" => main_1_7_3().await,
        "1_7_4" => main_1_7_4().await,
        "1_7_5" => main_1_7_5().await,
        "1_7_6" => main_1_7_6().await,
        "2_1_1" => main_2_1_1().await,
        "2_2_1" => main_2_2_1().await,
        "2_2_2" => main_2_2_2().await,
        "2_2_3" => main_2_2_3().await,
        "2_2_4" => main_2_2_4().await,
        "2_2_5" => main_2_2_5().await,
        "2_3_1" => main_2_3_1().await,
        "2_3_2" => main_2_3_2().await,
        "2_4_1" => main_2_4_1().await,
        "2_4_2" => main_2_4_2().await,
        "2_4_3" => main_2_4_3().await,
        "2_4_4" => main_2_4_4().await,
        "2_4_5" => main_2_4_5().await,
        "2_5_1" => main_2_5_1().await,
        "2_5_2" => main_2_5_2().await,
        "2_5_3" => main_2_5_3().await,
        "2_5_4" => main_2_5_4().await,
        "2_6_1" => main_2_6_1().await,
        "2_6_2" => main_2_6_2().await,
        "3_1_1" => main_3_1_1().await,
        "4_1_1" => main_4_1_1().await,
        _ => log::error!("Unknown tutorial id: {}", tutorial_id),
    }
}
