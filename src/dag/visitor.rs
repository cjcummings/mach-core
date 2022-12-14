use crate::dag::MachNode;


///
/// Visitor trait.
/// 
pub trait Visitor {
    /// Visit a node.
    fn visit(&self, _node: &MachNode) { /* Abstract */ }

    /// Visit mutable node.
    fn visit_mut(&mut self, node: &mut MachNode) { self.visit(node); }
}