use lazy_static::lazy_static;
use sophia::api::ns::Namespace;
use sophia_api::namespace;
namespace! {
    "http://example.org/cat#",
    containerID,
    containerBarcode,
    hasContainerPositionAndQuantity,
    ContainerPositionAndQuantity,
    has_chemical,
    chemicalName,
    casNumber,
    Sample,
    hasSample,
    role,
    expectedDatum,
    internalBarCode,
    vialShape,
    AddAction,
    setTemperatureAction,
    hasBatch,
    Batch,
    localEquipmentName,
    temperatureShakerShape,
    temperatureTumbleStirrerShape,
    speedInRPM,
    dispenseType
}
lazy_static! {
    pub static ref ns: Namespace<&'static str> = Namespace::new(PREFIX.as_str()).unwrap();
}
