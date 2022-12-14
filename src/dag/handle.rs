use super::{MachGraph, MachNode};


///
/// Handle.
/// Used for referencing nodes within a MachGraph.
///
#[derive(Debug, Clone)] 
pub struct Handle {
    /// Name of the node (is a dot-separated path in a search).
    pub path: String,

    /// Index of the node in the graph.
    pub index: Option<u32>
}


///
/// Handle implementation.
/// 
impl Handle {
    ///
    /// Has index?
    /// 
    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }


    ///
    /// Get index.
    /// 
    pub fn get_index(&self, graph: &MachGraph) -> Option<u32> {
        if self.has_index() { return self.index.clone(); }
        Self::index(graph, &self.path)
    }


    ///
    /// Set index.
    /// 
    pub fn set_index(&mut self, graph: &MachGraph) {
        self.index = Self::index(graph, &self.path);
    }


    ///
    /// Set path.
    /// 
    pub fn set_path(&mut self, graph: &MachGraph) {
        if self.has_index() {
            if let Some(path) = Self::path(graph, self.index.unwrap()) {
                self.path = path;
            }
        }  
    }


    ///
    /// Get a path for an index.
    /// 
    pub fn path(graph: &MachGraph, index: u32) -> Option<String> {
        let mut current_index = index as usize;
        if current_index < graph.nodes.len() {
            let mut current = &graph.nodes[current_index];
            let mut result = current.name.clone();
            while current.has_parent() {
                current_index = current.parent as usize;
                if current_index < graph.nodes.len() {
                    current = &graph.nodes[current_index];
                    result.push('.');
                    result.push_str(current.name.as_str());
                } else {
                    return None; // Issues with your graph dude...
                }
            }
            return Some(result);
        }
        None
    }


    ///
    /// Get an index for a path.
    /// Does the best it can with the path provided.
    /// 
    pub fn index(graph: &MachGraph, path: &String) -> Option<u32> {
        let mut current: &MachNode = graph.get_root().expect("No root found on graph");
        let mut set = false;
        for name in path.split('.') {
            // Get starting node - first node found with 'name'. Assumes first name in path is unique or starts high enough in the tree.
            if !set {
                for node in &graph.nodes {
                    if node.name == name {
                        current = node;
                        set = true;
                        break;
                    }
                }
                if !set { return None; }
            }
            // If name != current.name, then current needs to be updated. Look in currents children.
            if name != current.name {
                let mut found = false;
                for child_index in &current.children {
                    let idx = *child_index as usize;
                    if idx < graph.nodes.len() {
                        let child = &graph.nodes[idx];
                        if child.name == name {
                            current = child;
                            found = true;
                            break;
                        }
                    }
                }
                if !found { return None; } // Don't continue with the path.... its not what the user expected.
            }
        }
        Some(current.index)
    }
}


///
/// From a path.
/// 
impl From<String> for Handle {
    fn from(path: String) -> Self {
        Self {
            path: path,
            index: None
        }
    }
}


///
/// From a path &str.
/// 
impl From<&str> for Handle {
    fn from(path: &str) -> Self {
        Self {
            path: String::from(path),
            index: None
        }
    }
}


///
/// From an index.
/// 
impl From<u32> for Handle {
    fn from(index: u32) -> Self {
        Self {
            path: String::from("undefined"),
            index: Some(index)
        }
    }
}


///
/// From a path and an index.
/// 
impl From<(String, u32)> for Handle {
    fn from((path, index): (String, u32)) -> Self {
        Self {
            path: path,
            index: Some(index)
        }
    }
}
