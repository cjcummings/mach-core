use serde::{Deserialize, Serialize};
use crate::dag::Visitor;


///
/// MachNode.
/// This is where data is referenced/owned as relational to other data.
/// Nodes are meant to be as small as possible. They are used for relationships, not actual data.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct MachNode {
    /// Name of this node - does not have to be unique to the graph.
    pub name: String,

    /// Parent index within graph. Same as 'index' means no parent.
    pub parent: u32,

    /// Index of this node within graph.
    pub index: u32,

    /// Children of this node.
    pub children: Vec<u32>,

    /// Index of components in data store (not owned by graph).
    pub components: Vec<u32>,
}


///
/// Default implementation.
/// 
impl Default for MachNode {
    fn default() -> Self {
        Self {
            name: String::from("root"),
            parent: 0,
            index: 0,
            children: Vec::new(),
            components: Vec::new()
        }
    }
}


///
/// Implementation for MachNode.
/// 
impl MachNode {
    /// New mach node with name.
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            ..Default::default()
        }
    }


    /// Has parent?
    pub fn has_parent(&self) -> bool {
        self.index != self.parent
    }


    /// Has children?
    pub fn has_children(&self) -> bool {
        self.children.len() > 0
    }


    /// Has components?
    pub fn has_components(&self) -> bool {
        self.components.len() > 0
    }


    /**********************************************************
     * Visitors
     **********************************************************/

    /// Accept a visitor.
    pub fn accept(&self, visitor: &impl Visitor) {
        visitor.visit(self);
    }


    /// Accept a visitor mutable.
    pub fn accept_mut(&mut self, visitor: &mut impl Visitor) {
        visitor.visit_mut(self);
    }
}


///
/// From a parent.
/// 
impl From<(String, u32)> for MachNode {
    fn from((name, parent): (String, u32)) -> Self {
        Self {
            name: name,
            parent: parent,
            ..Default::default()
        }
    }
}
