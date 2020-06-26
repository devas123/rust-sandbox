use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::mem;

fn main() {

    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut bytes = 10;
    let mut m: usize = 0;
    // let mut f = io::BufReader::new(File::open("text.txt").expect("Unable to open file"));
    while bytes > 1 {
        let mut buffer = String::new();
        bytes = io::stdin().read_line(&mut buffer).expect("Error reading string");
        // bytes = f.read_line(&mut buffer).expect("Error reading string");
        let mut iter = buffer.split_whitespace();
        let mut vertices: Vec<usize> = Vec::new();
        loop {
            match iter.next() {
                Some(str) => {
                    let value: usize = str.parse().expect("Parse error ");
                    m = max(m, value);
                    vertices.push(value)
                }
                None => {
                    if !vertices.is_empty() && vertices.len() == 2 {
                        let v1 = vertices.get(0).expect("Error");
                        let v2 = vertices.get(1).expect("Error");
                        graph.entry(*v1).or_insert(Vec::new()).push(*v2);
                        graph.entry(*v2).or_insert(Vec::new()).push(*v1);
                    }
                    break;
                }
            };
        }
    }

    let n: usize = m + 1;
    let mut id: u32 = 0;
    let mut low: Vec<u32> = vec![0; n];
    let mut ids: Vec<u32> = vec![0; n];
    let mut visited = vec![false; n];
    let mut root_oucoming_e_c: u32;

    fn dfs(root: usize,
           at: usize,
           parent: usize,
           articulation_points: &mut HashSet<usize>,
           vstd: &mut Vec<bool>,
           root_node_outcoming_edge_count: &mut u32,
           id: &mut u32,
           low: &mut Vec<u32>,
           ids: &mut Vec<u32>,
           graph: &HashMap<usize, Vec<usize>>) -> (HashSet<usize>, Vec<u32>, Vec<u32>) {
        if parent == root {
            *root_node_outcoming_edge_count += 1;
        }
        *id += 1;
        mem::replace(&mut vstd[at], true);
        mem::replace(&mut low[at], *id);
        mem::replace(&mut ids[at], *id);
        match graph.get(&at) {
            Some(vec) => {
                for to in vec {
                    if *to == parent {
                        continue;
                    }
                    if !vstd[*to] {
                        let (_b, l, is) = dfs(root,
                                              *to,
                                              at,
                                              articulation_points,
                                              vstd,
                                              root_node_outcoming_edge_count,
                                              id,
                                              low,
                                              ids,
                                              graph);
                        mem::replace(&mut low[at], min(l[at], l[*to]));
                        if is[at] <= l[*to] && at != root {
                            articulation_points.insert(at);
                        }
                    } else {
                        let tmp = min(low[at], ids[*to]);
                        mem::replace(&mut low[at], tmp);
                    }
                }
            }
            _ => {}
        }
        (articulation_points.to_owned(), low.to_owned(), ids.to_owned())
    }
    let mut points: HashSet<usize> = HashSet::new();
    for i in 0..(n - 1) {
        if !visited[i] {
            root_oucoming_e_c = 0;
            dfs(i, i, n, &mut points, &mut visited, &mut root_oucoming_e_c, &mut id, &mut low, &mut ids, &mut graph);
            if root_oucoming_e_c > 1 {
                points.insert(i);
            }
        }
    }

    for bridge in points {
        print!("{} ", bridge)
    }
    print!("\n");
}
