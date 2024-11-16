use crate::parser::person::Person;

pub fn parse_json(json_data: &str) -> serde_json::Result<Person> {
    // Parse the JSON string into a `Person` object
    let person: Person = serde_json::from_str(json_data)?;

    // Log the parsed person object for debugging
    println!("{:?}", person);

    // Return the `Person` object
    Ok(person)
}
