use serde::{Serialize, Deserialize};
use crate::dag::Visitor;
use super::{MachNode, Handle};


///
/// MachGraph.
/// This is where nodes are organized.
/// 
#[derive(Debug, Serialize, Deserialize)]
pub struct MachGraph {
    /// Name of this graph.
    pub name: String,

    /// Optionally used index of this graph.
    pub index: u32,

    /// Root index of this graph.
    pub root: Handle,

    /// Nodes in this graph.
    pub nodes: Vec<MachNode>,
}


///
/// Default implementation for MachGraph.
/// 
impl Default for MachGraph {
    fn default() -> Self {
        let root = MachNode::default();
        Self {
            name: String::from("default"),
            index: 0,
            root: Handle::from((root.name.clone(), 0)),
            nodes: vec![root],
        }
    }
}


///
/// Implementation for MachGraph.
/// 
impl MachGraph {
    /// New graph with a name.
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            ..Default::default()
        }
    }


    /**********************************************************
     * Getters
     **********************************************************/

    /// Get root node reference.
    pub fn get_root(&self) -> Option<&MachNode> {
        self.get_node(&self.root)
    }


    /// Get root node mutable reference.
    pub fn get_root_mut(&mut self) -> Option<&mut MachNode> {
        self.get_node_mut(&self.root.clone())
    }


    /// Get node reference.
    pub fn get_node(&self, handle: &Handle) -> Option<&MachNode> {
        let index: u32;
        if handle.has_index() { index = handle.index.unwrap(); }
        else { 
            if let Some(idx) = Handle::index(self, &handle.path) {
                index = idx;
            } else {
                return None;
            }
        }
        let u = index as usize;
        if u < self.nodes.len() {
            return Some(&self.nodes[u]);
        }
        None
    }


    /// Get node mutable reference.
    pub fn get_node_mut(&mut self, handle: &Handle) -> Option<&mut MachNode> {
        let index: u32;
        if handle.has_index() { index = handle.index.unwrap(); }
        else { 
            if let Some(idx) = Handle::index(self, &handle.path) {
                index = idx;
            } else {
                return None;
            }
        }
        let u = index as usize;
        if u < self.nodes.len() {
            return Some(&mut self.nodes[u]);
        }
        None
    }


    /// Get parent.
    pub fn get_parent(&self, handle: &Handle) -> Option<&MachNode> {
        if let Some(node) = self.get_node(handle) {
            if !node.has_parent() { return None; }
            return self.get_node(&Handle::from(node.parent));
        }
        None
    }


    /**********************************************************
     * Components
     **********************************************************/

    /// Push a component to a node.
    pub fn push_component(&mut self, node: &Handle, component: u32) {
        if let Some(node) = self.get_node_mut(node) {
            node.components.push(component);
        }
    }


    /**********************************************************
     * Children
     **********************************************************/

    /// Push a child node of root with a name.
    pub fn push_child(&mut self, name: &str) -> Handle {
        self.push_child_of(name, &self.root.clone())
    }


    /// Push a new child node with a name and a parent.
    pub fn push_child_of(&mut self, name: &str, parent: &Handle) -> Handle {
        if let Some(parent_index) = parent.get_index(self) {
            let mut node = MachNode::from((name.into(), parent_index));
            let index = self.nodes.len() as u32;
            node.index = index;
            if let Some(parent) = self.get_node_mut(parent) {
                parent.children.push(node.index);
            }
            self.nodes.push(node);
            let mut handle = Handle::from(index);
            handle.set_path(self);
            return handle;
        }
        Handle::from("root")
    }


    /// Push a node to this graph. Sets index and returns it. Not used often...
    pub fn push(&mut self, mut node: MachNode) -> u32 {
        let index = self.nodes.len() as u32;
        node.index = index;
        self.nodes.push(node);
        index
    }


    /**********************************************************
     * Visitors
     **********************************************************/

    /// Visit all nodes (not in graph order).
    pub fn visit_all(&self, visitor: &impl Visitor) {
        for node in &self.nodes { node.accept(visitor); }
    }


    /// Visit all nodes mutable (not in graph order).
    pub fn visit_all_mut(&mut self, visitor: &mut impl Visitor) {
        for node in &mut self.nodes { node.accept_mut(visitor); }
    }


    /// Pre-visit.
    pub fn pre_visit(&self, visitor: &impl Visitor) {
        let mut handle = self.root.clone();
        self.pre_visit_internal(visitor, &mut handle);
    }
    fn pre_visit_internal(&self, visitor: &impl Visitor, handle: &mut Handle) {
        if let Some(node) = self.get_node(&handle) {
            node.accept(visitor);
            for child in &node.children {
                let mut handle = Handle::from(*child);
                self.pre_visit_internal(visitor, &mut handle);
            }
        }
    }


    /// Pre-visit mutable.
    pub fn pre_visit_mut(&mut self, visitor: &mut impl Visitor) {
        let mut handle = self.root.clone();
        self.pre_visit_internal_mut(visitor, &mut handle);
    }
    fn pre_visit_internal_mut(&mut self, visitor: &mut impl Visitor, handle: &mut Handle) {
        let mut children: Vec<u32> = Vec::new();
        if let Some(node) = self.get_node_mut(&handle) {
            node.accept_mut(visitor);
            children = node.children.clone();
        }
        for child in &children {
            let mut handle = Handle::from(*child);
            self.pre_visit_internal_mut(visitor, &mut handle);
        }
    }


    /// Post-visit.
    pub fn post_visit(&self, visitor: &impl Visitor) {
        let mut handle = self.root.clone();
        self.post_visit_internal(visitor, &mut handle);
    }
    fn post_visit_internal(&self, visitor: &impl Visitor, handle: &mut Handle) {
        if let Some(node) = self.get_node(&handle) {
            for child in &node.children {
                let mut handle = Handle::from(*child);
                self.post_visit_internal(visitor, &mut handle);
            }
            node.accept(visitor);
        }
    }


    /// Post-visit mutable.
    pub fn post_visit_mut(&mut self, visitor: &mut impl Visitor) {
        let mut handle = self.root.clone();
        self.post_visit_internal_mut(visitor, &mut handle);
    }
    fn post_visit_internal_mut(&mut self, visitor: &mut impl Visitor, handle: &mut Handle) {
        let mut children: Vec<u32> = Vec::new();
        if let Some(node) = self.get_node_mut(&handle) {
            children = node.children.clone();
        }
        for child in &children {
            let mut handle = Handle::from(*child);
            self.post_visit_internal_mut(visitor, &mut handle);
        }
        if let Some(node) = self.get_node_mut(&handle) {
            node.accept_mut(visitor);
        }
    }
}