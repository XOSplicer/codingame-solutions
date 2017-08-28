use std::io;
use std::io::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;
use std::iter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Node
}

impl Cell {
    fn try_from_char(c: char) -> Result<Cell, ()> {
        match c {
            '.' => Ok(Cell::Empty),
            '0' => Ok(Cell::Node),
            _   => Err(())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    x: u32,
    y: u32,
    right: Option<Rc<RefCell<Node>>>,
    bottom: Option<Rc<RefCell<Node>>>
}

impl Node {
    fn new(x: u32, y: u32) -> Node {
        Node {
            x: x,
            y: y,
            right: None,
            bottom: None
        }
    }

    fn get_x(&self) -> u32 {
        self.x
    }

    fn get_y(&self) -> u32 {
        self.y
    }

    fn set_right(&mut self, right: Option<Rc<RefCell<Node>>>) {
        self.right = right;
    }

    fn set_bottom(&mut self, bottom: Option<Rc<RefCell<Node>>>) {
        self.bottom = bottom;
    }

}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x2 = self.right.clone().map_or(-1, |n| n.borrow().get_x() as i32);
        let y2 = self.right.clone().map_or(-1, |n| n.borrow().get_y() as i32);
        let x3 = self.bottom.clone().map_or(-1, |n| n.borrow().get_x() as i32);
        let y3 = self.bottom.clone().map_or(-1, |n| n.borrow().get_y() as i32);
        write!(f, "{} {} {} {} {} {}", self.x, self.y, x2, y2, x3, y3)
    }
}


macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            writeln!(&mut ::std::io::stderr(), $($arg)*).ok();
        }
    )
}

fn main() {

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());
    let width: usize  = lines.next().unwrap().parse().unwrap();
    let heigth: usize = lines.next().unwrap().parse().unwrap();
    print_err!("width: {:?}", &width);
    print_err!("height: {:?}", &heigth);
    let cell_lines: Vec<String> = lines.take(heigth).collect();
//    print_err!("cell lines: {:?}", &cell_lines);
    let cells: Vec<Vec<Cell>> = cell_lines.iter()
                                        .map(|x| x.chars()
                                                .map(|c| Cell::try_from_char(c).unwrap())
                                                .collect())
                                        .collect();
    let mut nodes: Vec<Rc<RefCell<Node>>> = Vec::new(); // only one dim, links in structs
    for c in cells.iter().enumerate().flat_map(|(i, v)| iter::repeat(i).enumerate().zip(v.iter())) {
//        print_err!("Cell to Node: {:?}", &c);
        if let((x, y), &Cell::Node) =  c {
            nodes.push(Rc::new(RefCell::new(Node::new(x as u32, y as u32))));
        }
    }

    //right neighbour
    for n in nodes.iter() {
        let neighbour_right = nodes.iter()
                                .filter(|v| v.borrow().get_y() == n.borrow().get_y()
                                        && v.borrow().get_x() > n.borrow().get_x())
                                .min_by_key(|v| v.borrow().get_x())
                                .map(|v| v.clone());
//        print_err!("neighbour_right {:?} for {:?}", &neighbour_right, &*n.borrow());
        n.borrow_mut().set_right(neighbour_right);
    }

    //bottom neighbour
    for n in nodes.iter() {
        let neighbour_bottom = nodes.iter()
                                .filter(|v| v.borrow().get_x() == n.borrow().get_x()
                                        && v.borrow().get_y() > n.borrow().get_y())
                                .min_by_key(|v| v.borrow().get_y())
                                .map(|v| v.clone());
//        print_err!("neighbour_bottom {:?} for {:?}", &neighbour_bottom, &*n.borrow());
        n.borrow_mut().set_bottom(neighbour_bottom);
    }

    //debug it
/*    for i in nodes.iter() {
        print_err!("Node: {:?}", &*i.borrow());
    }
*/
    // print it
    // Three coordinates: a node, its right neighbor, its bottom neighbor
    for i in nodes.iter() {
        println!("{}", &*i.borrow());
    }


}
