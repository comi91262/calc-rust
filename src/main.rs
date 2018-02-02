use std::io::prelude::*;
use std::fs::File;
use std::collections::LinkedList;

type Parsed<A> = Option<(A, usize)>;

// N = {start, exp, sum, prod, val, num} 
// Σ = {(, ), + , - , * , / , %, [0-9]} 
// P = {
//   start = exp \n / ε
//   exp   = prod ((`+` / '-') prod)*,
//   prod  = val  ((`*` / `/` / '%') val)*,
//   val   = num / `(` exp `)`,
//   num   = [1-9][0-9]*,
// }


#[derive(PartialEq, Debug, Clone)]
enum Operator {
    Add, Sub, Prod, Div, Mod
}

#[derive(PartialEq, Debug)]
enum Syntree {
    Empty,
    B(u8),
    Both(Box<Syntree>, Box<Syntree>),
    List(LinkedList<Syntree>),
    N(u32),
    C(Operator, Box<Syntree>, Box<Syntree>)
}

#[derive(PartialEq, Debug)]
enum Result {
    NoValue,
    Value(u32)
}


use Syntree::*;

#[allow(unused_variables)]
fn scan_empty(index: usize, input: &[u8]) -> Parsed<Syntree> {
    Some((Empty, index))
}


fn scan_byte<F>(f: F, index: usize, input: &[u8]) -> Parsed<Syntree> 
where F: Fn(u8) -> bool {
    match input.get(index) {
        Some(c) => {
            if f(*c) {
                Some((B(*c), index + 1))
            } else {
                None
            }
        }
        None    => None
    }
}


#[allow(unused_variables)] //TODO あとで外す
fn scan_seq<F, G>(scan1: F, scan2: G, index: usize, input: &[u8]
                 ) -> Parsed<Syntree> 
                   where F: Fn(usize, &[u8]) -> Parsed<Syntree>, 
                         G: Fn(usize, &[u8]) -> Parsed<Syntree>
                 {
                     match scan1(index, &input) {
                         Some((c1, i1)) => 
                             match scan2(i1, &input) {
                                 Some((c2, i2)) => 
                                     Some((Both(Box::new(c1), Box::new(c2)), i2)),
                                 None           => None
                             }
                         None => None
                     }
                 }

fn scan_either<F, G>(scan1: F, scan2: G, index: usize, input: &[u8]
                 ) -> Parsed<Syntree> 
                   where F: Fn(usize, &[u8]) -> Parsed<Syntree>, 
                         G: Fn(usize, &[u8]) -> Parsed<Syntree>
                 {
                     if let Some((c1, i1)) = scan1(index, &input){
                         return Some((c1, i1));
                     } 
                     
                     if let Some((c2, i1)) = scan2(index, &input){
                         return Some((c2, i1));
                     } 

                     return None;
                 }

fn scan_repeat<F>(scan: F, i: usize, input: &[u8]) 
    -> Parsed<Syntree> 
    where F: Fn(usize, &[u8]) -> Parsed<Syntree> {
        match scan(i, input) {
            None => Some((Empty, i)),
            Some ((e, i2)) => {
                match scan_repeat(scan, i2, &input) {
                    Some((Empty, i)) => {
                        let mut li = LinkedList::new();
                        li.push_front(e);
                        Some((List(li), i))
                    }
                    Some((List(mut e1), i3)) => {
                        e1.push_front(e);
                        Some((List(e1), i3))
                    }
                    _ => panic!("error: scan_repeat")
                }
            }
        }
    }



fn scan_0(i: usize, input: &[u8]) -> Parsed<Syntree>{
    scan_byte (|b: u8| -> bool {b == '0' as u8}, i, input)
}
fn scan_1(i: usize, input: &[u8]) -> Parsed<Syntree>{
    scan_byte (|b: u8| -> bool {b == '1' as u8}, i, input)
}
fn scan_2(i: usize, input: &[u8]) -> Parsed<Syntree>{
    scan_byte (|b: u8| -> bool {b == '2' as u8}, i, input)
}
fn scan_3(i: usize, input: &[u8]) -> Parsed<Syntree>{
    scan_byte (|b: u8| -> bool {b == '3' as u8}, i, input)
}
fn scan_4(i: usize, input: &[u8]) -> Parsed<Syntree>{
    scan_byte (|b: u8| -> bool {b == '4' as u8}, i, input)
}
fn scan_5(i: usize, input: &[u8]) -> Parsed<Syntree>{
    scan_byte (|b: u8| -> bool {b == '5' as u8}, i, input)
}
fn scan_6(i: usize, input: &[u8]) -> Parsed<Syntree>{
    scan_byte (|b: u8| -> bool {b == '6' as u8}, i, input)
}
fn scan_7(i: usize, input: &[u8]) -> Parsed<Syntree>{
    scan_byte (|b: u8| -> bool {b == '7' as u8}, i, input)
}
fn scan_8(i: usize, input: &[u8]) -> Parsed<Syntree>{
    scan_byte (|b: u8| -> bool {b == '8' as u8}, i, input)
}
fn scan_9(i: usize, input: &[u8]) -> Parsed<Syntree>{
    scan_byte (|b: u8| -> bool {b == '9' as u8}, i, input)
}

fn scan_lparen(i: usize, input: &[u8]) -> Parsed<Syntree>{
    scan_byte (|b: u8| -> bool {b == '(' as u8}, i, input)
}

fn scan_rparen(i: usize, input: &[u8]) -> Parsed<Syntree>{
    scan_byte (|b: u8| -> bool {b == ')' as u8}, i, input)
}

fn scan_plus(i: usize, input: &[u8]) -> Parsed<Syntree>{
    scan_byte (|b: u8| -> bool {b == '+' as u8}, i, input)
}
fn scan_hyphen(i: usize, input: &[u8]) -> Parsed<Syntree>{
    scan_byte (|b: u8| -> bool {b == '-' as u8}, i, input)
}
fn scan_aster(i: usize, input: &[u8]) -> Parsed<Syntree>{
    scan_byte (|b: u8| -> bool {b == '*' as u8}, i, input)
}
fn scan_slash(i: usize, input: &[u8]) -> Parsed<Syntree>{
    scan_byte (|b: u8| -> bool {b == '/' as u8}, i, input)
}
fn scan_perc(i: usize, input: &[u8]) -> Parsed<Syntree>{
    scan_byte (|b: u8| -> bool {b == '%' as u8}, i, input)
}
//fn scan_lf(i: usize, input: &[u8]) -> Parsed<Syntree> {
//    scan_byte (|b: u8| -> bool {b == '\n' as u8}, i, input)
//}

fn scan_123456789(i: usize, input: &[u8]) -> Parsed<Syntree> {
    if let Some((r, i)) = scan_either(scan_1, scan_2, i, &input) {
        return Some((r, i));
    }
    if let Some((r, i)) = scan_either(scan_3, scan_4, i, &input) {
        return Some((r, i));
    }
    if let Some((r, i)) = scan_either(scan_5, scan_6, i, &input) {
        return Some((r, i));
    }
    if let Some((r, i)) = scan_either(scan_7, scan_8, i, &input) {
        return Some((r, i));
    }
    if let Some((r, i)) = scan_9(i, &input) {
        return Some((r, i));
    }
    return None;
}

fn scan_0123456789(i: usize, input: &[u8]) -> Parsed<Syntree> {
    if let Some((r, i)) = scan_either(scan_0, scan_123456789, i, &input) {
        return Some((r, i));
    }
    return None;
}

fn build_num(head: u8, tail: &mut LinkedList<Syntree>) -> Syntree {
    let mut n = 0;
    let mut m = 1;
    let base = 10;


    loop {
        match tail.pop_back() {
            Some(B(b)) => {
                n += m * (b - '0' as u8);
                m *= base;
            }
            Some(_) => panic!("error: build_num"),
            None => break
        }
    }

    n += m * (head - '0' as u8);
    N(n as u32)
}

//   num   = [1-9][0-9]*,
fn scan_num(i: usize, input: &[u8]) -> Parsed<Syntree>{
    match scan_123456789(i, &input) {
        Some((B(e), i1)) => {
            match scan_repeat(scan_0123456789, i1, &input) {
                Some((List(mut li), i2)) => {
                    Some((build_num(e, &mut li), i2))
                }
                Some((Empty, i2)) => {
                    Some((N((e - '0' as u8) as u32), i2))
                }
                _ => None
            }
        }
        Some(_) => panic!("error: scan_num"),
        None => None
    }
}


fn to_op(leaf: Syntree) -> Operator {
    match leaf {
        B(b) => {
            match b as u8 {
                43 => Operator::Add,  // '+' as u8
                45 => Operator::Sub,  // '-' as u8
                42 => Operator::Prod, // '*' as u8
                47 => Operator::Div,  // '/' as u8
                37 => Operator::Mod,  // '%' as u8
                _ => panic!("error: to_op")
            }
        }
        _ => panic!("error: to_op")
    }

}




fn to_tree(head: Syntree, tail: &mut LinkedList<Syntree>) -> Syntree {
    match tail.pop_front() {
        Some(h1) => {
            if let Both(e1, e2) = h1 {
                C(to_op(*e1), Box::new(to_tree(head, tail)), e2)
            } else {
                panic!("error: to_tree")
            }
        }
        None => head
    }
}


//   exp   = prod ((`+` / '-') prod)*,
fn scan_exp(i: usize, input: &[u8]) -> Parsed<Syntree> {
    let scan_op = |i: usize, input: &[u8]| -> Parsed<Syntree> {
        scan_either(scan_plus, scan_hyphen, i, input)
    };

    match scan_seq(scan_prod, 
                   |i: usize, input: &[u8]| -> Parsed<Syntree> {
                       scan_repeat(
                           |i: usize, input: &[u8]| -> Parsed<Syntree> {
                               scan_seq(&scan_op, scan_prod, i, input)
                           }
                           ,i , input)
                   }, i, input) {
        Some((Both(e1, e2),i1)) => {
            match *e2 {
                Empty => Some((*e1, i1)),
                List(mut li) => Some ((to_tree(*e1, &mut li), i1)),
                _ => {
                    println!("{:?}", *e1);
                    println!("{:?}", *e2);
                    panic!("error: scan_exp")
                }
            }
        }
        _ => None
    }
}

//   prod  = val  ((`*` / `/` / '%') val)*,
fn scan_prod(i: usize, input: &[u8]) -> Parsed<Syntree> {
    let scan_op = |i: usize, input: &[u8]| -> Parsed<Syntree> {
        scan_either(scan_aster,
                   |i: usize, input: &[u8]| -> Parsed<Syntree> {
                       scan_either(scan_slash, scan_perc, i, input)
                   }, i, input)
    };


    match scan_seq(scan_val, 
             |i: usize, input: &[u8]| -> Parsed<Syntree> {
                 scan_repeat(
                     |i: usize, input: &[u8]| -> Parsed<Syntree> {
                         scan_seq(&scan_op, scan_val, i, input)
                     }
                     ,i , input)
             }, i, input) {
        Some((Both(e1, e2),i1)) => {
            match *e2 {
                Empty => Some((*e1, i1)),
                List(mut li) => Some ((to_tree(*e1, &mut li), i1)),
                _ => {
                    panic!("error: scan_prod")
                }
            }
        }
        _ => None
    }
}

//   val   = num / `(` exp `)`,
fn scan_val(i: usize, input: &[u8]) -> Parsed<Syntree> {
    match scan_either(scan_num, 
                      |i: usize, input: &[u8]| -> Parsed<Syntree> {
                          scan_seq(scan_lparen,
                                   |i: usize, input: &[u8]| -> Parsed<Syntree> {
                                       scan_seq(scan_exp, scan_rparen, i, input)
                                   }, i, input)
                      }, i, input) {
        Some((N(n), i)) => Some((N(n), i)),
        Some((Both(_, e), i)) => {
            match *e {
                Both(e, _) => Some((*e, i)),
                _ => panic!("error: scan_val")
            }
        }
        _ => None
    }
}

//   start = exp / ε
fn scan_start(i: usize, input: &[u8]) -> Parsed<Syntree> {
    scan_either(scan_exp, scan_empty, i, input)
}

fn eval(tree: Syntree) -> Result {
    match tree {
        Empty => Result::NoValue,
        N(n)  => Result::Value(n),
        C(op, ltree, rtree) => {

            if let Result::Value(lvalue) = eval(*ltree) {
                if let Result::Value(rvalue) = eval(*rtree) {
                    match op {
                        Operator::Add  => Result::Value(lvalue + rvalue),
                        Operator::Sub  => Result::Value(lvalue - rvalue),
                        Operator::Prod => Result::Value(lvalue * rvalue),
                        Operator::Div  => Result::Value(lvalue / rvalue),
                        Operator::Mod  => Result::Value(lvalue % rvalue),
                    }
                } else {
                    Result::NoValue
                }
            } else {
                Result::NoValue
            }
        }
        _ => panic!("error: eval") 

    }

}

fn main() {
    let mut text = vec![];

    match File::open("hoge.txt") {
        Ok(mut t)  => {
            match t.read_to_end(&mut text) {
                Ok(_) => {
                    let input = text.clone();
                    println!("input: {}", String::from_utf8(input).unwrap());

                    //半角スペースを取り除く
                    text.retain(|&n| n != 32);

                    match scan_start(0, &text) {
                        Some((output, _)) => {
                            println!("output: {:?}", output);
                            println!("result: {:?}", eval(output));
                        }
                        None                  => panic!("error: main")
                    }
                }
                Err(error) => panic!("error: {}", error)
            }
        }
        Err(e) =>  panic!("error: {}", e)
    }

}



#[test]
fn test_scan_empty(){
    let v1 = vec![1];
    let v2 = vec![1];

    assert_eq!(scan_empty(0, &v1), Some((Empty, 0)));
}

#[test]
fn test_scan_byte(){
    let v1 = vec![1];

    let f = |b : u8| -> bool {false};
    assert_eq!(scan_byte(&f, 0, &v1), None);

    let v2 = vec![1,2];
    let g = |b : u8| -> bool {b == 1};
    assert_eq!(scan_byte(&g, 0, &v2), Some((B(1), 1)));
}

//#[test]
//fn test_scan_either(){
//    let v1 = vec!['1' as u8];
//    let v2 = vec!['2' as u8];
//    let v3 = vec!['3' as u8];
//
//    let f = scan_1;
//    let g = scan_2;
//
//    assert_eq!(scan_either(f, g, 0, &v1), Some(('1' as u8, 1)));
//    assert_eq!(scan_either(f, g, 0, &v2), Some(('2' as u8, 1)));
//    assert_eq!(scan_either(f, g, 0, &v3), None              );
//    
//}
//

#[test]
fn test_scan_repeat(){
    let v1 = vec!['1' as u8, '2' as u8, '3' as u8];
    let v2 = vec!['a' as u8];
    let v3 = vec!['3' as u8, 'a' as u8];
    let v4 = vec![];

    let f = scan_123456789;

    let mut a1 = LinkedList::new();
    a1.push_front(B('3' as u8));
    a1.push_front(B('2' as u8));
    a1.push_front(B('1' as u8));
    let mut a2 = LinkedList::new();
    a2.push_front(B('3' as u8));

    assert_eq!(scan_repeat(f, 0, &v1), Some((List(a1), 3)));
    assert_eq!(scan_repeat(f, 0, &v2), Some((Empty, 0)));
    assert_eq!(scan_repeat(f, 0, &v3), Some((List(a2), 1)));
    assert_eq!(scan_repeat(f, 0, &v4), Some((Empty, 0)));
}
//
//#[test]
//fn test_scan_1(){
//    let v = vec!['1' as u8 ,2,3];
//    assert_eq!(scan_1(0, &v), Some(('1' as u8, 1)));
//}
//
//#[test]
//fn test_scan_0123456789(){
//    let v = vec!['0' as u8, '1' as u8, '2' as u8, '3' as u8,
//                 '4' as u8, '5' as u8, '6' as u8, '7' as u8,
//                 '8' as u8, '9' as u8, 'a' as u8];
//
//    assert_eq!(scan_0123456789(0, &v), Some(('0' as u8, 1)));
//    assert_eq!(scan_0123456789(1, &v), Some(('1' as u8, 2)));
//    assert_eq!(scan_0123456789(2, &v), Some(('2' as u8, 3)));
//    assert_eq!(scan_0123456789(3, &v), Some(('3' as u8, 4)));
//    assert_eq!(scan_0123456789(4, &v), Some(('4' as u8, 5)));
//    assert_eq!(scan_0123456789(5, &v), Some(('5' as u8, 6)));
//    assert_eq!(scan_0123456789(6, &v), Some(('6' as u8, 7)));
//    assert_eq!(scan_0123456789(7, &v), Some(('7' as u8, 8)));
//    assert_eq!(scan_0123456789(8, &v), Some(('8' as u8, 9)));
//    assert_eq!(scan_0123456789(9, &v), Some(('9' as u8, 10)));
//    assert_eq!(scan_0123456789(10, &v),None); 
//}

#[test]
fn test_scan_num(){
    let v1 = vec!['0' as u8];
    let v2 = vec![];
    let v3 = vec!['1' as u8, '2' as u8, '3' as u8];
    let v4 = vec!['0' as u8, '2' as u8, '3' as u8];
    let v5 = vec!['0' as u8, '2' as u8, '3' as u8];
    let v6 = vec!['1' as u8, 'a' as u8, '3' as u8];


    assert_eq!(scan_num(0, &v1), None);
    assert_eq!(scan_num(0, &v2), None);
    assert_eq!(scan_num(0, &v3), Some((N(123), 3)));
    assert_eq!(scan_num(0, &v4), None);
    assert_eq!(scan_num(1, &v5), Some((N(23), 3)));
    assert_eq!(scan_num(0, &v6), Some((N(1), 1)));
    
}

#[test]
fn test_to_op(){
    let a1 = B('+' as u8);
    let a2 = B('-' as u8);
    let a3 = B('*' as u8);
    let a4 = B('/' as u8);
    let a5 = B('%' as u8);

    
    assert_eq!(to_op(a1), Operator::Add);
    assert_eq!(to_op(a2), Operator::Sub);
    assert_eq!(to_op(a3), Operator::Prod);
    assert_eq!(to_op(a4), Operator::Div);
    assert_eq!(to_op(a5), Operator::Mod);

}

#[test]
fn test_to_tree(){
    //let h1 = N(1);
    //let t1 = Empty;
    //let mut li = LinkedList::new();
    //l1.push_back(t1);

    let h2 = N(1);
    let t2 = Both(Box::new(B('+' as u8)), Box::new(N(1)));
    let mut l2 = LinkedList::new();
    l2.push_back(t2);
    
    let h3 = N(1);
    let t31 = Both(Box::new(B('+' as u8)), Box::new(N(1)));
    let t32 = Both(Box::new(B('+' as u8)), Box::new(N(1)));
    let mut l3 = LinkedList::new();
    l3.push_back(t31);
    l3.push_back(t32);

    //assert_eq!(to_tree(h1, l1), N(1));
    assert_eq!(to_tree(h2, &mut l2), C(Operator::Add, Box::new(N(1)), 
                                       Box::new(N(1))));
    assert_eq!(to_tree(h3, &mut l3), C(Operator::Add, Box::new(
                C(Operator::Add, Box::new(N(1)), Box::new(N(1)))), Box::new(N(1))));


}
