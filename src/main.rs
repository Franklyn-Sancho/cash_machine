mod account;
mod deposit;
mod withdraw;
mod menus;
mod database;
mod user;
mod authentication;
mod utils;

use database::Database;
use deposit::deposit;
use rusqlite::Result;
use withdraw::withdraw;
use menus::login_register_menu;

fn main() -> Result<()> {
    let db = Database::new("users.db");

    //cria uma tabela users no banco de dados
    db.create_tables();

    login_register_menu(&db);

    Ok(())
}







