#[path ="../src/utils.rs"]
mod utils;
use utils::*;

#[test]
fn group_repo() {
    assert_eq!(
        get_user_repo("git@github.com:HomyeeKing/gx.git"),
        "HomyeeKing/gx"
    );
}