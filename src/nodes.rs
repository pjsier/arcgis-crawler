use serde::{Deserialize, Serialize};
use std::cmp::{min, Ord, Ordering, PartialEq, PartialOrd};

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
        // Compare last available index so /server1/test comes before /server2
        let compare_index = min(self.0.len(), other.0.len());

        if compare_index == 0 {
            return match (self.0.len(), other.0.len()) {
                (0, 0) => Some(Ordering::Equal),
                (0, _) => Some(Ordering::Less),
                (_, 0) => Some(Ordering::Greater),
                (_, _) => None,
            };
        }

        if self.0.len() == other.0.len() {
            return Some(self.0.last().cmp(&other.0.last()));
        }

        Some(self.0[compare_index].cmp(&other.0[compare_index]))
    }
}
