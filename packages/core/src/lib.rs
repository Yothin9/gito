pub mod utils;
pub use utils::*;

use std::process::Command;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[wasm_bindgen]
#[allow(dead_code)]
pub struct GitInfo {
    // username:String,
    // email:String,
    ssh_url: String,
    user_repo: String,
}

#[wasm_bindgen]
pub fn get_git_info() -> GitInfo {
    let ssh_url = get_stdout(
        &Command::new("git")
            .args(["remote", "get-url", "origin"])
            .output()
            .unwrap(),
    );

    GitInfo {
        user_repo: get_user_repo(&ssh_url),
        ssh_url,
    }
}
