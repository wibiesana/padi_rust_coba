use padi_core::prelude::*;

pub struct CommentResource;

impl Resource for CommentResource {
    fn to_array(item: &Value) -> Value {
        json!({
            "id": item["id"],
            "post_id": item["post_id"],
            "user_id": item["user_id"],
            "comment": item["comment"],
            "status": item["status"],
            "created_at": item["created_at"],
            "updated_at": item["updated_at"],
        })
    }
}
