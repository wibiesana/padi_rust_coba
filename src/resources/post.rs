use padi_core::prelude::*;

pub struct PostResource;

impl Resource for PostResource {
    fn to_array(item: &Value) -> Value {
        json!({
            "id": item["id"],
            "user": when_loaded(item, "user").unwrap_or_else(|| json!({
                "id": item["user_id"]
            })),
            "title": item["title"],
            "slug": item["slug"],
            "content": item["content"],
            "status": item["status"],
            "created_at": item["created_at"],
            "updated_at": item["updated_at"],
        })
    }
}
