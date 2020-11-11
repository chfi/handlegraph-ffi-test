use handlegraph::{
    conversion, handle::*, handlegraph::*, hashgraph::*, mutablehandlegraph::*, pathgraph::*,
};

use gfa::{
    gfa::GFA,
    optfields::OptFields,
    parser::{GFAParser, GFAResult},
};

pub struct CGraph {
    graph: HashGraph,
}

pub struct HandlesIter<'a> {
    iter: NodeIdRefHandles<'a, std::collections::hash_map::Keys<'a, NodeId, Node>>,
}

impl CGraph {
    pub fn parse_gfa(file: &str) -> Self {
        let parser = GFAParser::new();
        let gfa: GFA<usize, ()> = parser.parse_file(file).unwrap();
        let hashgraph = conversion::from_gfa::<HashGraph, _>(&gfa);
        CGraph { graph: hashgraph }
    }

    pub fn handles(&self) -> HandlesIter<'_> {
        let iter = self.graph.all_handles();
        HandlesIter { iter }
    }
}

#[no_mangle]
pub unsafe extern "C" fn load_hashgraph() -> *const CGraph {
    Box::into_raw(Box::new(CGraph::parse_gfa(&"lil.gfa")))
}

#[no_mangle]
pub extern "C" fn add(first: i32, second: i32) -> i32 {
    first + second
}

#[no_mangle]
pub unsafe extern "C" fn graph_handles<'a>(graph: *const CGraph) -> *mut HandlesIter<'a> {
    let g = &(*graph);
    let iter = g.handles();
    Box::into_raw(Box::new(iter))
}

#[no_mangle]
pub unsafe extern "C" fn handles_next<'a>(handles: *mut HandlesIter<'a>) -> u64 {
    let handles = &mut (*handles);
    let next = handles.iter.next();
    match next {
        None => 0,
        Some(h) => u64::from(h.id()),
    }
}
