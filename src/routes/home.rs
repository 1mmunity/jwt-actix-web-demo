use actix_web::Responder;

#[get("/")]
async fn home() -> impl Responder {
  ok_res!(200u16, "server is OK")
}