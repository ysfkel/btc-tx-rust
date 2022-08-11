use std;
use tokio_postgres::NoTls;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub async fn run_migrations(database_url: &str) -> std::result::Result<(), Error> {
    print!("running db migrations");

    let (mut client, con) = tokio_postgres::connect(database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = con.await {
            println!("connecrion error {}", e);
        }
    });

    let migration_report = embedded::migrations::runner()
        .run_async(&mut client)
        .await?;

    let migrations = migration_report.applied_migrations();

    println!("migrations {} ", migrations.len());

    for migration in migrations {
        println!(
            "Migration Applied -  Name: {}, Version: {}",
            migration.name(),
            migration.version()
        )
    }

    println!("DB migrations finished!");

    Ok(())
}
