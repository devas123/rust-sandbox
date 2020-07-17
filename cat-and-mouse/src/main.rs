use std::collections::{VecDeque};
use std::mem::replace;

fn main() {
    println!("{}", cat_mouse_game(vec![vec![2, 5], vec![3], vec![0, 4, 5], vec![1, 4, 5], vec![2, 3], vec![0, 2, 3]]));
    println!("{}", cat_mouse_game(vec![vec![1,3],vec![0],vec![3],vec![0,2]]));
    // println!("{}", cat_mouse_game(vec![vec![2,3],vec![3,4],vec![0,4],vec![0,1],vec![1,2]]));
    // println!("{}", cat_mouse_game(vec![vec![6], vec![4], vec![9], vec![5], vec![1, 5], vec![3, 4, 6], vec![0, 5, 10], vec![8, 9, 10], vec![7], vec![2, 7], vec![6, 7]]));
}


/**
A game on an undirected graph is played by two players, Mouse and Cat, who alternate turns.

The graph is given as follows: graph[a] is a list of all nodes b such that ab is an edge of the graph.

Mouse starts at node 1 and goes first, Cat starts at node 2 and goes second, and there is a Hole at node 0.

During each player's turn, they must travel along one edge of the graph that meets where they are.  For example, if the Mouse is at node 1, it must travel to any node in graph[1].

Additionally, it is not allowed for the Cat to travel to the Hole (node 0.)

Then, the game can end in 3 ways:

If ever the Cat occupies the same node as the Mouse, the Cat wins.
If ever the Mouse reaches the Hole, the Mouse wins.
If ever a position is repeated (ie. the players are in the same position as a previous turn, and it is the same player's turn to move), the game is a draw.
Given a graph, and assuming both players play optimally, return 1 if the game is won by Mouse, 2 if the game is won by Cat, and 0 if the game is a draw.


Example 1:

Input: [[2,5],[3],[0,4,5],[1,4,5],[2,3],[0,2,3]]
Output: 0
Explanation:
4---3---1
|   |
2---5
\ /
0


Note:

3 <= graph.length <= 50
It is guaranteed that graph[1] is non-empty.
It is guaranteed that graph[2] contains a non-zero element.
*/
pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
    let n = graph.len();
    let mut color: Vec<Vec<Vec<usize>>> = vec![vec![vec![0;2];n];n];
    let mut degree: Vec<Vec<Vec<usize>>> = vec![vec![vec![0;2];n];n];
    const CAT_TURN: usize = 1;
    const MOUSE_TURN: usize = 0;
    const CAT_WINS: usize = 2;
    const MOUSE_WINS: usize = 1;

    for i in 0..n {
        for j in 0..n {
            replace(&mut degree[i][j][MOUSE_TURN], graph[j].len());
            replace(&mut degree[i][j][CAT_TURN], graph[i].len());
            for k in &graph[i] {
                if *k == 0 {
                    let len = degree[i][j][CAT_TURN];
                    replace(&mut degree[i][j][CAT_TURN], len - 1);
                    break;
                }
            }
        }
    };

    let mut q: VecDeque<(usize, usize, usize, usize)> = VecDeque::new();

    for k in 0..n {
        for m in 0..2 {
            replace(&mut color[k][k][m], CAT_WINS);
            q.push_back((k, k, m, CAT_WINS));
            replace(&mut color[k][0][m], MOUSE_WINS);
            q.push_back((k, 0, m, MOUSE_WINS));
        }
    }

    while !q.is_empty() {
        let tmp = q.pop_front().unwrap();
        let prev_turn = 1 - tmp.2;
        let parents_of = if prev_turn == CAT_TURN { tmp.0 } else { tmp.1 };
        let prev_cat_or_mouse = if prev_turn == CAT_TURN { tmp.1 } else { tmp.0 };
        let c = tmp.3;
        if tmp.0 == 2 && tmp.1 == 1 && tmp.2 == MOUSE_TURN {
            return c as i32;
        }
        for prev_vertex in &graph[parents_of] {
            let prev_cat = if prev_turn == CAT_TURN { *prev_vertex as usize } else { prev_cat_or_mouse };
            let prev_mouse = if prev_turn == CAT_TURN { prev_cat_or_mouse } else { *prev_vertex  as usize };
            if prev_cat == 0 { continue; }
            if color[prev_cat][prev_mouse][prev_turn] > 0 { continue; }
            if prev_turn == 1 && c == 2 || prev_turn == 0 && c == 1 {
                replace(&mut color[prev_cat][prev_mouse][prev_turn], c);
                q.push_back((prev_cat, prev_mouse, prev_turn, c))
            } else if degree[prev_cat][prev_mouse][prev_turn] == 0 {
                replace(&mut color[prev_cat][prev_mouse][prev_turn], c);
                q.push_back((prev_cat, prev_mouse, prev_turn, c))
            } else {
                let cur_degr = degree[prev_cat][prev_mouse][prev_turn];
                replace(&mut degree[prev_cat][prev_mouse][prev_turn], cur_degr - 1);
                if degree[prev_cat][prev_mouse][prev_turn] == 0 {
                    replace(&mut color[prev_cat][prev_mouse][prev_turn], c);
                    q.push_back((prev_cat, prev_mouse, prev_turn, c))
                }
            }
        }
    }

    return 0;
}