
#[allow(unused)]
#[macro_export]
macro_rules! graph {
    ($var:ident, $block:block) => {
        let mut $var = MachGraph::default();
        $block;
    };
    ($var:ident, $name:expr, $block:block) => {
        let mut $var = MachGraph::new($name);
        $block;
    };
}

#[allow(unused)]
#[macro_export]
macro_rules! node {
    ($graph:ident, $var:ident, $name:expr) => {
        let $var = $graph.push_child($name);
    };
    ($graph:ident, $var:ident, $name:expr, $block:block) => {
        let $var = $graph.push_child($name);
        $block;
    };
    ($graph:ident, $var:ident, $name:expr, $parent:expr) => {
        let $var = $graph.push_child_of($name, &$parent);
    };
    ($graph:ident, $var:ident, $name:expr, $parent:expr, $block:block) => {
        let $var = $graph.push_child_of($name, &$parent);
        $block;
    };
}


/*

mach!(graph, {
    node!(graph, left, "left", {
        node!(graph, child, "another", left, {
            node!(graph, child_child, "ch", left_c);
        });
    });
});

*/