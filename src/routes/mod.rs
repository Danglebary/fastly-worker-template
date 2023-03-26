pub mod errors;
mod index;

use fastly::{Error, Request, Response};

use crate::utils::env::AppConfig;

use self::index::handle_index_route;

pub fn router(req: Request, app_config: AppConfig) -> Result<Response, Error> {
    match req.get_path() {
        "/" => handle_index_route(req, app_config),

        _ => {
            let err = errors::UnknownRouteError {};
            Err(err.into())
        }
    }
}
