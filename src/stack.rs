
#[derive(Debug)]
struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
        } else{
            None
        }
    }
}

struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            size: 0,
            data: Vec::new()
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.data.clear();
        self.size = 0;
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            self.data.pop()
        }
    }

    fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.data.get(self.size - 1)
        }
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            self.data.get_mut(self.size - 1)
        }
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }
        iterator
    }
}

#[test]
fn test_stack_basic() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    println!("size:{}, {:?}", s.size, s);

    let pop = s.pop();
    println!("pop: {:?}", pop);
    println!("size:{}, {:?}", s.size, s);
    while let Some(pop) = s.pop() {
        println!("pop: {:?}", pop);
        println!("size:{}, {:?}", s.size, s);
    }
}