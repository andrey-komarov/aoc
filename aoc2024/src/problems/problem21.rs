use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::fmt::Formatter;
use std::io::{BufRead, Write};
use anyhow::anyhow;
use crate::problems::common::{Readable, Solvable};

#[derive(Clone, PartialEq, Eq, Hash)]
enum Direction {
    Left, Right, Up, Down
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        }
    }

    fn button_pos(&self) -> Vector {
        match self {
            Direction::Left => Vector::new(1, -2),
            Direction::Right => Vector::new(1, 0),
            Direction::Up => Vector::new(0, -1),
            Direction::Down => Vector::new(1, -1),
        }
    }
}

#[derive(Clone)]
enum Action {
    Move(Direction),
    Type(char),
    Activate,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn shifted(&self, dir: &Direction) -> Self {
        let (dx, dy) = dir.offset();
        Pos {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    robots: Vec<Pos>,
    output: Vec<char>,
}

struct Panel {
    buttons: Vec<Vec<Option<Action>>>,
}

impl Panel {
    fn action(&self, pos: &Pos) -> Option<Action> {
        self.buttons.get(pos.x as usize)?.get(pos.y as usize)?.clone()
    }
}

struct StateMachine {
    panels: Vec<Panel>,
}

impl StateMachine {

    fn new(layers: usize) -> Self {
        let a = || Some(Action::Activate);
        let d = |digit| Some(Action::Type(digit));
        let m = |dir| {
            match dir {
                '>' => Some(Action::Move(Direction::Right)),
                '<' => Some(Action::Move(Direction::Left)),
                'v' => Some(Action::Move(Direction::Down)),
                '^' => Some(Action::Move(Direction::Up)),
                _ => None
            }
        };
        let mut panels: Vec<_> = (0..layers).map(|_| Panel { buttons: vec![
            vec![None, m('^'), a()],
            vec![m('<'), m('v'), m('>')],
        ]}).collect();
        panels.push(Panel { buttons: vec![
                vec![d('7'), d('8'), d('9')],
                vec![d('4'), d('5'), d('6')],
                vec![d('1'), d('2'), d('3')],
                vec![None, d('0'), d('A')],
            ]},
        );
        Self { panels }
    }

    fn start(&self) -> State {
        let mut robots = vec![Pos {x: 0, y: 2}; self.panels.len() - 1];
        robots.push(Pos {x: 3, y: 2});
        State {
            robots,
            output: vec![],
        }
    }

    fn go1(&self, state: &State, action: &Action, layer: usize) -> Option<State> {
        match action {
            Action::Move(dir) => {
                let new_pos = state.robots[layer].shifted(dir);
                self.panels[layer].action(&new_pos)?;
                let mut new_state = (*state).clone();
                new_state.robots[layer] = new_pos;
                Some(new_state)
            }
            Action::Type(msg) => {
                let mut new_state = (*state).clone();
                new_state.output.push(*msg);
                Some(new_state)
            }
            Action::Activate => {
                let pos = &state.robots[layer];
                let new_action = self.panels[layer].action(pos)?;
                self.go1(state, &new_action, layer + 1)
            }
        }
    }
    fn go(&self, state: &State, action: &Action) -> Option<State> {
        self.go1(state, action, 0)
    }
}

struct Input {
    codes: Vec<Vec<char>>
}

impl Readable for Input {
    fn parse_from<R: BufRead>(mut input: R) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        let codes = s.lines().filter(|line| !line.is_empty()).map(|line| {
            line.chars().map(|c| {
                match c {
                    'A' => Some('A'),
                    '0'..='9' => Some(c),
                    _ => None,
                }
            }).collect()
        }).collect::<Option<_>>().ok_or_else(|| anyhow!("Cannot parse codes"))?;
        Ok(Input {codes})
    }
}

type Output = i64;
pub(crate) struct PartOne {
    layers: usize,
}

impl PartOne {

    pub(crate) fn new(layers: usize) -> Self {
        Self { layers }
    }

    fn is_dead_end(&self, input: &Input, state: &State) -> bool {
        !input.codes.iter().any(|code| {
            code.len() >= state.output.len() &&
            code.iter().zip(state.output.iter()).all(|(code, out)| {
                *code == *out
            })
        })
    }
    fn solve(&self, input: Input) -> Output {
        let state_machine = StateMachine::new(self.layers);
        let state = state_machine.start();
        let mut distance = HashMap::from([(state.clone(), 0)]);
        let mut queue = VecDeque::from([(0, state)]);
        let mut remaining_codes: HashSet<Vec<char>> = HashSet::from_iter(input.codes.clone().into_iter());
        let mut answers = HashMap::new();
        while let Some((dist, state)) = queue.pop_front() {
            if remaining_codes.contains(&state.output) {
                remaining_codes.remove(&state.output);
                answers.insert(state.output.clone(), dist);
            }
            let actions = vec![
                Action::Activate,
                Action::Move(Direction::Left),
                Action::Move(Direction::Right),
                Action::Move(Direction::Up),
                Action::Move(Direction::Down),
            ];
            for action in actions {
                if let Some(new_state) = state_machine.go(&state, &action) {
                    if self.is_dead_end(&input, &new_state) {
                        continue
                    }
                    distance.entry(new_state).or_insert_with_key(|state| {
                        queue.push_back((dist + 1, (*state).clone()));
                        dist + 1
                    });
                }
            }
        }
        println!("Answers {:?}", answers);
        answers.into_iter().map(|(mut k, v)| {
            k.pop();
            String::from_iter(k.into_iter()).parse::<i64>().unwrap() * v
        }).sum()
    }
}
impl Solvable for PartOne {
    fn solve<R: BufRead, W: Write>(&self, input: R, mut output: W) -> anyhow::Result<()> {
        let input = Input::parse_from(input)?;
        let out = self.solve(input);
        writeln!(output, "{}", out)?;
        Ok(())
    }
}

pub(crate) struct PartTwo {
    layers: usize,
}

impl PartTwo {
    pub(crate) fn new(layers: usize) -> Self {
        Self { layers }
    }

    fn solve(&self, input: Input) -> Output {
        let mut x = PartTwoMut::new(self.layers);
        x.solve(input)
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Vector {
    dx: isize,
    dy: isize,
}

impl fmt::Debug for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.dx, self.dy)
    }
}

impl Vector {
    fn new(dx: isize, dy: isize) -> Self {
        Self { dx, dy }
    }

    fn to(&self, other: &Vector) -> Self {
        Self { dx: other.dx - self.dx, dy: other.dy - self.dy }
    }

    fn len(&self) -> usize {
        self.dx.abs() as usize + self.dy.abs() as usize
    }

    fn extra_clicks(&self) -> usize {
        let dx = if self.dx.abs() > 1 {
            self.dx.abs() as usize - 1
        } else { 0 };
        let dy = if self.dy.abs() > 1 {
            self.dy.abs() as usize - 1
        } else { 0 };
        dx + dy
    }

    fn to_paths(&self) -> Vec<Vec<Vector>> {
        let vertical = if self.dx < 0 { Direction::Up } else { Direction::Down };
        let horizontal = if self.dy < 0 { Direction::Left } else { Direction::Right };
        let start = Vector::new(0, 0);
        fn to_circular_path(points: &Vec<Vector>) -> Vec<Vector> {
            let mut res = Vec::new();
            let zero = Vector::new(0, 0);
            let mut cur = &zero;
            for point in points {
                res.push(cur.to(&point));
                cur = point;
            }
            res.push(cur.to(&zero));
            res
        }
        if self.dx == 0 {
            vec![to_circular_path(&vec![horizontal.button_pos()])]
        } else if self.dy == 0 {
            vec![to_circular_path(&vec![vertical.button_pos()])]
        } else {
            vec![
                to_circular_path(&vec![horizontal.button_pos(), vertical.button_pos()]),
                to_circular_path(&vec![vertical.button_pos(), horizontal.button_pos()]),
            ]
        }
    }
}

struct PartTwoMut {
    layers: usize,
    cache: HashMap<(usize, Vector), usize>,
}

impl PartTwoMut {


    fn new(layers: usize) -> Self {
        Self {
            layers,
            cache: HashMap::new(),
        }
    }

    fn click_at(&mut self, layer: usize, v: &Vector) -> usize {
        if layer == 0 {
            return 1;
        }
        if let Some(res) = self.cache.get(&(layer, v.clone())) {
            return *res;
        }
        let res = v.to_paths().into_iter().map(|path| {
            path.iter().map(|v| {
                self.click_at(layer - 1, &v) // + v.extra_clicks()
            }).sum::<usize>()
        }).min().unwrap() + v.extra_clicks();
        println!("click_at({}, {:?}) = ...", layer, v);
        let res2 = v.to_paths().into_iter().map(|path| {
            let x = path.iter().map(|v| {
                self.click_at(layer - 1, &v) // + v.extra_clicks()
            }).sum::<usize>();
            println!("... {:?}: {}", path, x );
            x
        }).min().unwrap();
        self.cache.insert((layer, v.clone()), res);
        println!("=== ({}, {:?}) = {:?}", layer, v, res);
        res
    }

    fn solve1(&mut self, input: Vec<char>) -> Output {
        fn button_pos(button: char) -> Vector {
            let panel = vec![
                vec!['7', '8', '9'],
                vec!['4', '5', '6'],
                vec!['1', '2', '3'],
                vec!['.', '0', 'A'],
            ];
            for (x, line) in panel.into_iter().enumerate() {
                for (y, c) in line.into_iter().enumerate() {
                    if c == button {
                        return Vector::new(x as isize, y as isize);
                    }
                }
            }
            panic!()
        }
        let mut pos = Vector::new(3, 2);
        let mut sum: Output = 0;
        for button in input.into_iter() {
            let v = button_pos(button);
            sum += self.click_at(self.layers + 1, &pos.to(&v)) as Output;
            pos = v;
        }
        sum
    }

    fn solve(&mut self, input: Input) -> Output {
        println!("{:?}", self.click_at(2, &Vector::new(1, -2)));
        // return 0;
        let answers: HashMap<_, _> = input.codes.into_iter().map(|code| {
            (code.clone(), self.solve1(code))
        }).collect();
        println!("Answers {:?}", answers);
        answers.into_iter().map(|(mut k, v)| {
            k.pop();
            String::from_iter(k.into_iter()).parse::<i64>().unwrap() * v
        }).sum()
    }
}

impl Solvable for PartTwo {
    fn solve<R: BufRead, W: Write>(&self, input: R, mut output: W) -> anyhow::Result<()> {
        let input = Input::parse_from(input)?;
        let out = self.solve(input);
        writeln!(output, "{}", out)?;
        Ok(())
    }
}