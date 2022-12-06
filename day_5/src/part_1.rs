use std::fmt::Debug;

fn get_cargo_movements() -> Vec<CrateMovement> {
    let movement_strings = vec![
        "move 3 from 5 to 7",
        "move 2 from 8 to 9",
        "move 4 from 3 to 5",
        "move 2 from 1 to 7",
        "move 1 from 3 to 6",
        "move 2 from 1 to 7",
        "move 1 from 8 to 7",
        "move 4 from 2 to 8",
        "move 10 from 9 to 1",
        "move 6 from 6 to 2",
        "move 1 from 6 to 7",
        "move 9 from 8 to 6",
        "move 4 from 2 to 4",
        "move 2 from 4 to 1",
        "move 6 from 1 to 6",
        "move 1 from 3 to 2",
        "move 2 from 1 to 4",
        "move 2 from 4 to 3",
        "move 2 from 1 to 3",
        "move 4 from 3 to 1",
        "move 15 from 7 to 9",
        "move 4 from 5 to 9",
        "move 13 from 9 to 4",
        "move 10 from 4 to 8",
        "move 1 from 7 to 4",
        "move 6 from 9 to 5",
        "move 11 from 6 to 7",
        "move 4 from 5 to 7",
        "move 3 from 8 to 7",
        "move 4 from 2 to 4",
        "move 1 from 5 to 1",
        "move 5 from 8 to 4",
        "move 1 from 5 to 4",
        "move 10 from 7 to 1",
        "move 8 from 7 to 9",
        "move 12 from 1 to 9",
        "move 8 from 9 to 1",
        "move 2 from 6 to 9",
        "move 2 from 8 to 4",
        "move 1 from 6 to 9",
        "move 13 from 4 to 2",
        "move 13 from 4 to 2",
        "move 1 from 6 to 1",
        "move 1 from 6 to 4",
        "move 1 from 4 to 5",
        "move 14 from 1 to 8",
        "move 1 from 5 to 4",
        "move 13 from 9 to 5",
        "move 9 from 8 to 2",
        "move 8 from 2 to 1",
        "move 5 from 8 to 2",
        "move 5 from 1 to 6",
        "move 3 from 1 to 3",
        "move 1 from 4 to 8",
        "move 9 from 5 to 9",
        "move 18 from 2 to 8",
        "move 3 from 3 to 5",
        "move 2 from 6 to 4",
        "move 14 from 2 to 7",
        "move 1 from 4 to 2",
        "move 1 from 6 to 9",
        "move 1 from 2 to 5",
        "move 1 from 6 to 2",
        "move 1 from 4 to 6",
        "move 6 from 8 to 1",
        "move 2 from 6 to 9",
        "move 5 from 5 to 3",
        "move 1 from 7 to 8",
        "move 10 from 9 to 7",
        "move 13 from 8 to 5",
        "move 5 from 5 to 2",
        "move 6 from 5 to 7",
        "move 1 from 8 to 5",
        "move 5 from 5 to 9",
        "move 5 from 9 to 7",
        "move 4 from 3 to 8",
        "move 6 from 1 to 6",
        "move 4 from 2 to 4",
        "move 3 from 7 to 5",
        "move 2 from 2 to 9",
        "move 1 from 3 to 7",
        "move 29 from 7 to 9",
        "move 4 from 5 to 2",
        "move 5 from 6 to 4",
        "move 3 from 7 to 9",
        "move 3 from 8 to 6",
        "move 1 from 2 to 6",
        "move 3 from 2 to 5",
        "move 1 from 8 to 4",
        "move 1 from 5 to 9",
        "move 8 from 4 to 9",
        "move 15 from 9 to 2",
        "move 1 from 5 to 1",
        "move 10 from 9 to 4",
        "move 5 from 4 to 5",
        "move 5 from 5 to 4",
        "move 1 from 1 to 9",
        "move 1 from 4 to 3",
        "move 8 from 2 to 4",
        "move 7 from 2 to 7",
        "move 1 from 3 to 8",
        "move 1 from 5 to 6",
        "move 4 from 7 to 3",
        "move 1 from 8 to 2",
        "move 7 from 4 to 7",
        "move 11 from 9 to 7",
        "move 5 from 4 to 2",
        "move 3 from 9 to 6",
        "move 3 from 3 to 8",
        "move 4 from 2 to 4",
        "move 5 from 9 to 5",
        "move 1 from 2 to 1",
        "move 3 from 8 to 5",
        "move 2 from 9 to 1",
        "move 1 from 2 to 5",
        "move 2 from 9 to 6",
        "move 3 from 7 to 5",
        "move 7 from 4 to 1",
        "move 4 from 4 to 9",
        "move 3 from 7 to 2",
        "move 3 from 1 to 9",
        "move 1 from 2 to 3",
        "move 2 from 7 to 9",
        "move 6 from 5 to 4",
        "move 6 from 4 to 3",
        "move 5 from 5 to 1",
        "move 6 from 7 to 8",
        "move 1 from 5 to 1",
        "move 2 from 9 to 4",
        "move 1 from 4 to 3",
        "move 10 from 6 to 4",
        "move 2 from 2 to 1",
        "move 6 from 4 to 1",
        "move 5 from 8 to 3",
        "move 1 from 8 to 2",
        "move 7 from 3 to 9",
        "move 1 from 6 to 9",
        "move 2 from 7 to 3",
        "move 20 from 1 to 6",
        "move 7 from 3 to 8",
        "move 2 from 9 to 6",
        "move 1 from 2 to 3",
        "move 2 from 3 to 6",
        "move 1 from 1 to 4",
        "move 6 from 4 to 7",
        "move 5 from 8 to 3",
        "move 22 from 6 to 4",
        "move 2 from 9 to 7",
        "move 3 from 3 to 4",
        "move 6 from 4 to 2",
        "move 11 from 9 to 3",
        "move 9 from 3 to 7",
        "move 5 from 4 to 2",
        "move 5 from 7 to 2",
        "move 5 from 7 to 6",
        "move 10 from 2 to 4",
        "move 3 from 2 to 1",
        "move 1 from 6 to 3",
        "move 1 from 1 to 7",
        "move 17 from 4 to 1",
        "move 1 from 8 to 4",
        "move 2 from 7 to 5",
        "move 3 from 2 to 5",
        "move 3 from 3 to 8",
        "move 4 from 5 to 1",
        "move 3 from 3 to 7",
        "move 1 from 4 to 5",
        "move 21 from 1 to 5",
        "move 3 from 8 to 3",
        "move 4 from 7 to 5",
        "move 1 from 1 to 7",
        "move 1 from 6 to 3",
        "move 4 from 4 to 1",
        "move 1 from 8 to 1",
        "move 3 from 4 to 9",
        "move 5 from 1 to 8",
        "move 3 from 9 to 3",
        "move 5 from 6 to 1",
        "move 5 from 1 to 4",
        "move 6 from 3 to 2",
        "move 1 from 3 to 2",
        "move 3 from 8 to 1",
        "move 7 from 2 to 1",
        "move 10 from 5 to 2",
        "move 12 from 5 to 7",
        "move 2 from 8 to 3",
        "move 5 from 5 to 8",
        "move 8 from 1 to 6",
        "move 5 from 4 to 5",
        "move 3 from 8 to 6",
        "move 1 from 8 to 3",
        "move 6 from 6 to 7",
        "move 2 from 3 to 8",
        "move 3 from 2 to 1",
        "move 6 from 2 to 9",
        "move 2 from 8 to 4",
        "move 1 from 3 to 9",
        "move 1 from 8 to 6",
        "move 1 from 6 to 9",
        "move 7 from 9 to 5",
        "move 1 from 9 to 7",
        "move 1 from 4 to 6",
        "move 2 from 6 to 5",
        "move 1 from 4 to 1",
        "move 1 from 2 to 7",
        "move 5 from 1 to 2",
        "move 10 from 7 to 4",
        "move 12 from 5 to 7",
        "move 6 from 4 to 8",
        "move 2 from 5 to 6",
        "move 1 from 8 to 9",
        "move 1 from 9 to 5",
        "move 30 from 7 to 9",
        "move 4 from 8 to 4",
        "move 1 from 8 to 7",
        "move 2 from 1 to 4",
        "move 6 from 6 to 3",
        "move 1 from 4 to 1",
        "move 1 from 1 to 2",
        "move 8 from 4 to 8",
        "move 1 from 4 to 5",
        "move 2 from 5 to 6",
        "move 2 from 9 to 8",
        "move 3 from 2 to 1",
        "move 4 from 3 to 2",
        "move 1 from 6 to 4",
        "move 1 from 7 to 1",
        "move 2 from 8 to 2",
        "move 1 from 9 to 2",
        "move 2 from 3 to 2",
        "move 1 from 4 to 2",
        "move 4 from 9 to 6",
        "move 3 from 6 to 4",
        "move 21 from 9 to 8",
        "move 13 from 2 to 7",
        "move 9 from 8 to 5",
        "move 3 from 1 to 4",
        "move 14 from 7 to 2",
        "move 5 from 8 to 9",
        "move 1 from 1 to 2",
        "move 7 from 8 to 6",
        "move 2 from 8 to 2",
        "move 8 from 6 to 9",
        "move 1 from 4 to 5",
        "move 5 from 8 to 2",
        "move 4 from 5 to 9",
        "move 9 from 9 to 6",
        "move 2 from 7 to 6",
        "move 1 from 8 to 7",
        "move 9 from 6 to 4",
        "move 1 from 6 to 5",
        "move 1 from 7 to 3",
        "move 1 from 4 to 7",
        "move 1 from 7 to 2",
        "move 9 from 2 to 3",
        "move 8 from 4 to 1",
        "move 8 from 9 to 2",
        "move 2 from 6 to 5",
        "move 4 from 5 to 2",
        "move 2 from 9 to 5",
        "move 1 from 4 to 9",
        "move 10 from 3 to 7",
        "move 1 from 9 to 2",
        "move 1 from 5 to 3",
        "move 7 from 2 to 8",
        "move 7 from 1 to 5",
        "move 1 from 1 to 2",
        "move 2 from 8 to 2",
        "move 1 from 3 to 5",
        "move 2 from 8 to 6",
        "move 2 from 8 to 9",
        "move 2 from 4 to 6",
        "move 3 from 2 to 8",
        "move 3 from 6 to 7",
        "move 7 from 5 to 8",
        "move 7 from 2 to 7",
        "move 1 from 6 to 8",
        "move 5 from 2 to 7",
        "move 6 from 8 to 3",
        "move 2 from 7 to 1",
        "move 7 from 2 to 5",
        "move 1 from 3 to 5",
        "move 1 from 1 to 5",
        "move 2 from 9 to 7",
        "move 4 from 3 to 7",
        "move 2 from 4 to 6",
        "move 1 from 1 to 6",
        "move 1 from 2 to 4",
        "move 16 from 5 to 6",
        "move 1 from 4 to 9",
        "move 19 from 6 to 1",
        "move 1 from 3 to 5",
        "move 1 from 9 to 1",
        "move 1 from 8 to 5",
        "move 5 from 8 to 3",
        "move 5 from 7 to 2",
        "move 3 from 2 to 9",
        "move 5 from 1 to 7",
        "move 2 from 5 to 1",
        "move 3 from 9 to 4",
        "move 4 from 1 to 9",
        "move 2 from 2 to 8",
        "move 2 from 8 to 6",
        "move 1 from 6 to 9",
        "move 4 from 3 to 8",
        "move 4 from 8 to 3",
        "move 2 from 3 to 8",
        "move 1 from 8 to 2",
        "move 1 from 9 to 7",
        "move 10 from 1 to 7",
        "move 26 from 7 to 6",
        "move 3 from 9 to 3",
        "move 1 from 4 to 6",
        "move 2 from 1 to 4",
        "move 1 from 1 to 6",
        "move 1 from 9 to 3",
        "move 1 from 2 to 3",
        "move 4 from 4 to 9",
        "move 10 from 7 to 8",
        "move 3 from 7 to 4",
        "move 4 from 9 to 4",
        "move 4 from 4 to 7",
        "move 4 from 3 to 9",
        "move 5 from 7 to 5",
        "move 3 from 5 to 1",
        "move 3 from 9 to 8",
        "move 3 from 1 to 5",
        "move 2 from 3 to 5",
        "move 7 from 8 to 1",
        "move 7 from 8 to 9",
        "move 4 from 6 to 3",
        "move 3 from 3 to 6",
        "move 1 from 3 to 4",
        "move 2 from 4 to 1",
        "move 1 from 9 to 6",
        "move 4 from 1 to 3",
        "move 3 from 5 to 1",
        "move 1 from 5 to 2",
        "move 6 from 1 to 2",
        "move 6 from 2 to 7",
        "move 2 from 7 to 4",
        "move 1 from 2 to 6",
        "move 1 from 1 to 4",
        "move 3 from 5 to 7",
        "move 6 from 7 to 4",
        "move 1 from 9 to 3",
        "move 1 from 3 to 6",
        "move 4 from 4 to 3",
        "move 9 from 6 to 1",
        "move 10 from 1 to 6",
        "move 7 from 4 to 5",
        "move 28 from 6 to 4",
        "move 3 from 6 to 7",
        "move 3 from 3 to 8",
        "move 4 from 5 to 7",
        "move 1 from 8 to 4",
        "move 18 from 4 to 7",
        "move 8 from 7 to 6",
        "move 6 from 4 to 1",
        "move 2 from 5 to 4",
        "move 8 from 6 to 1",
        "move 2 from 8 to 9",
        "move 1 from 5 to 3",
        "move 1 from 9 to 1",
        "move 5 from 9 to 2",
        "move 2 from 9 to 3",
        "move 1 from 2 to 5",
        "move 2 from 1 to 5",
        "move 6 from 7 to 5",
        "move 1 from 6 to 4",
        "move 6 from 5 to 9",
        "move 2 from 4 to 1",
        "move 8 from 1 to 8",
        "move 4 from 9 to 7",
        "move 1 from 5 to 6",
        "move 1 from 1 to 6",
        "move 2 from 1 to 2",
        "move 1 from 9 to 7",
        "move 3 from 2 to 4",
        "move 2 from 8 to 3",
        "move 5 from 8 to 2",
        "move 4 from 2 to 5",
        "move 1 from 8 to 9",
        "move 12 from 3 to 2",
        "move 2 from 6 to 2",
        "move 12 from 2 to 4",
        "move 6 from 2 to 3",
        "move 4 from 1 to 9",
        "move 8 from 4 to 7",
        "move 3 from 3 to 4",
        "move 1 from 5 to 4",
        "move 5 from 9 to 6",
        "move 3 from 5 to 8",
        "move 1 from 9 to 1",
        "move 2 from 8 to 5",
        "move 3 from 5 to 6",
        "move 1 from 8 to 4",
        "move 4 from 7 to 8",
        "move 1 from 1 to 3",
        "move 2 from 8 to 3",
        "move 7 from 6 to 7",
        "move 1 from 3 to 7",
        "move 2 from 8 to 6",
        "move 22 from 7 to 8",
        "move 6 from 4 to 8",
        "move 5 from 8 to 6",
        "move 5 from 6 to 2",
        "move 4 from 2 to 3",
        "move 6 from 8 to 5",
        "move 4 from 4 to 7",
        "move 1 from 3 to 7",
        "move 4 from 4 to 5",
        "move 1 from 5 to 4",
        "move 2 from 6 to 5",
        "move 9 from 5 to 6",
        "move 10 from 6 to 7",
        "move 1 from 2 to 1",
        "move 3 from 4 to 8",
        "move 16 from 7 to 9",
        "move 1 from 7 to 8",
        "move 1 from 1 to 8",
        "move 1 from 8 to 3",
        "move 2 from 7 to 4",
        "move 15 from 8 to 1",
        "move 1 from 8 to 1",
        "move 4 from 8 to 4",
        "move 7 from 9 to 7",
        "move 3 from 5 to 9",
        "move 10 from 9 to 6",
        "move 2 from 9 to 2",
        "move 7 from 7 to 4",
        "move 9 from 3 to 2",
        "move 8 from 2 to 7",
        "move 1 from 8 to 4",
        "move 3 from 2 to 1",
        "move 9 from 7 to 1",
        "move 9 from 4 to 1",
        "move 2 from 7 to 5",
        "move 1 from 5 to 4",
        "move 1 from 5 to 2",
        "move 6 from 1 to 3",
        "move 16 from 1 to 2",
        "move 9 from 2 to 1",
        "move 5 from 6 to 9",
        "move 2 from 1 to 9",
        "move 1 from 2 to 5",
        "move 4 from 4 to 8",
        "move 2 from 8 to 2",
        "move 2 from 2 to 3",
        "move 17 from 1 to 2",
        "move 2 from 1 to 9",
        "move 13 from 2 to 8",
        "move 1 from 2 to 4",
        "move 11 from 8 to 3",
        "move 3 from 3 to 4",
        "move 3 from 9 to 2",
        "move 1 from 5 to 2",
        "move 1 from 9 to 3",
        "move 3 from 4 to 3",
        "move 1 from 4 to 9",
        "move 3 from 3 to 4",
        "move 1 from 8 to 7",
        "move 7 from 2 to 9",
        "move 3 from 1 to 7",
        "move 3 from 2 to 8",
        "move 3 from 7 to 9",
        "move 10 from 3 to 5",
        "move 3 from 6 to 9",
        "move 8 from 9 to 4",
        "move 1 from 2 to 1",
        "move 1 from 7 to 9",
        "move 2 from 2 to 3",
        "move 4 from 4 to 8",
        "move 1 from 6 to 2",
        "move 7 from 5 to 3",
        "move 1 from 5 to 2",
        "move 9 from 8 to 9",
        "move 12 from 3 to 8",
        "move 1 from 1 to 9",
        "move 9 from 8 to 6",
        "move 1 from 5 to 7",
        "move 1 from 5 to 4",
        "move 2 from 2 to 9",
        "move 1 from 2 to 6",
        "move 2 from 4 to 3",
        "move 9 from 4 to 8",
        "move 6 from 3 to 6",
        "move 12 from 6 to 2",
        "move 2 from 6 to 7",
        "move 8 from 8 to 3",
        "move 5 from 8 to 7",
        "move 3 from 6 to 5",
        "move 6 from 3 to 7",
        "move 6 from 7 to 6",
        "move 1 from 4 to 9",
        "move 4 from 6 to 5",
        "move 20 from 9 to 6",
        "move 4 from 9 to 8",
        "move 2 from 8 to 7",
        "move 4 from 6 to 4",
        "move 10 from 6 to 1",
    ];
    let mut movements: Vec<CrateMovement> = vec![];

    for movement_string in movement_strings {
        let splitted_string = movement_string.split(" ").collect::<Vec<&str>>();
        movements.push(CrateMovement {
            amount: splitted_string[1].to_string().parse::<usize>().unwrap(),
            from: splitted_string[3].to_string().parse::<usize>().unwrap(),
            to: splitted_string[5].to_string().parse::<usize>().unwrap(),
        });
    }

    return movements;
}

#[derive(Debug, Clone)]
pub struct Stack<T> {
    pub elements: Vec<T>,
}

#[derive(Debug, Clone)]
pub struct CrateMovement {
    pub from: usize,
    pub to: usize,
    pub amount: usize,
}

impl<T: Debug> Stack<T> {
    fn new() -> Self {
        Stack { elements: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        self.elements.len() == 0
    }

    pub fn push(&mut self, element: T) {
        self.elements.push(element);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub fn print_top(&self) {
        if self.is_empty() {
            println!(" ");
        } else {
            println!("{:?}", self.elements[self.elements.len() - 1]);
        }
    }
}

pub fn get_answer() {
    let crate_movements = get_cargo_movements();
    let mut stacks: Vec<Stack<String>> = vec![];

    stacks.push(Stack::new());
    stacks.push(Stack::new());
    stacks.push(Stack::new());
    stacks.push(Stack::new());
    stacks.push(Stack::new());
    stacks.push(Stack::new());
    stacks.push(Stack::new());
    stacks.push(Stack::new());
    stacks.push(Stack::new());

    stacks[0].push(String::from("W"));
    stacks[0].push(String::from("M"));
    stacks[0].push(String::from("L"));
    stacks[0].push(String::from("F"));

    stacks[1].push(String::from("B"));
    stacks[1].push(String::from("Z"));
    stacks[1].push(String::from("V"));
    stacks[1].push(String::from("M"));
    stacks[1].push(String::from("F"));

    stacks[2].push(String::from("H"));
    stacks[2].push(String::from("V"));
    stacks[2].push(String::from("R"));
    stacks[2].push(String::from("S"));
    stacks[2].push(String::from("L"));
    stacks[2].push(String::from("Q"));

    stacks[3].push(String::from("F"));
    stacks[3].push(String::from("S"));
    stacks[3].push(String::from("V"));
    stacks[3].push(String::from("Q"));
    stacks[3].push(String::from("P"));
    stacks[3].push(String::from("M"));
    stacks[3].push(String::from("T"));
    stacks[3].push(String::from("J"));

    stacks[4].push(String::from("L"));
    stacks[4].push(String::from("S"));
    stacks[4].push(String::from("W"));

    stacks[5].push(String::from("F"));
    stacks[5].push(String::from("V"));
    stacks[5].push(String::from("P"));
    stacks[5].push(String::from("M"));
    stacks[5].push(String::from("R"));
    stacks[5].push(String::from("J"));
    stacks[5].push(String::from("W"));

    stacks[6].push(String::from("J"));
    stacks[6].push(String::from("Q"));
    stacks[6].push(String::from("C"));
    stacks[6].push(String::from("P"));
    stacks[6].push(String::from("N"));
    stacks[6].push(String::from("R"));
    stacks[6].push(String::from("F"));

    stacks[7].push(String::from("V"));
    stacks[7].push(String::from("H"));
    stacks[7].push(String::from("P"));
    stacks[7].push(String::from("S"));
    stacks[7].push(String::from("Z"));
    stacks[7].push(String::from("W"));
    stacks[7].push(String::from("R"));
    stacks[7].push(String::from("B"));

    stacks[8].push(String::from("B"));
    stacks[8].push(String::from("M"));
    stacks[8].push(String::from("J"));
    stacks[8].push(String::from("C"));
    stacks[8].push(String::from("G"));
    stacks[8].push(String::from("H"));
    stacks[8].push(String::from("Z"));
    stacks[8].push(String::from("W"));

    for stack in stacks.clone() {
        stack.print_top();
    }

    for crate_movement in crate_movements {
        for _ in 0..crate_movement.amount {
            let moved_crate = stacks[crate_movement.from - 1].pop().unwrap();
            stacks[crate_movement.to - 1].push(moved_crate);
        }
    }

    println!("After movements...");

    for stack in stacks {
        stack.print_top();
    }
}
