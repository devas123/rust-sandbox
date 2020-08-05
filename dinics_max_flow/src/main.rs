use std::cell::RefCell;
use std::cmp::min;
use std::collections::VecDeque;
use std::io;
use std::rc::Rc;

const INF: i32 = i32::max_value() / 2;

struct Edge {
    to: i32,
    flow: i32,
    residual: Option<Rc<RefCell<Edge>>>,
    capacity: i32,
}


struct NetworkSolver {
    graph: Vec<Vec<Rc<RefCell<Edge>>>>,
    level: Vec<i32>,
    solved: bool,
    n: usize,
    t: i32,
    s: i32,
    maxflow: i32,
}

impl NetworkSolver {
    #[inline]
    pub fn new(n: usize, s: i32, t: i32) -> NetworkSolver {
        let mut graph: Vec<Vec<Rc<RefCell<Edge>>>> = Vec::with_capacity(n);
        for _i in 0..n {
            let tmp = Vec::<Rc<RefCell<Edge>>>::new();
            graph.push(tmp);
        }
        let level = vec![0; n];
        NetworkSolver { graph, n, s, t, level, solved: false, maxflow: 0 }
    }

    pub fn add_edge(&mut self, from: i32, to: i32, capacity: i32) {
        let e1 = Edge::new(to, 0, capacity);
        let e2 = Edge::new(from, 0, 0);
        let e1rc = Rc::new(RefCell::new(e1));
        let e2rc = Rc::new(RefCell::new(e2));
        {
            e1rc.borrow_mut().residual.get_or_insert(e2rc.clone());
            let e1v: &mut Vec<Rc<RefCell<Edge>>> = self.graph.get_mut(from as usize).unwrap();
            e1v.push(e1rc.clone());
        }
        {
            e2rc.borrow_mut().residual.get_or_insert(e1rc.clone());
            let e2v: &mut Vec<Rc<RefCell<Edge>>> = self.graph.get_mut(to as usize).unwrap();
            e2v.push(e2rc.clone());
        }
    }

    pub fn solve(&mut self) -> i32 {
        if !self.solved {
            self.solved = true;
            while self.bfs() {
                let mut next = vec![0; self.n];
                let s = self.s;
                let mut f = self.dfs(s, &mut next, INF);
                while f != 0 {
                    self.maxflow += f;
                    let s = self.s;
                    f = self.dfs(s, &mut next, INF);
                }
            }
        }
        self.maxflow
    }

    fn bfs(&mut self) -> bool {
        self.level = vec![-1; self.n];
        self.level[self.s as usize] = 0;
        let mut deque = VecDeque::new();
        deque.push_front(self.s);
        while !deque.is_empty() {
            let node = deque.pop_back().unwrap();
            // println!("node: {}, level: {:?}, queue: {:?}", node, self.level, deque);
            for x in &self.graph[node as usize] {
                let cap = x.borrow().remaining_capacity();
                let to = x.borrow().to;
                if cap > 0 && self.level[to as usize] == -1 {
                    self.level[to as usize] = self.level[node as usize] + 1;
                    deque.push_front(to);
                }
            };
        }
        return self.level[self.t as usize] != -1;
    }

    fn dfs(&mut self, at: i32, next: &mut Vec<usize>, flow: i32) -> i32 {
        if at == self.t {
            flow
        } else {
            let a = at as usize;
            let num_edges = self.graph[a].len();
            while next[a] < num_edges {
                let cap;
                let to;
                {
                    let edge = self.graph[a].get(next[a]).unwrap().borrow();
                    cap = edge.remaining_capacity();
                    to = edge.to as usize;
                }
                if cap > 0 && self.level[to] == self.level[a] + 1 {
                    let bottleneck;
                    {
                        bottleneck = self.dfs(to as i32, next, min(flow, cap));
                    }
                    if bottleneck > 0 {
                        {
                            let mut edge = self.graph[a].get_mut(next[a]).unwrap().borrow_mut();
                            edge.augment(bottleneck);
                        }
                        return bottleneck;
                    }
                }
                next[a] += 1;
            }
            0
        }
    }
}

impl Edge {
    #[inline]
    pub fn new(to: i32, flow: i32, capacity: i32) -> Edge {
        return Edge { residual: None, to, flow, capacity };
    }

    pub fn remaining_capacity(&self) -> i32 {
        return self.capacity - self.flow;
    }

    pub fn augment(&mut self, bottleneck: i32) {
        self.flow += bottleneck;
        let rc = self.residual.as_mut().unwrap();
        rc.borrow_mut().flow -= bottleneck;
    }
}

fn main() {
    let mut bytes;
    let mut buffer = String::new();
    bytes = io::stdin().read_line(&mut buffer).expect("Error reading string");
    let mut a = buffer.split_whitespace().map(|x: &str| { x.parse::<i32>().unwrap() });
    let vert_num = a.next().unwrap();
    let edge_num = a.next().unwrap();
    let mut i = 0;
    let mut network_solver = NetworkSolver::new(vert_num as usize, 0, vert_num - 1);
    while bytes > 1 && i < edge_num {
        i += 1;
        let mut buffer = String::new();
        bytes = io::stdin().read_line(&mut buffer).expect("Error reading string");
        let mut iter = buffer.split_whitespace();
        let mut vertices: Vec<usize> = Vec::new();
        loop {
            match iter.next() {
                Some(str) => {
                    let value: usize = str.parse().expect("Parse error ");
                    vertices.push(value)
                }
                None => {
                    if !vertices.is_empty() && vertices.len() == 3 {
                        let v1 = vertices.get(0).expect("Error");
                        let capacity = vertices.get(2).expect("Error");
                        let v2 = vertices.get(1).expect("Error");
                        network_solver.add_edge(*v1 as i32, *v2 as i32, *capacity as i32);
                    }
                    break;
                }
            };
        }
    }
    println!("{}", network_solver.solve());
}
