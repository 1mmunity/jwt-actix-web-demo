use std::fmt::Debug;

use actix_web::{ResponseError, HttpResponseBuilder, http::{header::ContentType, StatusCode}};

use crate::resp::GenericResponse;

#[derive(Debug, Serialize, Clone)]
pub enum AppErrorType {
  DbError,
  HostError,
  CritError,
  AuthError,
  UserError
}

#[derive(Debug, Serialize, Clone)]
pub struct AppError {
  #[serde(skip_serializing)]
  pub status_code: u16,
  pub error_code: String,
  pub message: Option<String>,
  pub error_type: AppErrorType
}

impl std::fmt::Display for AppError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl ResponseError for AppError {
  fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
    HttpResponseBuilder::new(StatusCode::from_u16(self.status_code).unwrap())
    .insert_header(ContentType::json())
    .json(GenericResponse::err_res(self.status_code, self.clone())) 
  }
}

impl From<std::io::Error> for AppError {
  fn from(value: std::io::Error) -> Self {
    Self {
      status_code: 500,
      error_code: "CANNOT_BIND_TO_HOST".into(),
      message: Some(value.to_string()),
      error_type: AppErrorType::HostError
    }
  }
}

pub trait TAppError {
  fn db_error<T: ToString, R: ToString>(status_code: u16, error_code: T, message: Option<R>) -> Self;
  fn user_error<T: ToString, R: ToString>(status_code: u16, error_code: T, message: Option<R>) -> Self;
  fn crit_error<T: ToString + Debug, R: ToString + Debug>(logger: slog::Logger, error_code: T, message: Option<R>) -> Self;
  fn auth_error<T: ToString, R: ToString>(status_code: u16, error_code: T, message: Option<R>) -> Self;
}

impl TAppError for AppError {
  fn db_error<T: ToString, R: ToString>(status_code: u16, error_code: T, message: Option<R>) -> Self {
    Self {
      status_code: status_code,
      error_code: error_code.to_string(),
      message: message.map(|s| s.to_string()),
      error_type: AppErrorType::DbError
    }
  }

  fn user_error<T: ToString, R: ToString>(status_code: u16, error_code: T, message: Option<R>) -> Self {
    Self {
      status_code: status_code,
      error_code: error_code.to_string(),
      message: message.map(|s| s.to_string()),
      error_type: AppErrorType::UserError
    }
  }

  fn auth_error<T: ToString, R: ToString>(status_code: u16, error_code: T, message: Option<R>) -> Self {
    Self {
      status_code,
      error_code: error_code.to_string(),
      message: message.map(|s| s.to_string()),
      error_type: AppErrorType::AuthError,
    }
  }

  fn crit_error<T: ToString + Debug, R: ToString + Debug>(logger: slog::Logger, error_code: T, message: Option<R>) -> Self {
    crit!(logger, "unhandled error occured ({:?}): {:?}", error_code, message);
    Self {
      status_code: 500,
      error_code: error_code.to_string(),
      message: message.map(|s| s.to_string()),
      error_type: AppErrorType::CritError
    }
  }
}