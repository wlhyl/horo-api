use actix_web::{body::BoxBody, HttpResponse, Responder as ActixxWebResponder, http::StatusCode};
use serde::Serialize;



/// 返回给前㾍的统一结构体
#[derive(Serialize)]
pub enum Responser<T: Serialize> {
    Ok(T),
    Created(T),
}

impl<T: Serialize> Responser<T> {
    fn code(&self) -> StatusCode {
        match self {
            Responser::Ok(_) => StatusCode::OK,
            Responser::Created(_) => StatusCode::CREATED,
        }
    }
    fn data(&self) -> &T {
        match self {
            Responser::Ok(data) => data,
            Responser::Created(data) => data,
        }
    }
}

impl<T: Serialize> ActixxWebResponder for Responser<T> {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        // let code = self.code();
        // let json = serde_json::to_string(self.data());
        // match json {
        // Ok(json) =>
        HttpResponse::build(self.code()).json(self.data())
        //     Err(error) => {
        //         let error = format!("Serialize error: {}", error);
        //         error!("Serialize error: {}", error);
        //         HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(error)
        //     }
        // }
    }
}
