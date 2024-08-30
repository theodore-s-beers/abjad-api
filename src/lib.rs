#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::future_not_send)]

use abjad::{Abjad, AbjadPrefs};
use worker::{event, Context, Env, Request, Response, ResponseBuilder, Result};

#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let url = req.url()?;
    let input = url
        .query_pairs()
        .find(|(key, _)| key == "input")
        .unwrap_or_default()
        .1;

    if input.is_empty() {
        return ResponseBuilder::new()
            .with_status(400)
            .from_json(&"Missing input parameter");
    }

    Response::from_json(&input.abjad(AbjadPrefs::default()))
}
