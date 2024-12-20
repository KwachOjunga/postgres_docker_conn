use clap::{Parser, Subcommand};
use postgres::{tls::NoTls, Client, Error};

//const CONN : Client = Client::connect("postgres://postgres@localhost:5432", NoTls).unwrap();

fn create_table(conn: &mut Client) -> Result<(), Error> {
    conn.execute(
        "CREATE TABLE user (
			id SERIAL PRIMARY KEY,
			name VARCHAR NOT NULL,
			email VARCHAR NOT NULL
	)",
        &[],
    )
    .map(drop)
}

fn create_user(conn: &mut Client, name: &str, email: &str) -> Result<(), Error> {
    conn.execute(
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        &[&name, &email],
    )
    .map(drop)
}
fn list_user(conn: &mut Client) -> Result<Vec<(String, String)>, Error> {
    let res = conn
        .query("SELECT name,email FROM users", &[])?
        .into_iter()
        .map(|row| (row.get(0), row.get(1)))
        .collect();
    Ok(res)
}

fn main() {
    let conn = Client::connect("postgres://postgres@localhost:5432", NoTls).unwrap();
}
