use std::{
    collections::HashMap,
    process::{self, Command, Output},
};

use regex::Regex;

pub async fn run(name: &str) {
    println!("the upstream name is {name}");
    // detect whether given upstream name exists
    let upstream_url = get_stdout(
        &Command::new("git")
            .args(["remote", "get-url", name])
            .output()
            .unwrap(),
    );
    println!("upstream_url is {upstream_url}");
    if upstream_url.trim().len() > 0 {
        eprintln!("`{name}` has existed, please check or input a new name");
    } else {
        println!("Ready to get upstream");
        let origin_remote = get_stdout(
            &Command::new("git")
                .args(["remote", "get-url", "origin"])
                .output()
                .unwrap(),
        );
        let user_repo = get_user_repo(&origin_remote);
        get_repo_meta_info(&user_repo).await;
    }
}

fn get_user_repo(remote_url: &str) -> String {
    // r means raw string https://doc.rust-lang.org/stable/reference/tokens.html#raw-string-literals
    let re: Regex = Regex::new(r"^git@github\.com:(.*)\.git$").unwrap();
    let caps = re.captures(remote_url).unwrap();
    return caps[1].to_string();
}

fn get_stdout(output: &Output) -> String {
    return (String::from_utf8(output.stdout.clone()))
        .unwrap()
        .trim()
        .to_string();
}

async fn get_repo_meta_info(user_repo: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("enter req, {}", user_repo);
    let res = reqwest::get(format!("https://api.github.com/repos/{}", user_repo))
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    dbg!(res);
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::get_user_repo;

    #[test]
    fn user_repo() {
        assert_eq!(
            get_user_repo("git@github.com:HomyeeKing/gx.git"),
            "HomyeeKing/gx"
        );
    }
}
