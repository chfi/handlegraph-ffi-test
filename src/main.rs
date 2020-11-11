use handlegraph::{
    conversion, handle::*, handlegraph::*, hashgraph::*, mutablehandlegraph::*, pathgraph::*,
};

use gfa::{
    gfa::GFA,
    optfields::OptFields,
    parser::{GFAParser, GFAResult},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = GFAParser::new();
    let gfa: GFA<usize, ()> = parser.parse_file(&"lil.gfa")?;
    let hashgraph = conversion::from_gfa::<HashGraph, _>(&gfa);

    for h in hashgraph.all_handles() {
        println!("{:?}", h);
    }

    for e in hashgraph.all_edges() {
        println!("{:?}", e);
    }

    Ok(())
}
