use serde::Serialize;

use crate::errors::TAppError;

#[derive(Serialize, Debug)]
pub struct GenericResponse<T> {
  success: bool,
  status: u16,
  content: T
}

impl <T: Serialize> GenericResponse<T> {
  pub fn ok_res(status: u16, content: T) -> Self {
    Self {
      success: true,
      status,
      content
    }
  }

  pub fn err_res(status: u16, err: T) -> Self
  where T: Serialize + TAppError
  {
    // we may need logger here
    Self {
      success: false,
      status,
      content: err
    }
  }

  pub fn crit_res(logger: slog::Logger, content: T) -> Self
  where T: Serialize + TAppError
  {
    crit!(logger, "{:?}", serde_json::to_string(&content));
    Self {
      success: false,
      status: 500,
      content
    }
  }
}

#[macro_export]
macro_rules! ok_res {
  ($status: expr, $content: expr) => {
    ($crate::actix_web::web::Json($crate::resp::GenericResponse::ok_res($status, $content)), $crate::actix_web::http::StatusCode::from_u16($status).unwrap())
  };

  ($status: expr, $content: expr, $cookie: expr) => {
    $crate::actix_web::HttpResponseBuilder::new($crate::actix_web::http::StatusCode::from_u16($status).unwrap())
    .cookie($cookie)
    .json($crate::resp::GenericResponse::ok_res($status, $content))
  };

  ($status: expr, $content: expr, $( $headers:expr ),*) => {
    let mut __response = $crate::actix_web::HttpResponseBuilder::new($crate::actix_web::http::StatusCode::from_u16($status).unwrap())
    for hdr in [$($headers),*] {
      __response.insert_header(hdr.0, hdr.1);
    }
    __response.json($crate::resp::GenericResponse::ok_res($status, $content))
    __response
  };
}

#[macro_export]
macro_rules! err_res {
  ($status: expr, $content: expr) => {
    ($crate::actix_web::web::Json($crate::resp::GenericResponse::err_res($status, $content)), $crate::actix_web::http::StatusCode::from_u16($status).unwrap())
  };
}

#[macro_export]
macro_rules! crit_res {
  ($logger: expr, $content: expr) => {
    ($crate::actix_web::web::Json($crate::resp::GenericResponse::crit_res($logger, $content)), $crate::actix_web::http::StatusCode::from_16(500).unwrap())
  };
}


#[macro_export]
macro_rules! ok_res_inner {
  ($status: expr, $content: expr) => {
    $crate::resp::GenericResponse::ok_res($status, $content)
  };
}

#[macro_export]
macro_rules! err_res_inner {
  ($status: expr, $content: expr) => {
    $crate::resp::GenericResponse::err_res($status, $content)
  };
}

#[macro_export]
macro_rules! crit_res_inner {
  ($logger: expr, $content: expr) => {
    $crate::resp::GenericResponse::crit_res($logger, $content)
  };
}
