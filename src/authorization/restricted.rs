use crate::utils::Result;

trait Restricted {
    fn new(ctx: RequestContext, permissions: Vec<Permissions>, user: Option<User>) -> Result<Self> {
    }
}
