use std::io;
use std::cmp;
use std::fmt;
use std::fmt::Write;
use std::collections::HashMap;
use std::collections::HashSet;

macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            writeln!(&mut ::std::io::stderr(), $($arg)*).ok();
        }
    )
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Owner {
    Me,
    Opponent,
    Neutral,
}

impl Owner {
    fn try_from(i: i32) -> Result<Self, ()> {
        match i {
            1   => Ok(Owner::Me),
            -1  => Ok(Owner::Opponent),
            0   => Ok(Owner::Neutral),
            _   => Err(())
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Factory {
    id: i32,
    owner: Owner,
    cyborgs: i32,
    production: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Troop {
    id: i32,
    owner: Owner,
    src: i32,
    dst: i32,
    cyborgs: i32,
    turns_left: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Bomb {
    id: i32,
    owner: Owner,
    src: i32,
    dst: i32,
    turns_left: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Action {
    Move(Move),
    Wait,
    Bomb(ThrowBomb),
    Upgrade(i32),
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Action::Move(Move {src: s, dst: d, cyborgs: n}) => write!(f, "MOVE {} {} {}", s, d, n),
            &Action::Wait => write!(f, "WAIT"),
            &Action::Bomb(ThrowBomb {src: s, dst: d}) => write!(f, "BOMB {} {}", s, d),
            &Action::Upgrade(i) => write!(f, "INC {}", i),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Move {
    src: i32,
    dst: i32,
    cyborgs: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct ThrowBomb {
    src: i32,
    dst: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct GlobalData {
    turn_no: i32,
    bombs_remaining: i32,
}

fn main() {

    let (factory_count, link_count, links) = read_init();
    print_err!("Links: {:?}", &links);

    let mut globals = GlobalData { turn_no: 0, bombs_remaining: 2 };

    // game loop
    loop {
        globals.turn_no += 1;

        let (factories, troops, bombs) = read_loop(factory_count);
        print_err!("Globals: {:?}", &globals);
        print_err!("Factories: {:?}", &factories);
        print_err!("Troops: {:?}", &troops);
        print_err!("Bombs: {:?}", &bombs);

        let actions = decide_action(&links, &factories, &troops, &bombs, &mut globals);
        let mut actions_string = String::new();
        for action in actions {
            let mut a_str = String::new();
            write!(&mut a_str, "{}", &action).expect("whoops");
            actions_string.push_str(&a_str);
            actions_string.push_str("; ");
        }
        actions_string.pop();
        actions_string.pop();
        print_err!("actions_string: {:?}", &actions_string);

        println!("{}", &actions_string);
    }
}

fn decide_action(links: &HashMap<(i32, i32), i32>, factories: &HashSet<Factory>, troops: &HashSet<Troop>, bombs: &HashSet<Bomb>, globals: &mut GlobalData) -> Vec<Action> {
    // Any valid action, such as "WAIT" or "MOVE source destination cyborgs"
    let mut actions = Vec::new();

    let op_ids = factories.iter()
                    .filter(|&&f| f.owner != Owner::Me )
                    .map(|&f| f.id)
                    .collect::<HashSet<_>>();

    for &my_f in factories.iter()
                    .filter(|&&f| f.owner == Owner::Me) {
        let nearest_id = links.iter()
                            .filter(|&(&(a, b), &dist)| a == my_f.id)
                            .filter(|&(&(a, b), &dist)| op_ids.contains(&b))
                            .min_by_key(|&(&(a, b), &dist)| dist)
                            .map(|(&(a, b), &dist)| b);
        //print_err!("justdebug1: {:?}", &nearest_id);
        if let Some(n_id) = nearest_id {
            let half: i32 = my_f.cyborgs / 2;
            let other: i32 = factories.iter()
                                .filter(|&&f| f.id == n_id )
                                .next()
                                .map(|&f| f.cyborgs)
                                .unwrap_or(0);
            let mut amount = cmp::max(half, other + 1);
            if my_f.cyborgs <= other {
                amount = 0;
            }
            //print_err!("justdebug2: {:?}", &amount);
            actions.push(Action::Move(Move{
                    src: my_f.id,
                    dst: n_id,
                    cyborgs: amount }));
        }
    }

    if (globals.turn_no == 5 || globals.turn_no == 10) && globals.bombs_remaining > 0 {
        let op_3_strongest = factories.iter()
                                .filter(|&&f| f.owner == Owner::Opponent)
                                .filter(|&&f| f.production == 3)
                                .max_by_key(|&f| f.cyborgs);
        let op_2_strongest = factories.iter()
                                .filter(|&&f| f.owner == Owner::Opponent)
                                .filter(|&&f| f.production == 2)
                                .max_by_key(|&f| f.cyborgs);
        if let Some(my_first) = factories.iter()
                            .filter(|&&f| f.owner == Owner::Me)
                            .take(1).next() {
            if let Some(f) = op_3_strongest {
                actions.push(Action::Bomb( ThrowBomb {
                                src: my_first.id,
                                dst: f.id,
                }));
            } else if let Some(f) = op_2_strongest {
                actions.push(Action::Bomb( ThrowBomb {
                                src: my_first.id,
                                dst: f.id,
                }));
            }
        }

    }

    if actions.is_empty() {
        actions.push(Action::Wait);
    }
    actions
}

fn read_loop(factory_count: i32) -> (HashSet<Factory>, HashSet<Troop>, HashSet<Bomb>) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let entity_count = parse_input!(input_line, i32); // the number of entities (e.g. factories and troops)

    let mut factories: HashSet<Factory> = HashSet::with_capacity(factory_count as usize);
    let mut troops: HashSet<Troop> = HashSet::with_capacity((entity_count - factory_count + 1) as usize);
    let mut bombs: HashSet<Bomb> = HashSet::with_capacity(4);

    for i in 0..entity_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let entity_id = parse_input!(inputs[0], i32);
        let entity_type = inputs[1].trim().to_string();
        let arg_1 = parse_input!(inputs[2], i32);
        let arg_2 = parse_input!(inputs[3], i32);
        let arg_3 = parse_input!(inputs[4], i32);
        let arg_4 = parse_input!(inputs[5], i32);
        let arg_5 = parse_input!(inputs[6], i32);
        if entity_type == "FACTORY" {
            factories.insert(Factory {
                                id: entity_id,
                                owner: Owner::try_from(arg_1).unwrap(),
                                cyborgs: arg_2,
                                production: arg_3 });
        } else if entity_type == "TROOP" {
            troops.insert(Troop {
                            id: entity_id,
                            owner: Owner::try_from(arg_1).unwrap(),
                            src: arg_2,
                            dst: arg_3,
                            cyborgs: arg_4,
                            turns_left: arg_5 });
        } else if entity_type == "BOMB" {
            bombs.insert(Bomb {
                            id: entity_id,
                            owner: Owner::try_from(arg_1).unwrap(),
                            src: arg_2,
                            dst: arg_3,
                            turns_left: arg_4});
        } else {
            print_err!("Unkown entity type: {}", entity_type);
        }
    }
    (factories, troops, bombs)
}

fn read_init() -> (i32, i32, HashMap<(i32, i32), i32>) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let factory_count = parse_input!(input_line, i32); // the number of factories
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let link_count = parse_input!(input_line, i32); // the number of links between factories

    let mut links: HashMap<(i32, i32), i32> = HashMap::with_capacity(link_count as usize);

    for i in 0..link_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let factory_1 = parse_input!(inputs[0], i32);
        let factory_2 = parse_input!(inputs[1], i32);
        let distance = parse_input!(inputs[2], i32);
        links.insert((factory_1, factory_2), distance);
        links.insert((factory_2, factory_1), distance);
    }
    (factory_count, link_count, links)
}
