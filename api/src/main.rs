use actix_web::{middleware::Logger, web, App, HttpServer};
use std::{env, net::SocketAddrV4};

#[cfg(feature = "swagger")]
use utoipa_swagger_ui::Url;

#[cfg(feature = "cors")]
use actix_cors::Cors;

use clap::Parser;
use horo_api::{
    args,
    routers::{health_routes, horo_routes},
    state::AppState,
};

#[cfg(feature = "swagger")]
use horo_api::swagger::{horo::HoroApiDoc, qizheng::QiZhengApiDoc};

#[cfg(feature = "swagger")]
use utoipa::OpenApi;

#[cfg(feature = "swagger")]
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let log4rs_config = env::var("LOG4RS_CONFIG")
        .expect("没设置 LOG4RS_CONFIG 环境变量，可在.env文件中设置或export LOG4RS_CONFIG=...");

    log4rs::init_file(&log4rs_config, Default::default())
        .map_err(|error| format!("配置日志系统失败，日志配置文件：{log4rs_config}, {error}"))
        .unwrap();

    let ephe_path = env::var("EPHE_PATH")
        .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");

    let shared_data = web::Data::new(AppState { ephe_path });

    let args = args::Args::parse();

    #[cfg(feature = "swagger")]
    let horo_openapi = HoroApiDoc::openapi();

    #[cfg(feature = "swagger")]
    let qizheng_openapi = QiZhengApiDoc::openapi();

    HttpServer::new(move || {
        #[cfg(feature = "cors")]
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        let app = App::new()
            .app_data(shared_data.clone())
            .configure(health_routes)
            .service(web::scope("/api").configure(horo_routes));
        #[cfg(feature = "swagger")]
        let app = app.service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![
            (
                Url::new("horo_api", "/api-docs/horo.json"),
                horo_openapi.clone(),
            ),
            (
                Url::new("qizheng_api", "/api-docs/qizheng.json"),
                qizheng_openapi.clone(),
            ),
        ]));

        #[cfg(feature = "cors")]
        let app = app.wrap(cors);

        let app = app.wrap(Logger::default());
        app
    })
    .workers(args.n)
    .bind(SocketAddrV4::new(args.ip, args.port))?
    .run()
    .await
}
