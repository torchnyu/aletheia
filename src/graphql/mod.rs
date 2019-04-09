//! The graphql module contains the code necessary for GraphQL integration
//! with Aletheia. All that should be necessary is `use crate::graphql::*;`
//! and then you should be good to go.

// GraphQL done with juniper

// There should be an easier way to do this.
mod project;
mod tokenized;

pub use project::*;
pub use tokenized::*;
