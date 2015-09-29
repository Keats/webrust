use postgres::error::Error;
use db;

#[derive(Serialize, Deserialize, Debug)]
pub struct Password {
    id: i32,
    name: String,
    encrypted: String,
}


pub fn list_passwords(conn: db::PostgresConnection) -> Result<Vec<Password>, Error> {
    let stmt = try!(conn.prepare("SELECT id, name, encrypted from passwords"));
    let mut passwords: Vec<Password> = Vec::new();

    for row in try!(stmt.query(&[])) {
        passwords.push(Password {
            id: row.get("id"),
            name: row.get("name"),
            encrypted: row.get("encrypted"),
        });
    }

    Ok(passwords)
}
