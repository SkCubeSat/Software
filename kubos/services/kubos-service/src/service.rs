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

use async_graphql::{EmptySubscription, ObjectType, Schema, http::GraphiQLSource};
use async_graphql_axum::GraphQL;
use axum::{
    Router,
    response::{self, IntoResponse},
    routing::get,
};
use log::info;
use radsat_system::Config;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tokio::net::TcpListener;

/// Context struct used by a service to provide async-graphql context,
/// subsystem access and persistent storage.
#[derive(Clone)]
pub struct Context<T> {
    /// The subsystem instance this context provides access to
    pub subsystem: T,
    /// Persistent key-value storage for the service
    pub storage: Arc<RwLock<HashMap<String, String>>>,
}

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
pub struct Service<Query, Mutation, S> {
    config: Config,
    schema: Schema<Query, Mutation, EmptySubscription>,
    _phantom: std::marker::PhantomData<S>,
}

impl<Query, Mutation, S> Service<Query, Mutation, S> {
    /// Returns a reference to the GraphQL schema.
    /// Useful for testing purposes.
    pub fn schema(&self) -> &Schema<Query, Mutation, EmptySubscription> {
        &self.schema
    }
}

impl<Query, Mutation, S> Service<Query, Mutation, S>
where
    Query: ObjectType + 'static,
    Mutation: ObjectType + 'static,
    S: Send + Sync + Clone + 'static,
{
    /// Creates a new service instance
    ///
    /// # Arguments
    ///
    /// `config` - The service configuration
    /// `subsystem` - An instance of the subsystem struct. This one instance will be used by all queries.
    /// `query` - The root query struct holding all other GraphQL queries.
    /// `mutation` - The root mutation struct holding all other GraphQL mutations.
    pub fn new(
        config: Config,
        subsystem: S,
        query: Query,
        mutation: Mutation,
    ) -> Self {
        let context = Context {
            subsystem,
            storage: Arc::new(RwLock::new(HashMap::new())),
        };

        // Build the GraphQL schema with the context as data
        let schema = Schema::build(query, mutation, EmptySubscription)
            .data(context)
            .finish();

        Service {
            config,
            schema,
            _phantom: std::marker::PhantomData,
        }
    }

    /// GraphiQL endpoint handler
    async fn graphiql() -> impl IntoResponse {
        response::Html(GraphiQLSource::build().endpoint("/graphql").finish())
    }



    /// Starts the service's GraphQL/HTTP server using a blocking runtime.
    /// This is the main entry point for services and maintains backward compatibility.
    ///
    /// # Panics
    ///
    /// The HTTP interface will panic if the ip address and port provided
    /// cannot be bound (like if they are already in use).
    pub fn start(self) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(self.start_async());
    }

    /// Starts the service's GraphQL/HTTP server asynchronously.
    /// This method can be used in async contexts.
    ///
    /// # Panics
    ///
    /// The HTTP interface will panic if the ip address and port provided
    /// cannot be bound (like if they are already in use).
    pub async fn start_async(self) {
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

        // Create our application with routes
        let app = Router::new()
            .route("/graphql", get(Self::graphiql).post_service(GraphQL::new(self.schema)))
            .route("/graphiql", get(Self::graphiql));

        info!("Listening on: {}", addr);

        // Start the server
        axum::serve(TcpListener::bind(addr).await.unwrap(), app)
            .await
            .unwrap();
    }
}
