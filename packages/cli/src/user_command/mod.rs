// TODO: seprate file
pub mod add {
    use crate::constants::*;
    use ini::Ini;
    pub fn run(name: &str, email: &str, alias: &str) {
        let mut user = Ini::new();
        user.with_section(Some(alias))
            .set("name", name)
            .set("email", email);
        user.write_to_file(get_git_account_file()).unwrap();
        println!("Add {alias} successfully");
    }
}

pub mod list {
    use std::vec;

    use crate::constants::get_git_account_file;
    use ini::Ini;
    use prettytable::{row, Cell, Row, Table};
    pub fn run() {
        let mut git_account_table = Table::new();
        git_account_table.add_row(row!["alias", "name", "email"]);
        let i = Ini::load_from_file(get_git_account_file()).unwrap();
        for (sec, prop) in i.iter() {
            let mut group: Vec<Cell> = vec![];
            group.push(Cell::new(sec.unwrap_or_default()));
            for (_, v) in prop.iter() {
                group.push(Cell::new(v));
            }
            git_account_table.add_row(Row::new(group));
        }
        // TODO: mark current user
        git_account_table.printstd();
    }
}
