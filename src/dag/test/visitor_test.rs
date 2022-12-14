#[cfg(test)]
mod visitor {
    use crate::dag::*;

    #[derive(Default)]
    struct TestLogVisitor {
    }
    impl Visitor for TestLogVisitor {
        fn visit(&self, node: &MachNode) {
            println!("{}", node.name);
        }
    }
    
    #[derive(Default)]
    struct TestVisitor {
        pub path: Vec<String>,
    }
    impl Visitor for TestVisitor {
        fn visit_mut(&mut self, node: &mut MachNode) {
            self.path.push(node.name.clone());
        }
    }

    #[test]
    fn visit_all() {
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

        //let log_visitor = TestLogVisitor::default();
        //println!("------ ALL ------");
        //graph.visit_all(&log_visitor);

        let mut visitor = TestVisitor::default();
        graph.visit_all_mut(&mut visitor);
        assert_eq!(visitor.path.len(), 16);
    }

    #[test]
    fn pre_visit() {
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

        //let log_visitor = TestLogVisitor::default();
        //println!("------ PRE ------");
        //graph.pre_visit(&log_visitor);

        let mut visitor = TestVisitor::default();
        graph.pre_visit_mut(&mut visitor);
        assert_eq!(visitor.path.len(), 16);
    }

    #[test]
    fn post_visit() {
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

        //let log_visitor = TestLogVisitor::default();
        //println!("------ POST ------");
        //graph.post_visit(&log_visitor);

        let mut visitor = TestVisitor::default();
        graph.post_visit_mut(&mut visitor);
        assert_eq!(visitor.path.len(), 16);
    }
}