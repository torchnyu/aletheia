use super::RequestContext;
use crate::db::models::Medium;
use juniper::FieldResult;

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
        Ok(self.large_url()?)
    }

    field medium_url(&executor) -> FieldResult<String> {
        Ok(self.medium_url()?)
    }
    
    field thumbnail_url(&executor) -> FieldResult<String> {
        Ok(self.thumbnail_url()?)
    }
});
