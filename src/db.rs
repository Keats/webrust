use iron::typemap::Key;
use postgres::SslMode;
use r2d2;
use r2d2_postgres::PostgresConnectionManager;


pub type PostgresPool = r2d2::Pool<PostgresConnectionManager>;
pub type PostgresConnection = r2d2::PooledConnection<PostgresConnectionManager>;

pub struct PostgresDB;
impl Key for PostgresDB { type Value = PostgresPool;}


// Gets a connection from the pool from the given request or returns a 500
macro_rules! get_pg_connection {
    ($req:expr) => (match $req.get::<persistent::Read<db::PostgresDB>>() {
        Ok(pool) => match pool.get() {
            Ok(conn) => conn,
            Err(_) => {
                println!("Couldn't get a connection to pg!");
                return Ok(Response::with((status::InternalServerError)));
            }
        },
        Err(_) => {
            println!("Couldn't get the pg pool from the request!");
            return Ok(Response::with((status::InternalServerError)));
        }
    })
}

pub fn setup_database(conn: PostgresConnection) {
    conn.execute("DROP TABLE IF EXISTS passwords;", &[]).unwrap();
    conn.execute("CREATE TABLE passwords (id INT PRIMARY KEY, name VARCHAR(255), encrypted VARCHAR(255));", &[]).unwrap();
    conn.execute("INSERT INTO passwords VALUES (1, 'github', 'qwewqe£EWQC2)');", &[]).unwrap();
    conn.execute("INSERT INTO passwords VALUES (2, 'facebook', 'scanfd;l392');", &[]).unwrap();
}

pub fn get_pool(uri: &str) -> PostgresPool {
    let manager = PostgresConnectionManager::new(uri, SslMode::None).unwrap();

    r2d2::Pool::new(r2d2::Config::default(), manager).unwrap()
}
