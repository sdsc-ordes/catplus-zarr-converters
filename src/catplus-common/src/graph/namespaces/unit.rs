use lazy_static::lazy_static;
use sophia::api::ns::Namespace;
use sophia_api::namespace;

// Rust cannot handle dashes in variable names, therefore this
// namespace uses rust adapted term names and then maps them on output
// to the real ontology units
namespace! {
    "http://qudt.org/vocab/unit/",
    Bar,
    DegC,
    GMPerMilliL,
    GMPerMol,
    MilliGM,
    MolPerL,
    RevPerMin,
    MilliM3,
    SEC,
    MIN,
    PERCENT, 
    NanoM,
    UNITLESS,
    CountsPerSec

}

lazy_static! {
    pub static ref ns: Namespace<&'static str> =
        Namespace::new("http://qudt.org/vocab/unit/").unwrap();
}
