#[derive(Debug)]
pub struct Queue<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new(size: usize) -> Self {
        Self {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    pub fn is_empty(&self) -> bool {
        // self.data.is_empty()
        0 == Self::len(&self)
    }

    pub fn is_full(&self) -> bool {
        self.cap == self.data.len()
    }

    pub fn len(&self) -> usize {
        self.data.len() as usize
    }

    pub fn clear(&mut self) {
        self.data = Vec::with_capacity(self.cap)
    }

    pub fn enqueue(&mut self, val: T) -> Result<(), String> {
        if self.len() == self.cap {
            return Err("No space available".to_string());
        } else {
            self.data.insert(0, val);
            Ok(())
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.len() > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }

        iterator
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        let mut iteartor = IterMut { stack: Vec::new() };
        for item in self.data.iter_mut() {
            iteartor.stack.push(item)
        }

        iteartor
    }
}

pub struct IntoIter<T>(Queue<T>);
// impl<T: Clone> Iterator for IntoIter<T>{
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            Some(self.0.data.remove(0))
        } else {
            None
        }
    }
}

pub struct Iter<'a, T> {
    stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if 0 != self.stack.len() {
            Some(self.stack.remove(0))
        } else {
            None
        }
    }
}

pub struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if 0 != self.stack.len() {
            Some(self.stack.remove(0))
        } else {
            None
        }
    }
}

pub fn hot_potato(names: Vec<&str>, num: usize) -> &str {
    let mut q = Queue::new(names.len());
    for iter in names {
        q.enqueue(iter);
    }

    while q.len() > 1 {
        for _i in 0..num {
            let q1 = q.dequeue().unwrap();
            q.enqueue(q1);
        }

        let _rm = q.dequeue();
    }

    q.dequeue().unwrap()
}
