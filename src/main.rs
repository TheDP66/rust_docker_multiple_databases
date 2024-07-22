use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use multiple_databases::{routes::user_route::user_config, utils::config::Config, AppState};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    // initialize env variable
    let config = Config::init().to_owned();

    // setup pool connection
    let pool_codein = match PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url_codein)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the \"codein\" database is successful!");
            pool
        }
        Err(err) => {
            eprintln!("🔥 Failed to connect to the \"codein\" database: {:?}", err);
            std::process::exit(1)
        }
    };

    // setup pool connection
    let pool_iekimtie = match PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url_iekimtie)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the \"iekimtie\" database is successful!");
            pool
        }
        Err(err) => {
            eprintln!(
                "🔥 Failed to connect to the \"iekimtie\" database: {:?}",
                err
            );
            std::process::exit(1)
        }
    };

    // setup pool connection
    let pool_wkawan = match PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url_wkawan)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the \"wkawan\" database is successful!");
            pool
        }
        Err(err) => {
            eprintln!("🔥 Failed to connect to the \"wkawan\" database: {:?}", err);
            std::process::exit(1)
        }
    };

    // run migration
    match sqlx::migrate!("./migrations").run(&pool_codein).await {
        Ok(_) => println!("✅ Migrations \"codein\" executed successfully."),
        Err(e) => eprintln!("🔥 Error executing \"codein\" migrations: {}", e),
    };

    match sqlx::migrate!("./migrations").run(&pool_iekimtie).await {
        Ok(_) => println!("✅ Migrations \"iekimtie\" executed successfully."),
        Err(e) => eprintln!("🔥 Error executing \"iekimtie\" migrations: {}", e),
    };

    match sqlx::migrate!("./migrations").run(&pool_wkawan).await {
        Ok(_) => println!("✅ Migrations \"wkawan\" executed successfully."),
        Err(e) => eprintln!("🔥 Error executing \"wkawan\" migrations: {}", e),
    };

    let app_state = AppState {
        db_codein: pool_codein.clone(),
        db_iekimtie: pool_iekimtie.clone(),
        db_wkawan: pool_wkawan.clone(),
        config: config.clone(),
    };
    let port = config.clone().port;
    println!(
        "{}",
        format!("🚀 Server is running on port http://127.0.0.1:{}", port)
    );

    // setup server
    let server = HttpServer::new(move || {
        // configure cors
        let cors = Cors::default()
            // .allowed_origin("http://127.0.0.1:3000")
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .configure(user_config)
            .route("/", web::get().to(health_checker_handler))
    })
    .bind(("0.0.0.0", port))?;

    // run server
    server.run().await?;

    println!("🚀 Server started successfully");

    Ok(())
}

async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Healthy";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}
