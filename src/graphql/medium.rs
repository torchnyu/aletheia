use super::RequestContext;
use crate::db::models::Medium;
use juniper::FieldResult;
use std::env;

graphql_object!(Medium: RequestContext |&self| {
    description: "A piece of media, mostly photos for now"

    field id(&executor) -> i32 {
        self.id
    }

    // You probably want to use url
    field folder_name(&executor) -> &str {
        &self.folder_name
    }

    field large_url(&executor) -> FieldResult<String> {
        let bucket_name = env::var("BUCKET_NAME")?;
        Ok(format!("https://s3.amazonaws.com/{}/{}/{}", bucket_name, self.folder_name, "large.png"))
    }
});
