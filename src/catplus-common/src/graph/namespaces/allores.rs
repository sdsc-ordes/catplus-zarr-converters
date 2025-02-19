use lazy_static::lazy_static;
use sophia::api::ns::Namespace;
use sophia_api::namespace;
namespace! {
    "http://purl.allotrope.org/ontologies/result#",
    AFR_0001606,
    AFR_0001723,
    AFR_0001952,
    AFR_0002036,
    AFR_0002240,
    AFR_0002294,
    AFR_0002295,
    AFR_0002296,
    AFR_0002423,
    AFR_0002464,
    AFR_0002764,
    AFRE_0000001,
    AFX_0000622
}
lazy_static! {
    pub static ref ns: Namespace<&'static str> = Namespace::new(PREFIX.as_str()).unwrap();
}
