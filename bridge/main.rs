use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io::{self, BufRead};

use std::cmp::Ordering;

type Person = i32;
type Arc = Vec<Person>;
type SemiNode = Vec<Person>;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct Position {
    semi_node: SemiNode,
    direction: bool,
}
impl Ord for Position {
    fn cmp(&self, other: &Position) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .semi_node
            .cmp(&self.semi_node)
            .then_with(|| self.direction.cmp(&other.direction))
    }
}
impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    position: Position,
    cost: i32,
    path: Vec<Arc>,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn neighbors(
    initial_left: &SemiNode,
    current_left: &SemiNode,
    path: &Vec<Arc>,
) -> (bool, Vec<Arc>) {
    let mut posibilities: Vec<Arc> = Vec::new();
    let direction;

    //To the right: always 2 people
    if path.len() % 2 == 0 {
        direction = true;
        if current_left.len() == 1 {
            let mut p = Vec::new();
            p.push(current_left[0]);
            posibilities.push(p);
        }
        for e in current_left.iter() {
            for f in current_left.iter() {
                if e == f && current_left.len() <= 1 {
                    posibilities.push(vec![*e]); // maybe remove that
                } else if e < f {
                    posibilities.push(vec![*e, *f]);
                }
            }
        }
    } else {
        direction = false;
        for e in initial_left {
            if !current_left.contains(e) {
                posibilities.push(vec![*e]);
            }
        }
    }
    (direction, posibilities)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let _n_persons: u32 = lines.next().unwrap().parse().unwrap();
    let persons_left: SemiNode = lines.map(|num| num.parse().unwrap()).collect();

    // The graph we are building is a graph of (SemiNode) -Arc-> (SemiNode)
    // Where the seminode is person_left (as persons_right can be infered)
    let mut solution = State {
        cost: 0,
        position: Position {
            semi_node: persons_left.clone(),
            direction: true,
        },
        path: Vec::new(),
    };
    let mut visited: HashMap<Position, State> = HashMap::new();
    let mut to_visit = BinaryHeap::new();

    to_visit.push(State {
        cost: 0,
        position: Position {
            semi_node: persons_left.clone(),
            direction: true,
        },
        path: Vec::new(),
    });

    while let Some(State {
        cost,
        position,
        path,
    }) = to_visit.pop()
    {
        //println!("Visiting: {:?}, c: {}, P:{:?}", position, cost, path);
        if position.semi_node.len() == 0 {
            solution = State {
                cost: cost,
                position: position,
                path: path,
            };
            break;
        }
        match visited.get(&position) {
            Some(s) if cost > s.cost => continue,
            Some(_) | None => {}
        }

        let (dir, edges) = neighbors(&persons_left, &position.semi_node, &path);
        for edge in edges.iter() {
            // println!("nei: {:?}", edge);
            let mut new_position: Position = Position {
                direction: !position.direction,
                semi_node: match dir {
                    true => position
                        .semi_node
                        .iter()
                        .filter(|x| !edge.contains(x))
                        .map(|&x| x)
                        .collect(),
                    false => position
                        .semi_node
                        .iter()
                        .chain(edge.iter())
                        .map(|&x| x)
                        .collect(),
                },
            };
            new_position.semi_node.sort();
            let next = State {
                cost: cost + edge.iter().max().unwrap(),
                position: new_position,
                path: {
                    let mut p = path.clone();
                    p.push(edge.to_vec());
                    p
                },
            };
            match visited.get(&position) {
                Some(s) if next.cost < s.cost => to_visit.push(next),
                None => to_visit.push(next),
                Some(_) => {}
            }
        }

        // relaxation
        // println!("relax: {} {:?}", cost, path);
        visited.insert(
            position.clone(),
            State {
                cost: cost,
                position: position,
                path: path,
            },
        );
    }
    // println!("{:?}", visited);
    println!("{:?}", solution.cost);
    for a in solution.path.iter() {
        match a.len() {
            1 => println!("{}", a[0]),
            2 => println!("{} {}", a[0], a[1]),
            _ => {}
        }
    }
}
