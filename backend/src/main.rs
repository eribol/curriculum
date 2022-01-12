use moon::{
    *,
    actix_web::{
        web::ServiceConfig,
        http::StatusCode,
        http::header::ContentType,
        HttpResponse,
        App,
        middleware::{Logger, Compat, Condition, ErrorHandlers},
        get,
        web::{get,post},
        Responder,
    },
    config::CONFIG,
};
use moon::actix_http::Response;

async fn frontend() -> Frontend {
    Frontend::new()
        .title("Libredu")
        .default_styles(false)
        .append_to_head(r#"<link href="_api/public/css/bulmaswatch.min.css" rel="stylesheet"/>"#)
        .append_to_head(r#"<link href="_api/public/css/fontawesome/css/all.css" rel="stylesheet"/>"#)
        .body_content(r#"<div id="main"></div>"#)
}

async fn up_msg_handler(_req: UpMsgRequest<()>) {
}

#[moon::main]
async fn main() -> std::io::Result<()> {
    /*
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap()).await{
        Ok(db) => {
            println!("db okey");
            db}
        Err(_) => {panic!("no db connection")}
    };

     */

    let service_config = |service_config: &mut ServiceConfig| {
        //service_config.service(hello);
    };

    start(frontend, up_msg_handler, service_config).await
}