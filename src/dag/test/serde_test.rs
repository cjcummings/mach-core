#[cfg(test)]
mod serde_test {
    use crate::dag::*;

    #[test]
    fn json() {
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

        let json = serde_json::to_string(&graph).expect("Error parsing graph to JSON.");
        assert_eq!(json, String::from("{\"name\":\"default\",\"index\":0,\"root\":{\"path\":\"root\",\"index\":0},\"nodes\":[{\"name\":\"root\",\"parent\":0,\"index\":0,\"children\":[1,12,13],\"components\":[]},{\"name\":\"body\",\"parent\":0,\"index\":1,\"children\":[2,5,9],\"components\":[]},{\"name\":\"base\",\"parent\":1,\"index\":2,\"children\":[3,4],\"components\":[]},{\"name\":\"left\",\"parent\":2,\"index\":3,\"children\":[],\"components\":[]},{\"name\":\"right\",\"parent\":2,\"index\":4,\"children\":[],\"components\":[]},{\"name\":\"mid\",\"parent\":1,\"index\":5,\"children\":[6,7,8],\"components\":[]},{\"name\":\"bottom_button\",\"parent\":5,\"index\":6,\"children\":[],\"components\":[]},{\"name\":\"middle_button\",\"parent\":5,\"index\":7,\"children\":[],\"components\":[]},{\"name\":\"top_button\",\"parent\":5,\"index\":8,\"children\":[],\"components\":[]},{\"name\":\"top\",\"parent\":1,\"index\":9,\"children\":[10,11],\"components\":[]},{\"name\":\"left\",\"parent\":9,\"index\":10,\"children\":[],\"components\":[]},{\"name\":\"right\",\"parent\":9,\"index\":11,\"children\":[],\"components\":[]},{\"name\":\"hat\",\"parent\":0,\"index\":12,\"children\":[],\"components\":[]},{\"name\":\"arms\",\"parent\":0,\"index\":13,\"children\":[14,15],\"components\":[]},{\"name\":\"left\",\"parent\":13,\"index\":14,\"children\":[],\"components\":[]},{\"name\":\"right\",\"parent\":13,\"index\":15,\"children\":[],\"components\":[]}]}"));
    }
}