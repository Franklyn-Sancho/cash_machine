use BANKING_SYSTEM::{database::database::Database, menus::menus::login_register_menu};
use rusqlite::Result;

fn main() -> Result<()> {
    let db = Database::new("users.db");

    //cria uma tabela users no banco de dados
    db.create_tables();

    login_register_menu(&db);

    Ok(())
}







