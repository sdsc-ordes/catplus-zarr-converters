use lazy_static::lazy_static;
use sophia::api::ns::Namespace;
use sophia_api::namespace;
namespace! {
    "http://qudt.org/schema/qudt/",
    quantity,
    unit,
    value
}
lazy_static! {
    pub static ref ns: Namespace<&'static str> = Namespace::new(PREFIX.as_str()).unwrap();
    pub static ref ns_vocab: Namespace<&'static str> = Namespace::new("http://qudt.org/vocab/unit/").unwrap();
}
