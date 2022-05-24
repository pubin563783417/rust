pub struct List<T> {
    head : Next<T>,
}

type Next<T> = Option<Box<Node<T>>>;
struct Node<T> {
    val : T ,
    next : Next<T> ,
}

impl <T>  List< T> {
    pub fn new() -> Self {
        List {head : None}
    }

    pub fn push(& mut self ,val : T) {
        let new_node = Some(Box::new(Node {val , next : self.head.take()}));
        self.head = new_node;
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(
            |x| {
                self.head = x.next;
                x.val
            }
        )
    }
}

impl <T : Copy> From<&[T]> for List<T> {
    fn from(arr: &[T]) -> Self {
        let mut list = List::new();
        arr.iter().rev().for_each(
            |x | {list.push(*x);}
        );
        return list;
    }
}

