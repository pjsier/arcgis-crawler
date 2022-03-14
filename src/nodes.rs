use serde::{Deserialize, Serialize};
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ArcgisResponse {
    #[serde(default)]
    pub folders: Vec<String>,
    #[serde(default)]
    pub services: Vec<ServerObject>,
    #[serde(default)]
    pub layers: Vec<LayerObject>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServerObject {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LayerObject {
    pub id: usize,
    pub name: String,
}

// full path to node, sorted by length and then alpha (should use name instead of ID)
#[derive(Debug, Clone)]
pub struct ServerNode(pub Vec<String>);

impl PartialEq for ServerNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for ServerNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.join("-").cmp(&other.0.join("-")))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_partial_cmp() {
        let node_1 = ServerNode(vec![
            "BuildingLabels".to_string(),
            "MapServer".to_string(),
            "Parcels".to_string(),
        ]);
        let node_2 = ServerNode(vec![
            "ExternalApps".to_string(),
            "Zoning".to_string(),
            "MapServer".to_string(),
            "Zoning".to_string(),
        ]);
        assert!(matches!(node_1.partial_cmp(&node_2), Some(Ordering::Less)));
    }
}
