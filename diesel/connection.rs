use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;

// The alias for both the Pool and a single connection retrieved from the pool
pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn create_pool() -> Pool {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(database_url);
    Pool::new(manager).expect("Failed to create db pool")
}

pub trait DataBaseConnection {
    fn connect(&self) -> DbConnection;
}

impl DataBaseConnection for Pool {
    fn connect(&self) -> DbConnection {
        self.get().unwrap()
    }
}
