// main.rs
mod account;
mod auth;
mod deposit;
mod withdraw;
mod model;
mod menus;

use deposit::deposit;
use model::create_tables;
use rusqlite::{Connection, Result};
use withdraw::withdraw;
use menus::login_register_menu;

fn main() -> Result<()> {
    let conn = Connection::open("users.db")?;

    //cria uma tabela users no banco de dados
    create_tables(&conn);

    login_register_menu(&conn);

    Ok(())
}







