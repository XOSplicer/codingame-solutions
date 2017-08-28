use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::str::Chars;

macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            writeln!(&mut ::std::io::stderr(), $($arg)*).ok();
        }
    )
}

#[derive(Debug, Clone)]
struct RootNode {
    childs: HashMap<char, Box<Node>>
}

impl RootNode {

    fn new() -> Self {
        RootNode {
            childs: HashMap::new()
        }
    }

    fn count_nodes(&self) -> u32 {
        self.childs.iter().map(|(_, n)| n.count_nodes()).sum::<u32>()
    }

    fn insert_seq(&mut self, seq: Chars) {
        let mut orig = seq.clone();
        let mut iter = seq.peekable();
        let curr = iter.next();
        if curr == None {
            return
        }
        let c = curr.unwrap();
        let entry = self.childs.entry(c)
                        .or_insert(Box::new(Node::new(c)));
        if iter.peek() == None {
            entry.make_end();
        } else {
            let _ = orig.next();
            entry.insert_seq(orig);
        }
    }

}

#[derive(Debug, Clone)]
struct Node {
    digit: char,
    childs: HashMap<char, Box<Node>>,
    isEnd: bool,
}

impl Node {

    fn new(digit: char) -> Self {
        Node {
            digit: digit,
            childs: HashMap::new(),
            isEnd: false,
        }
    }

    fn count_nodes(&self) -> u32 {
        self.childs.iter().map(|(_, n)| n.count_nodes()).sum::<u32>() + 1
    }

    fn make_end(&mut self) {
        self.isEnd = true;
    }

    fn insert_seq(&mut self, seq: Chars) {
        let mut orig = seq.clone();
        let mut iter = seq.peekable();
        let curr = iter.next();
        if curr == None {
            return
        }
        let c = curr.unwrap();
        let entry = self.childs.entry(c)
                        .or_insert(Box::new(Node::new(c)));
        if iter.peek() == None {
            entry.make_end();
        } else {
            let _ = orig.next();
            entry.insert_seq(orig);
        }
    }

}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|s| s.unwrap());
    let N = lines.next().unwrap().parse::<u32>().unwrap();
    //let numbers = lines.cloned().collect::<Vec<_>>();
    //print_err!("{}, {:?}", N, numbers);
    let mut root = RootNode::new();
    for line in lines {
        root.insert_seq(line.chars());
    }
    let nodes = root.count_nodes();
    println!("{}", nodes);
}
