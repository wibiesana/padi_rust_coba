use padi_core::prelude::*;
use super::base::post_base::BASE_POSTS;

pub const POSTS: Model = BASE_POSTS
    .relations(&[
        Relation::belongs_to("user", || crate::models::USERS, "user_id", "id")
            .columns(&["id", "name"]),
    ])
    .default_with(&["user"]);
