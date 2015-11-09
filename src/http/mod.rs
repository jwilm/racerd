use iron::{Iron, Chain};

use super::Config;

mod definition;
mod file;
mod completion;

/// Start the http server using the given configuration
///
/// `serve` is a blocking call which runs for the duration of the HTTP server.
///
/// # Example
///
/// ```no_run
/// # use libracerd::{Config, http};
/// let cfg = Config {
///     port: 3000,
///     secret_file: "/tmp/secret".to_string()
/// };
///
/// http::serve(cfg);
/// ```
///
pub fn serve(config: Config) {
    // For debugging. Torn about `log` crate since there's potentially a lot of noise from our
    // dependencies.
    println!("{:?}", config);

    let chain = Chain::new(router!(
        post "/parse_file"       => file::parse,
        post "/find_definition"  => definition::find,
        post "/list_completions" => completion::list,
        get  "/ping"             => handle::ping));

    let host = format!("localhost:{}", config.port);

    // TODO return this result instead of unwrapping. Need to add error type, first.
    Iron::new(chain).http(&host[..]).unwrap();
}

/// Handlers for Iron
#[allow(unused_variables)]
mod handle {
    use iron::prelude::*;
    use iron::status;

    /// Check if the server is accepting requests
    pub fn ping(req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "{\"pong\": true}")))
    }
}
