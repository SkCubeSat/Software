//
// Copyright (C) 2018 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use juniper::{
    Context as JuniperContext, EmptySubscription, GraphQLType, GraphQLTypeAsync, RootNode,
};
use log::info;
use radsat_system::Config;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

use warp::{reply::Reply, Filter};

/// Context struct used by a service to provide Juniper context,
/// subsystem access and persistent storage.
#[derive(Clone)]
pub struct Context<T> {
    /// The subsystem instance this context provides access to
    pub subsystem: T,
    /// Persistent key-value storage for the service
    pub storage: Arc<RwLock<HashMap<String, String>>>,
}

impl<T> JuniperContext for Context<T> {}

impl<T> Context<T> {
    /// Returns a reference to the context's subsystem instance
    pub fn subsystem(&self) -> &T {
        &self.subsystem
    }

    /// Attempts to get a value from the context's storage
    ///
    /// # Arguments
    ///
    /// `name` - Key to search for in storage
    pub fn get(&self, name: &str) -> String {
        let stor = self.storage.read().unwrap();
        match stor.get(&name.to_string()) {
            Some(s) => s.clone(),
            None => "".to_string(),
        }
    }

    /// Sets a value in the context's storage
    ///
    /// # Arguments
    ///
    /// `key` - Key to store value under
    /// `value` - Value to store
    pub fn set(&self, key: &str, value: &str) {
        let mut stor = self.storage.write().unwrap();
        stor.insert(key.to_string(), value.to_string());
    }

    /// Clears a single key/value from storage
    ///
    /// # Arguments
    ///
    /// `key` - Key to clear (along with corresponding value)
    pub fn clear(&self, name: &str) {
        let mut storage = self.storage.write().unwrap();
        storage.remove(name);
    }

    /// Clears all key/value pairs from storage
    pub fn clear_all(&self) {
        self.storage.write().unwrap().clear();
    }
}

/// This structure represents a hardware service.
///
/// Specifically the functionality provided by this struct
/// exists to provide a GraphQL interface over HTTP, a means
/// of exposing a subsystem to GraphQL queries and means
/// for persistence throughout GraphQL queries.
///
/// ### Examples
///
/// # Creating and starting a service.
/// ```rust,ignore
/// use kubos_service::Service;
///
/// let sub = model::Subsystem::new();
/// Service::new(
///     "example-service",
///     sub,
///     schema::QueryRoot,
///     schema::MutationRoot,
/// ).start();
/// ```
pub struct Service {
    config: Config,
    // Using a boxed filter that returns any type implementing Reply
    filter: warp::filters::BoxedFilter<(Box<dyn Reply>,)>,
}

impl Service {
    /// Creates a new service instance
    ///
    /// # Arguments
    ///
    /// `name` - The name of the service. This is used to find the appropriate config information
    /// `subsystem` - An instance of the subsystem struct. This one instance will be used by all queries.
    /// `query` - The root query struct holding all other GraphQL queries.
    /// `mutation` - The root mutation struct holding all other GraphQL mutations.
    pub fn new<Query, Mutation, S>(
        config: Config,
        subsystem: S,
        query: Query,
        mutation: Mutation,
    ) -> Self
    where
        Query: GraphQLType<Context = Context<S>, TypeInfo = ()>
            + GraphQLTypeAsync<Context = Context<S>>
            + Send
            + Sync
            + 'static,
        Mutation: GraphQLType<Context = Context<S>, TypeInfo = ()>
            + GraphQLTypeAsync<Context = Context<S>>
            + Send
            + Sync
            + 'static,
        S: Send + Sync + Clone + 'static,
    {
        // Create the root node with query, mutation, and empty subscription
        let root_node = RootNode::new(query, mutation, EmptySubscription::<Context<S>>::new());

        let context = Context {
            subsystem,
            storage: Arc::new(RwLock::new(HashMap::new())),
        };

        // Wrap the root node in an Arc to share it
        let schema = Arc::new(root_node);

        // Clone context for filter use
        let ctx = context.clone();

        // Create the GraphQL routes
        // POST /graphql -> GraphQL API
        let graphql =
            warp::path("graphql")
                .and(warp::post())
                .and(juniper_warp::make_graphql_filter(
                    schema,
                    warp::any().map(move || ctx.clone()),
                ));

        // GET /graphiql -> GraphiQL interface
        let graphiql = warp::path("graphiql")
            .and(warp::get())
            .and(juniper_warp::graphiql_filter("/graphql", None));

        // Create a single filter that handles both routes and box the reply
        let routes = graphql
            .or(graphiql)
            .map(|reply| Box::new(reply) as Box<dyn Reply>)
            .boxed();

        Service {
            config,
            filter: routes,
        }
    }

    /// Starts the service's GraphQL/HTTP server. This function runs
    /// without return.
    ///
    /// # Panics
    ///
    /// The HTTP interface will panic if the ip address and port provided
    /// cannot be bound (like if they are already in use).
    pub fn start(self) {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let hosturl = self
                .config
                .hosturl()
                .ok_or_else(|| {
                    log::error!("Failed to load service URL");
                    "Failed to load service URL"
                })
                .unwrap();

            let addr = hosturl
                .parse::<SocketAddr>()
                .map_err(|err| {
                    log::error!("Failed to parse SocketAddr: {:?}", err);
                    err
                })
                .unwrap();

            info!("Listening on: {}", addr);

            warp::serve(self.filter).run(addr).await;
        });
    }
}
