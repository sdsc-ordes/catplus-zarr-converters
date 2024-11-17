// src/graph/namespaces.rs
use sophia_api::namespace;

pub mod schema {
    // Defining terms under the "https://schema.org/" namespace
    namespace! {
        "https://schema.org/",
        name,       // This term should be accessible as `schema::name`
    }
}
