use oxrdf::{Graph, NamedNodeRef, SubjectRef, Triple};

// ------------------ INSERT TRIPLES 

pub fn insert_triple<'graph>(
    graph: &'graph mut Graph, 
    subject: SubjectRef, 
    predicate: NamedNodeRef, 
    object: NamedNodeRef
) -> Result<&'graph mut Graph , Box<dyn std::error::Error>> {
    let triple = Triple::new(subject, predicate, object);
    graph.insert(&triple);
    Ok(graph)
}

// with Sofia - None functional

// use sophia::term::GenericTerm::iri;
// use sophia::term::GenericLiteral::Literal;
// use sophia::inmem::graph::LightGraph;
// use sophia::inmem::graph::GenericLightGraph::insert;


// pub fn insert_triple(graph: &LightGraph, subject: &iri, predicate:&iri, object: &Literal) -> Result<(), Box<dyn std::error::Error>> {
    
//     // Add a triple to the graph
//     graph.insert(&subject, &predicate, &object)?;

//     Ok(())
// }


