use log::LevelFilter;

use rbatis::plugin::intercept::SqlIntercept;
use rbatis::plugin::log::LogPlugin;
use rbdc_mysql::driver::MysqlDriver;

use futures::executor::block_on;
use lazy_static::lazy_static;
use rbatis::rbatis::Rbatis;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use sea_orm::{ConnectOptions, DbErr};
use std::time::Duration;

lazy_static! {
    pub static ref RB: Rbatis = {
        let mut rb = InitDb::new("mysqlurl").0;
        return rb;
    };
}

lazy_static! {
    pub static ref database_connection: DatabaseConnection = {
        let mut opt = ConnectOptions::new("protocol://username:password@host/database".to_owned());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Debug);

        let db = block_on(run(""));
        return db.unwrap();
    };
}
async fn run(DATABASE_URL: &str) -> Result<DatabaseConnection, DbErr> {
    let db1 = Database::connect(DATABASE_URL).await?;
    Ok(db1)
}

pub struct InitDb(Rbatis);

impl InitDb {
    pub fn new(url: &str) -> InitDb {
        let mut rbatis: Rbatis = Rbatis::new();
        rbatis.set_sql_intercepts(vec![Box::new(Intercept {})]);
        rbatis
            .init(MysqlDriver {}, "options")
            .expect("connect database is error ");
        println!("rbatis init success");
        InitDb(rbatis)
    }
}

#[derive(Debug)]
pub struct Intercept {}

impl SqlIntercept for Intercept {
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    fn do_intercept(
        &self,
        rb: &Rbatis,
        sql: &mut String,
        args: &mut Vec<rbs::Value>,
        is_prepared_sql: bool,
    ) -> Result<(), rbatis::Error> {
        println!("执行sql:{:?},arg = {:?}", sql, args);
        Ok(())
    }
}

#[derive(Debug)]
pub struct RbatisLog {}

impl LogPlugin for RbatisLog {
    fn get_level_filter(&self) -> LevelFilter {
        LevelFilter::Debug
    }

    fn set_level_filter(&self, level: LevelFilter) {}

    fn get_level(&self, level: log::Level) -> log::Level {
        log::Level::Debug
    }

    fn set_level(&self, from: log::Level, to: log::Level) {
        todo!()
    }
}
