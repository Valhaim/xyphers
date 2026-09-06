use std::collections::BTreeMap;

pub type NodeId = u32;
pub type WeightedAdj = BTreeMap<NodeId, Vec<(NodeId, f64)>>;

#[derive(Debug, Clone)]
pub struct EntropyResult {
    pub per_node: BTreeMap<NodeId, f64>,
    pub global: f64,
}
