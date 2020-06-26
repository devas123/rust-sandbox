use std::cmp::Ordering;
use std::collections::{BinaryHeap, LinkedList};
use std::mem::replace;

fn main() {
    println!("{}", shortest_bridge(vec![vec![0, 1], vec![1, 0]]));
    println!("{}", shortest_bridge(vec![vec![1, 1, 1, 1, 1], vec![1, 0, 0, 0, 1], vec![1, 0, 1, 0, 1], vec![1, 0, 0, 0, 1], vec![1, 1, 1, 1, 1]]));
    println!("{}", shortest_bridge(vec![vec![0, 1, 0], vec![0, 0, 0], vec![0, 0, 1]]));
}

#[derive(Eq)]
struct SearchNode {
    i: usize,
    j: usize,
    dist: i32,
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist && self.i == other.i && self.j == other.j
    }
}

pub fn shortest_bridge(a: Vec<Vec<i32>>) -> i32 {
    fn add_neighbors_dfs(queue_stack: &mut LinkedList<(usize, usize)>, n: usize, visited: &Vec<Vec<bool>>, i: usize, j: usize) {
        if i >= 1 {
            if !visited[i - 1][j] {
                queue_stack.push_front((i - 1, j));
            }
        }
        if i < n - 1 {
            if !visited[i + 1][j] {
                queue_stack.push_front((i + 1, j));
            }
        }
        if j >= 1 {
            if !visited[i][j - 1] {
                queue_stack.push_front((i, j - 1));
            }
        }
        if j < n - 1 {
            if !visited[i][j + 1] {
                queue_stack.push_front((i, j + 1));
            }
        }
    }

    let mut stack: LinkedList<(usize, usize)> = LinkedList::new();
    let mut search_queue: BinaryHeap<SearchNode> = BinaryHeap::new(); // min-heap
    let n = a.len();
    let mut visited = vec![vec![false; n]; n];
    let mut answer: i32 = 0;
    stack.push_front((0, 0));
    while !stack.is_empty() {
        let (i, j) = stack.pop_front().expect("Empty");
        replace(&mut visited[i][j], true);
        add_neighbors_dfs(&mut stack, n, &visited, i, j);
        if a[i][j] > 0 {
            search_queue.push(SearchNode {
                i,
                j,
                dist: 0,
            });
            break;
        }
    }

    while !search_queue.is_empty() {
        let node = search_queue.pop().expect("Empty");
        replace(&mut visited[node.i][node.j], true);
        if node.dist == 0 {
            // we're still on the source island;
            if node.i < n - 1 && !visited[node.i + 1][node.j] {
                replace(&mut visited[node.i + 1][node.j], true);
                search_queue.push(SearchNode {
                    i: node.i + 1,
                    j: node.j,
                    dist: if a[node.i + 1][node.j] == 0 { 1 } else { 0 },
                })
            }
            if node.i > 0 && !visited[node.i - 1][node.j] {
                replace(&mut visited[node.i - 1][node.j], true);
                search_queue.push(SearchNode {
                    i: node.i - 1,
                    j: node.j,
                    dist: if a[node.i - 1][node.j] == 0 { 1 } else { 0 },
                })
            }
            if node.j < n - 1 && !visited[node.i][node.j + 1] {
                replace(&mut visited[node.i][node.j + 1], true);
                search_queue.push(SearchNode {
                    i: node.i,
                    j: node.j + 1,
                    dist: if a[node.i][node.j + 1] == 0 { 1 } else { 0 },
                })
            }
            if node.j > 0 && !visited[node.i][node.j - 1] {
                replace(&mut visited[node.i][node.j - 1], true);
                search_queue.push(SearchNode {
                    i: node.i,
                    j: node.j - 1,
                    dist: if a[node.i][node.j - 1] == 0 { 1 } else { 0 },
                })
            }
        } else {
            //we're off the 1st island
            if node.i < n - 1 && !visited[node.i + 1][node.j] {
                replace(&mut visited[node.i + 1][node.j], true);
                if a[node.i + 1][node.j] == 0 {
                    search_queue.push(SearchNode {
                        i: node.i + 1,
                        j: node.j,
                        dist: node.dist + 1,
                    })
                } else {
                    answer = node.dist;
                    break;
                }
            }
            if node.i > 0 && !visited[node.i - 1][node.j] {
                replace(&mut visited[node.i - 1][node.j], true);
                if a[node.i - 1][node.j] == 0 {
                    search_queue.push(SearchNode {
                        i: node.i - 1,
                        j: node.j,
                        dist: node.dist + 1,
                    })
                } else {
                    answer = node.dist;
                    break;
                }
            }
            if node.j < n - 1 && !visited[node.i][node.j + 1] {
                replace(&mut visited[node.i][node.j + 1], true);
                if a[node.i][node.j + 1] == 0 {
                    search_queue.push(SearchNode {
                        i: node.i,
                        j: node.j + 1,
                        dist: node.dist + 1,
                    })
                } else {
                    answer = node.dist;
                    break;
                }
            }
            if node.j > 0 && !visited[node.i][node.j - 1] {
                replace(&mut visited[node.i][node.j - 1], true);
                if a[node.i][node.j - 1] == 0 {
                    search_queue.push(SearchNode {
                        i: node.i,
                        j: node.j - 1,
                        dist: node.dist + 1,
                    })
                } else {
                    answer = node.dist;
                    break;
                }
            }
        }
    }
    return answer;
}
