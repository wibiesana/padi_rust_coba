use padi_core::prelude::*;

use crate::controllers::posts;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(posts::index).post(posts::store))
        .route("/all", get(posts::all))
        .route("/{id}", get(posts::show).put(posts::update).delete(posts::destroy))
}
