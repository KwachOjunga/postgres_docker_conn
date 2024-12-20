use clap::{arg, Parser, Subcommand};
use postgres::{tls::NoTls, Client, Error};

//const CONN : Client = Client::connect("postgres://postgres@localhost:5432", NoTls).unwrap();

fn create_table(conn: &mut Client) -> Result<(), Error> {
    conn.execute(
        "CREATE TABLE users (
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


#[derive(Parser)]
#[command(version, about = "A cli that works as postgres client" )]
struct Cli {
	///Sets an address of db connection
	#[arg(short, long, value_name = "ADDR")]
	db : Option<String>,

	#[command(subcommand)]
	command: Option<Commands>
} 

#[derive(Subcommand)]
enum Commands {
	/// add users to table
	Add {
		/// user's name
		#[arg(long)]
		name: String,
		/// user's email
		#[arg(long)]
		email: String
	},
	/// create users table
	Create,
	/// list users in table
	List
}

fn main() {
	let cli = Cli::parse();
	let db = cli.db.as_deref().unwrap_or("postgres://postgres@localhost:8000");	
	let mut conn = Client::connect(db, NoTls).unwrap();
	match &cli.command {
		Some(Commands::Create) => {create_table(&mut conn).unwrap();},
		Some(Commands::Add { name, email }) => {create_user(&mut conn, name, email).unwrap();}
		Some(Commands::List) => {println!("{:?}",list_user(&mut conn).unwrap());},
		_ => /*println!("try using oe of the commands")*/{}
	}
}
