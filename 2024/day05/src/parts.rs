use std::collections::{HashMap, HashSet};
enum KahnStates {
    Initial,
    FoundSome,
    FoundNone,
}
fn create_ordering(edge_list: &[(i32, i32)], nodes_used: &[i32]) -> Option<HashMap<i32, i32>> {
    let mut in_values = HashMap::<i32, i32>::new();
    let mut edges = HashMap::<i32, Vec<i32>>::new();
    let nodes_set: HashSet<i32> = nodes_used.iter().cloned().collect();
    for edge in edge_list {
        if !nodes_set.contains(&edge.0) || !nodes_set.contains(&edge.1) {
            continue;
        }
        match in_values.get(&edge.0) {
            None => in_values.insert(edge.0, 0),
            Some(x) => in_values.insert(edge.0, *x),
        };
        match in_values.get(&edge.1) {
            None => in_values.insert(edge.1, 1),
            Some(x) => in_values.insert(edge.1, x + 1),
        };
        match edges.get_mut(&edge.0) {
            None => {
                edges.insert(edge.0, Vec::from([edge.1]));
            }
            Some(x) => {
                x.push(edge.1);
            }
        };
    }
    let mut ans = HashMap::<i32, i32>::new();
    let mut state = KahnStates::Initial;
    let mut idx = 0;
    while !matches!(state, KahnStates::FoundNone) {
        state = KahnStates::FoundNone;
        let mut to_remove = Vec::new();
        for (node, num) in in_values.iter().by_ref() {
            if *num == 0 {
                state = KahnStates::FoundSome;
                ans.insert(*node, idx);
                to_remove.push(node.clone());
            }
        }
        for node in to_remove {
            in_values.insert(node, -1);
            if let Some(out_nodes) = edges.get(&node) {
                for out_node in out_nodes {
                    if let Some(x) = in_values.get(&out_node) {
                        in_values.insert(*out_node, x - 1);
                    }
                }
            }
        }
        idx += 1;
    }
    if in_values.values().any(|x| *x > 0) {
        return None;
    }
    return Some(ans);
}
pub fn part1_per_order(edges: &[(i32, i32)], order: &[i32]) -> i32 {
    if let Some(ordering) = create_ordering(edges, order.as_ref()) {
        let is_ordered =
            order.is_sorted_by(|a, b| ordering.get(a).unwrap() <= ordering.get(b).unwrap());
        if is_ordered {
            order[order.len() / 2]
        } else {
            0
        }
    } else {
        0
    }
}
pub fn part2_per_order(edges: &[(i32, i32)], order: &[i32]) -> i32 {
    let ordering = create_ordering(edges, order.as_ref())
        .expect("Part 2 assumes creating the ordering will always succeed.");
    let is_ordered =
        order.is_sorted_by(|a, b| ordering.get(a).unwrap() <= ordering.get(b).unwrap());
    if is_ordered {
        0
    } else {
        let mut sorted_bad_order: Box<[i32]> = order.iter().cloned().collect();
        sorted_bad_order.sort_by(|a, b| ordering.get(a).unwrap().cmp(ordering.get(b).unwrap()));
        sorted_bad_order[sorted_bad_order.len() / 2]
    }
}
