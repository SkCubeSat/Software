use async_graphql::{EmptySubscription, Schema};

use crate::schema::{MutationRoot, QueryRoot};

#[test]
fn schema_exposes_generated_adcs_fields() {
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .finish();
    let sdl = schema.sdl();

    assert!(sdl.contains("fssCubesenseSunRaw"));
    assert!(sdl.contains("hil"));
    assert!(sdl.contains("controlMode"));
    assert!(sdl.contains("currentUnixTime"));
}

#[test]
fn schema_exposes_service_control_fields() {
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .finish();
    let sdl = schema.sdl();

    assert!(sdl.contains("health"));
    assert!(sdl.contains("setInterfaceUp"));
    assert!(sdl.contains("resetInterface"));
    assert!(sdl.contains("sendCommandRaw"));
}
