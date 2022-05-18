use std::cmp;
use std::io;

impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        // println!("n:{} connections:{:?}", n, connections);
        let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
        for connection in connections {
            g[connection[0] as usize].push(connection[1] as usize);
            g[connection[1] as usize].push(connection[0] as usize);
        }
        let mut low = vec![0; n];
        let mut ret: Vec<Vec<i32>> = Vec::new();

        Solution::tarjan(&g, 0, &mut 1, &mut low, &mut vec![false; n], 0, &mut ret);
        ret
    }

    pub fn tarjan(g: &Vec<Vec<usize>>, node_id: usize, idx: &mut u32, low: &mut [u32],
                  visited_nodes: &mut [bool], prev_node: usize, ret: &mut Vec<Vec<i32>>) {
        // println!("visit:{} target_nodes:{:?}", node_id, g[node_id]);
        let current_idx = *idx;
        *idx += 1;
        visited_nodes[node_id] = true;
        low[node_id] = current_idx;
        for target_node in &g[node_id] {
            if !visited_nodes[*target_node] {
                Solution::tarjan(g, *target_node, idx, low, visited_nodes, node_id, ret);
            }
            if *target_node != prev_node {
                low[node_id] = cmp::min(low[node_id], low[*target_node]);
            }
        }
        if low[node_id] == current_idx && node_id != 0 {
            ret.push(vec![prev_node as i32, node_id as i32]);
        }
        // println!("low:{:?} current_idx:{} node_id:{} ret:{:?}", low, current_idx, node_id, ret);
    }
}

struct Solution();


fn main() {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let input = input.trim();

        let n = input.parse().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let mut connections: Vec<Vec<i32>> = Vec::new();
        for row in input.split("],[") {
            let mut row = row.replace("[[", "");
            row = row.replace("]]", "");
            connections.push(Vec::new());
            let l = connections.len();
            for column in row.split(',') {
                connections[l - 1].push(column.parse().unwrap());
            }
        }
        println!("{:?}", Solution::critical_connections(n, connections));
    }
}
