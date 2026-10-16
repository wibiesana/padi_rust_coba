use padi_core::prelude::*;

use crate::controllers::comments;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(comments::index).post(comments::store))
        .route("/all", get(comments::all))
        .route("/{id}", get(comments::show).put(comments::update).delete(comments::destroy))
}
