use lazy_static::lazy_static;
use sophia::api::ns::Namespace;
use sophia_api::namespace;
use sophia_api::ns::NsTerm;

// Rust cannot handle dashes in variable names, therefore this
// namespace uses rust adapted term names and then maps them on output
// to the real ontology units
namespace! {
    "https://qudt.org/vocab/unit/",
    Bar,
    DegC,
    MilliGM,
    GMPerMilliL,
    MolPerL,
    RevPerMin
}

lazy_static! {
    pub static ref ns: Namespace<&'static str> = Namespace::new("https://qudt.org/vocab/unit/").unwrap();
}

pub enum Unit {
    Bar,
    DegC,
    MilliGM,
    GMPerMilliL,
    MolPerL,
    RevPerMin
}

impl Unit {
    pub fn display_name(&self) -> &'static str {
        match self {
            Unit::Bar => "Bar",
            Unit::DegC => "DEG-C",
            Unit::MilliGM => "MilliGM",
            Unit::GMPerMilliL => "GM-PER-MilliL",
            Unit::MolPerL => "MOL-PER-L",
            Unit::RevPerMin => "REV-PER-MIN",
        }
    }
}

// Define the trait
pub trait ToNsTerm {
    fn to_ns_term(&self) -> NsTerm<'_>;
}

// Implement the trait for Unit
impl ToNsTerm for Unit {
    fn to_ns_term(&self) -> NsTerm<'_> {
        // Borrow the term to match the return type
        ns.get(self.display_name()).expect("Term not found in namespace")
    }
}
