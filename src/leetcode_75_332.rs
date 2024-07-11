use std::collections::{HashMap, HashSet};

use crate::Solution;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        fn dfs<'a>(
            node: &'a str,
            adj_list: &HashMap<&'a str, Vec<&'a str>>,
            edges_visited: &mut HashSet<(&'a str, &'a str)>,
            output: &mut Vec<&'a str>,
        ) {
            if let Some(neighbours) = adj_list.get(&node) {
                //  find the lexicographically smaller neighbour which has not been visited
                for neighbour in neighbours {
                    if edges_visited.contains(&(node, neighbour)) {
                        continue;
                    }
                    output.push(neighbour);
                    edges_visited.insert((node, neighbour));
                    dfs(neighbour, adj_list, edges_visited, output);
                }
            }
        }

        //  build the adjacency list
        let mut hash_map: HashMap<&str, Vec<&str>> = HashMap::new();
        for ticket in &tickets {
            let from = &ticket[0];
            let to = &ticket[1];
            hash_map
                .entry(from)
                .and_modify(|e| match e.binary_search(&to.as_str()) {
                    Ok(_) => panic!("found to location already inserted for {}", from),
                    Err(insert_at) => {
                        e.insert(insert_at, to);
                    }
                })
                .or_insert(vec![to]);
        }
        let mut output: Vec<&str> = Vec::new();
        output.push("JFK");
        let mut edges_visited: HashSet<(&str, &str)> = HashSet::new();
        dfs("JFK", &hash_map, &mut edges_visited, &mut output);
        output.iter().map(|&x| x.to_string()).collect()
    }
}

/*
tickets = [["JFK","SFO"],["JFK","ATL"],["SFO","ATL"],["ATL","JFK"],["ATL","SFO"]];
adj list -> { JFK: [ATL, SFO], SFO: [ATL], ATL: [JFK, SFO] }
node = JFK, add (JFK, ATL) to edges visited
node = ATL, add (ATL, JFK) to edges visited
node = JFK, add (JFK, SFO) to edges visited
node = SFO, add (SFO, ATL) to edges visited
node = ATL, add (ATL, SFO) to edges visited
[JFK, ATL, JFK, SFO, ATL, SFO]

Complexity

building hash map -> Time => O(V+E), Space => O(V+E)
dfs -> Time => O(E), Space => O(E) because of recursion stack size
Additional space -> edges visited O(E) [as many edges as are available]
*/
