#[cfg(test)]
mod handle {
    use crate::dag::*;

    #[test]
    fn construction() {
        let mut handle = Handle::from(1);
        assert_eq!(handle.index.unwrap(), 1);
        assert_eq!(handle.path, String::from("undefined"));

        handle = Handle::from("name");
        assert_eq!(handle.path, String::from("name"));
        assert!(handle.index.is_none());

        handle = Handle::from(("name", 1));
        assert_eq!(handle.path, String::from("name"));
        assert_eq!(handle.index.unwrap(), 1);
    }

    #[test]
    fn from_string() {
        graph!(graph, {
            node!(graph, body, "body", {
                node!(graph, base, "base", body, {
                    node!(graph, _left_foot, "left", base);
                    node!(graph, _right_foot, "right", base);
                });
                node!(graph, mid, "mid", body, {
                    node!(graph, _bottom, "bottom_button", mid);
                    node!(graph, _middle, "middle_button", mid);
                    node!(graph, _top, "top_button", mid);
                });
                node!(graph, top, "top", body, {
                    node!(graph, _left_eye, "left", top);
                    node!(graph, _right_eye, "right", top);
                });
            });
            node!(graph, _hat, "hat");
            node!(graph, arms, "arms", {
                node!(graph, _left_arm, "left", arms);
                node!(graph, _right_arm, "right", arms);
            });
        });

        let mut handle = Handle::from("root");
        handle.set_index(&graph);
        assert_eq!(handle.index.unwrap(), 0);
        assert_eq!(handle.path, String::from("root"));
        let mut node = graph.get_node(&handle).expect("Not found");
        assert_eq!(node.index, 0);
        assert_eq!(node.name, String::from("root"));

        handle = Handle::from("bottom_button");
        node = graph.get_node(&handle).expect("not found 1");
        assert_eq!(node.index, 6);
        assert_eq!(node.name, String::from("bottom_button"));
        assert!(handle.index.is_none());
        handle.set_index(&graph);
        assert_eq!(handle.index.unwrap(), 6);

        handle = Handle::from("root.body.top.right");
        node = graph.get_node(&handle).expect("not found 2");
        assert_eq!(node.index, 11);
        assert_eq!(node.name, String::from("right"));
        handle.set_index(&graph);
        assert_eq!(handle.index.unwrap(), 11);

        handle = Handle::from("arms.right");
        node = graph.get_node(&handle).expect("not found 3");
        assert_eq!(node.index, 15);
        assert_eq!(node.name, String::from("right"));
        handle.set_index(&graph);
        assert_eq!(handle.index.unwrap(), 15);

        node = graph.get_node(&handle).expect("not found 3");
        assert_eq!(node.index, 15);
        assert_eq!(node.name, String::from("right"));
        handle.set_path(&graph);
        assert_eq!(handle.path, String::from("root.arms.right"));
    }
}