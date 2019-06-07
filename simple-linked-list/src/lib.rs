struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self{
            head:None
        }
    }

    pub fn len(&self) -> usize {
        let mut length = 0;
        let mut n = &self.head;
        while let Some(boxed_node) = n {
            length += 1;
            n = &boxed_node.next;
        }

        length
    }

    // not requested, but here is a recursive version also
    pub fn len2(&self) -> usize {
        fn rec_len<T>(node:&Option<Box<Node<T>>>) -> usize {
            match node {
                Some(boxed_node) => 1 + rec_len(&boxed_node.next),
                None => 0
            }
        }

        rec_len(&self.head)
    }

    pub fn push(&mut self, data: T) {
        self.head = Some(Box::new(Node{
            data,
            next: self.head.take()
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        return match self.head.take() {
            Some(n) => {
                self.head = n.next;
                Some(n.data)
            },
            None => None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        return match &self.head {
            Some(b) => Some(&b.data),
            None => None
        };
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut list:SimpleLinkedList<T> = SimpleLinkedList::new();
        let mut n = &self.head;
        while let Some(boxed_node) = n {
            list.push(boxed_node.data.clone());
            n = &boxed_node.next;
        }
        list
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        // the classic, iterative version
        /*let mut list:SimpleLinkedList<T> = SimpleLinkedList::new();
        item.iter().for_each(|i: &T| {
            list.push(i.clone());
        });
        list*/
        item.iter().fold(SimpleLinkedList::new(), |mut list, v|{
            list.push(v.clone());
            list
        })
    }
}

impl<T:Clone> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut v: Vec<T> = Vec::new();
        let mut n = &self.rev().head;
        while let Some(boxed_node) = n {
            v.push(boxed_node.data.clone());
            n = &boxed_node.next;
        }
        v
    }
}
