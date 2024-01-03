use spinners::Spinner;
use spinners::Spinners;
use sqlx::migrate::Migrator;
use sqlx::Pool;
use sqlx::Postgres;
use std::path::Path;
use temboclient::models::ConnectionInfo;

pub struct SqlxUtils {}

impl SqlxUtils {
    // run sqlx migrate
    pub async fn run_migrations(connection_info: ConnectionInfo) -> Result<(), anyhow::Error> {
        let mut sp = Spinner::new(Spinners::Line, "Running SQL migration".into());

        let connection_string = format!(
            "postgresql://{}:{}@{}:{}",
            connection_info.user,
            connection_info.password,
            connection_info.host,
            connection_info.port
        );

        let pool = Pool::<Postgres>::connect(connection_string.as_str()).await?;

        let m = Migrator::new(Path::new("./migrations")).await?;
        m.run(&pool).await?;

        sp.stop_with_message("- SQL migration completed".to_string());

        Ok(())
    }
}