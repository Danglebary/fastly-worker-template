use fastly::{Error, Request, Response};

use crate::utils::{env, logger};

mod routes;
mod utils;

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    let app_config = env::init();
    logger::init();

    routes::router(req, app_config)
}
