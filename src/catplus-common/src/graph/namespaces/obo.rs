use lazy_static::lazy_static;
use sophia::api::ns::Namespace;
use sophia_api::namespace;
namespace! {
    "http://purl.obolibrary.org/obo/",
    CHEBI_25367,
    IAO_0000005,
    IAO_0000009,
    IAO_0000017,
    IAO_0000590,
    PATO_0001019
}
lazy_static! {
    pub static ref ns: Namespace<&'static str> = Namespace::new(PREFIX.as_str()).unwrap();
}
