use fastly::{
    http::{header, Method, StatusCode},
    Error, Request, Response,
};

use crate::utils::env::AppConfig;

use super::errors;

const ALLOWED_METHODS: &'static str = "GET, POST HEAD, OPTIONS";

// General flow here is to explicitly allow the methods we want to allow, and return an `UnexpectedMethodError` for the rest
pub fn handle_index_route(req: Request, _app_config: AppConfig) -> Result<Response, Error> {
    match req.get_method() {
        // Handle unsupported methods first
        &Method::PUT | &Method::TRACE => unsupported_methods(),
        // Handle OPTION and HEAD methods
        &Method::OPTIONS => options(),
        &Method::HEAD => head(),

        // Handle CRUD methods
        &Method::GET => get(req),
        &Method::POST => post(req),
        &Method::PATCH => patch(req),
        &Method::DELETE => delete(req),

        _ => {
            let err = errors::UnexpectedMethodError {};
            Err(err.into())
        }
    }
}

fn get(_req: Request) -> Result<Response, Error> {
    Ok(Response::from_body("GET '/'").with_status(StatusCode::OK))
}

fn post(req: Request) -> Result<Response, Error> {
    let body_str: String = req.into_body_str();

    Ok(Response::from_body(format!("recieved: {body_str}")).with_status(StatusCode::OK))
}

fn patch(req: Request) -> Result<Response, Error> {
    let body_str: String = req.into_body_str();
    Ok(Response::from_body(format!("recieved: {body_str}")).with_status(StatusCode::OK))
}

fn delete(req: Request) -> Result<Response, Error> {
    let body_str: String = req.into_body_str();
    Ok(Response::from_body(format!("recieved: {body_str}")).with_status(StatusCode::OK))
}

fn unsupported_methods() -> Result<Response, Error> {
    Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
        .with_header(header::ALLOW, ALLOWED_METHODS)
        .with_body_text_plain("This method is not allowed"))
}

fn options() -> Result<Response, Error> {
    Ok(Response::from_status(StatusCode::OK).with_header(header::ALLOW, ALLOWED_METHODS))
}

fn head() -> Result<Response, Error> {
    Ok(Response::from_status(StatusCode::OK))
}
