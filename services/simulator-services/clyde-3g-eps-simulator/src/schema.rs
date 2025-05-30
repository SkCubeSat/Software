use crate::model::Subsystem;
use juniper;
use juniper::EmptySubscription;
use std::sync::Arc;

pub mod mutation;
pub mod query;

// This Context struct allows us to share state across requests
pub struct Context {
    pub subsystem: Arc<Subsystem>,
}

impl juniper::Context for Context {}

// Create a new RootNode to serve queries and mutations
pub type Schema =
    juniper::RootNode<'static, query::Root, mutation::Root, EmptySubscription<Context>>;
