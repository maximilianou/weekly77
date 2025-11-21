use clap::Parser;
use dotenvy::dotenv;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Database URL (sqlite). Use `sqlite::memory:` for in-memory.
    #[arg(long)]
    db_url: Option<String>,

    /// Initialize DB schema
    #[arg(long, default_value_t = false)]
    init_db: bool,

    /// Run demo printing to stdout
    #[arg(long, default_value_t = false)]
    demo: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let args = Args::parse();

    if args.demo {
        src02::run_demo().await;
        return Ok(());
    }

    // Determine DB URL: CLI -> ENV -> default (in-memory)
    let db_url = args
        .db_url
        .or_else(|| std::env::var("DB_URL").ok())
        .unwrap_or_else(|| "sqlite::memory:".to_string());

    println!("Using DB URL: {}", db_url);

    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    if args.init_db {
        src02::persistence::init_db(&pool).await?;
        println!("DB initialized at {}", db_url);
    }

    Ok(())
}
