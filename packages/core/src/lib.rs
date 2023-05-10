pub mod utils;
pub use utils::*;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[wasm_bindgen(getter_with_clone)]
#[allow(dead_code)]
#[derive(Debug)]
pub struct GitInfo {
    username:String,
    email:String,
    ssh_url: String,
    user_repo: String,
}

#[wasm_bindgen]
pub async fn get_git_info() -> GitInfo {
    let (ssh_url, username, email) = tokio::join!(
        run_git_async(vec!["config", "remote.origin.url"]),
        run_git_async(vec!["config", "user.name"]),
        run_git_async(vec!["config", "user.email"])
    );
    println!("{}",ssh_url);
    GitInfo {
      user_repo: get_user_repo(&ssh_url),
      ssh_url,
      username,
      email
  }
}
