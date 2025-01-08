use lazy_static::lazy_static;
use sophia::api::ns::Namespace;
use sophia_api::namespace;
namespace! {
    "https://qudt.org/vocab/unit/",
    DEG_C,
    MilliGM,
    GM_PER_MilliL,
    MOL_PER_L,
    REV_PER_MIN
}
lazy_static! {
    pub static ref ns: Namespace<&'static str> = Namespace::new(PREFIX.as_str()).unwrap();
}
