mod link;
mod user;

use async_graphql::{EmptySubscription, MergedObject, Schema};

use self::link::{LinkMutationRoot, LinkQueryRoot};
use self::user::{UserMutationRoot, UserQueryRoot};

#[derive(MergedObject, Default)]
pub struct MutationRoot(pub UserMutationRoot, pub LinkMutationRoot);

#[derive(MergedObject, Default)]
pub struct QueryRoot(pub UserQueryRoot, pub LinkQueryRoot);

pub type GraphQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
