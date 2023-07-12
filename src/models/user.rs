#[derive(Serialize)]
pub struct User {
  pub id: uuid::Uuid,
  pub username: String,
  pub email: String,

  #[serde(skip_serializing)]
  pub password: String
}