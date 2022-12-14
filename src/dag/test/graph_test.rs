#[cfg(test)]
mod mach_graph_test {
    use crate::dag::*;

    #[test]
    fn construction() {
        let graph = MachGraph::default();
        assert_eq!(graph.index, 0);
        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.root.index.unwrap(), 0);
    }

    #[test]
    fn push_child() {
        let mut graph = MachGraph::default();
        let child = graph.push_child("child");

        assert_eq!(child.index.unwrap(), 1);
        assert_eq!(graph.nodes.len(), 2);
        
        if let Some(child) = graph.get_node(&child) {
            assert_eq!(child.index, 1);
            assert_eq!(child.parent, 0);
            assert_eq!(child.name, String::from("child"));
            assert_eq!(child.children.len(), 0);
            assert_eq!(child.components.len(), 0);
        }

        if let Some(root) = graph.get_root() {
            assert_eq!(root.index, 0);
            assert_eq!(root.parent, 0);
            assert_eq!(root.children.len(), 1);
            assert_eq!(root.children[0], 1);
            assert_eq!(root.name, String::from("root"));
            assert_eq!(root.components.len(), 0);
        }
    }

    #[test]
    fn push_child_of() {
        let mut graph = MachGraph::default();

        let left = graph.push_child("left");
        let left_child = graph.push_child_of("left_child", &left);
        
        let right = graph.push_child("right");
        let right_child = graph.push_child_of("right_child", &right);
        
        assert_eq!(graph.nodes.len(), 5);
        assert_eq!(graph.root.index.unwrap(), 0);

        if let Some(root) = graph.get_root() {
            assert_eq!(root.index, 0);
            assert_eq!(root.parent, 0);
            assert_eq!(root.children.len(), 2);
            assert_eq!(root.children[0], 1);
            assert_eq!(root.children[1], 3);
        }

        if let Some(left) = graph.get_node(&left) {
            assert_eq!(left.index, 1);
            assert_eq!(left.parent, 0);
            assert_eq!(left.children.len(), 1);
            assert_eq!(left.children[0], 2);
        }

        if let Some(left_child) = graph.get_node(&left_child) {
            assert_eq!(left_child.index, 2);
            assert_eq!(left_child.parent, 1);
            assert_eq!(left_child.children.len(), 0);
        }

        if let Some(right) = graph.get_node(&right) {
            assert_eq!(right.index, 3);
            assert_eq!(right.parent, 0);
            assert_eq!(right.children.len(), 1);
            assert_eq!(right.children[0], 4);
        }

        if let Some(right_child) = graph.get_node(&right_child) {
            assert_eq!(right_child.index, 4);
            assert_eq!(right_child.parent, 3);
            assert_eq!(right_child.children.len(), 0);
        }
    }

    #[test]
    fn syntax() {
        graph!(graph, {
            node!(graph, first, "first", {
                node!(graph, _second, "second", first, {});
            });

            node!(graph, second, "another");
            {
                node!(graph, _second, "second", second);
            }
        });

        assert_eq!(graph.nodes.len(), 5);
    }

    #[test]
    fn push_child_of_syntax() {
        graph!(graph, "only_one", {
            
            node!(graph, left, "left");
            node!(graph, left_child, "left_child", left);
            node!(graph, right, "right");
            node!(graph, right_child, "right_child", right);

            assert_eq!(graph.nodes.len(), 5);
            assert_eq!(graph.root.index.unwrap(), 0);

            if let Some(root) = graph.get_root() {
                assert_eq!(root.index, 0);
                assert_eq!(root.parent, 0);
                assert_eq!(root.children.len(), 2);
                assert_eq!(root.children[0], 1);
                assert_eq!(root.children[1], 3);
            }

            if let Some(left) = graph.get_node(&left) {
                assert_eq!(left.index, 1);
                assert_eq!(left.parent, 0);
                assert_eq!(left.children.len(), 1);
                assert_eq!(left.children[0], 2);
            }

            if let Some(left_child) = graph.get_node(&left_child) {
                assert_eq!(left_child.index, 2);
                assert_eq!(left_child.parent, 1);
                assert_eq!(left_child.children.len(), 0);
            }

            if let Some(right) = graph.get_node(&right) {
                assert_eq!(right.index, 3);
                assert_eq!(right.parent, 0);
                assert_eq!(right.children.len(), 1);
                assert_eq!(right.children[0], 4);
            }

            if let Some(right_child) = graph.get_node(&right_child) {
                assert_eq!(right_child.index, 4);
                assert_eq!(right_child.parent, 3);
                assert_eq!(right_child.children.len(), 0);
            }
        });
    }
}