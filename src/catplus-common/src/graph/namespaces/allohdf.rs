use lazy_static::lazy_static;
use sophia::api::ns::Namespace;
use sophia_api::namespace;
namespace! {
    "http://purl.allotrope.org/ontologies/hdf5/1.8#",
    HardLink
}
lazy_static! {
    pub static ref ns: Namespace<&'static str> = Namespace::new(PREFIX.as_str()).unwrap();
}
