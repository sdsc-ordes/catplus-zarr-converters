use lazy_static::lazy_static;
use sophia::api::ns::Namespace;
use sophia_api::namespace;

// Rust cannot handle underscores in variable names, therefore this
// namespace needs display names the contain the real literals with
// underscores.
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