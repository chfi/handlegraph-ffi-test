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

#[repr(C)]
pub struct HandlesIter<'a> {
    finished: bool,
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
        HandlesIter {
            finished: false,
            iter,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn load_hashgraph(file_name: *const u8, file_name_len: usize) -> *mut CGraph {
    let file_name_slice = std::slice::from_raw_parts(file_name, file_name_len);
    let file_str = std::str::from_utf8(file_name_slice).unwrap();
    Box::into_raw(Box::new(CGraph::parse_gfa(file_str)))
}

#[no_mangle]
pub unsafe extern "C" fn free_hashgraph(graph: *mut CGraph) {
    let _ = Box::from_raw(graph);
}

#[no_mangle]
pub unsafe extern "C" fn graph_handles<'a>(graph: *const CGraph) -> *mut HandlesIter<'a> {
    let g = &(*graph);
    let iter = g.handles();
    Box::into_raw(Box::new(iter))
}

#[no_mangle]
pub unsafe extern "C" fn free_handles_iter<'a>(handles: *mut HandlesIter<'a>) {
    let _ = Box::from_raw(handles);
}

#[no_mangle]
pub unsafe extern "C" fn handles_next<'a>(handles: *mut HandlesIter<'a>) -> u64 {
    let handles = &mut (*handles);
    let next = handles.iter.next();
    match next {
        None => {
            handles.finished = true;
            0
        }
        Some(h) => u64::from(h.id()),
    }
}
