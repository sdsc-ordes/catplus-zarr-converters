use lazy_static::lazy_static;
use sophia::api::ns::Namespace;
use sophia_api::namespace;
namespace! {
    "http://purl.allotrope.org/ontology/qudt-ext/unit#",
    MilliAbsorbanceUnitTimesSecond,
    MilliAbsorbanceUnit
}
lazy_static! {
    pub static ref ns: Namespace<&'static str> = Namespace::new(PREFIX.as_str()).unwrap();
}
