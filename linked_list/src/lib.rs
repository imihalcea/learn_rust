struct LinkedList<T> {
    head:Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T>{
        LinkedList{
            head:None,
        }
    }

    pub fn push(&mut self, element:T){
        let old_head = std::mem::replace(&mut self.head, None);
        let node = Box::new(Node::new(element, old_head));
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T>{
        self.head.take().map(|n|{
            self.head = n.next;
            n.element
        })
    }

    pub fn peek(&mut self) -> Option<&T>{
        self.head.as_ref().map(|n|&n.element)
    }
}


#[derive(PartialEq, Debug)]
struct Node<T> {
    element:T,
    next:Option<Box<Node<T>>>
}

type Link<T> = Option<Box<Node<T>>>;


impl<T> Node<T> {
    pub fn new(element:T, next:Option<Box<Node<T>>>) -> Node<T>{
        Node{
            element,
            next
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepend_tests() {
        let mut list = LinkedList::new();
        list.push(32);
        list.push(42);
        if let Link::Some(n) = list.head{
            assert_eq!(n.element, 42);
            match n.next {
                None =>  panic!("test failed"),
                Some(next) => assert_eq!(next.element, 32),
            }

        }else {
            panic!("test failed");
        }
    }

    #[test]
    fn pop_tests() {
        let mut list = LinkedList::new();
        list.push(32);
        list.push(42);

        assert_eq!(list.pop(),Some(42));
        if let Link::Some( n) = &list.head {
            assert_eq!(n.element, 32);
        }
        else {
            panic!("test failed");
        }

        assert_eq!(list.pop(),Some(32));
        assert_eq!(list.head, None)
    }

    #[test]
    fn peek_tests() {
        let mut list = LinkedList::new();
        list.push(32);
        list.push(42);

        assert_eq!(list.peek(),Some(&42));
        if let Link::Some( n) = &list.head {
            assert_eq!(n.element, 42);
        }
        else {
            panic!("test failed");
        }
    }
}
