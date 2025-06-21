#[derive(Debug)]
pub struct Deque<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Deque<T> {
    pub fn new(size: usize) -> Self {
        Self {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    pub fn is_empty(&self) -> bool {
        // self.data.is_empty()
        // 0 == Self::len(&self)
        0usize == self.len()
    }

    pub fn is_full(&self) -> bool {
        self.cap == self.len()
    }

    pub fn len(&self) -> usize {
        self.data.len() as usize
    }

    pub fn clear(&mut self) {
        self.data = Vec::with_capacity(self.cap)
    }

    pub fn add_front(&mut self, val: T) -> Result<(), String> {
        if self.len() == self.cap {
            return Err("No space avaliable".to_string());
        } else {
            self.data.push(val);
            Ok(())
        }
    }

    pub fn add_rear(&mut self, val: T) -> Result<(), String> {
        if self.len() == self.cap {
            return Err("No space avaliable".to_string());
        } else {
            self.data.insert(0, val);
            Ok(())
        }
    }

    pub fn remove_front(&mut self) -> Option<T> {
        if self.len() > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    pub fn remove_rear(&mut self) -> Option<T> {
        if self.len() > 0 {
            Some(self.data.remove(0))
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

pub struct IntoIter<T>(Deque<T>);
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

// 回文检查
pub fn palindrome_checker(pal: &str) -> bool {
    // 数据入队列
    let mut d = Deque::new(pal.len());
    for c in pal.chars() {
        let _r = d.add_rear(c);
    }

    let mut is_pal = true;
    while d.len() > 1 && is_pal {
        // 出队首尾字符
        let head = d.remove_front();
        let tail = d.remove_rear();

        // 比较首尾字符, 若不同则非回文
        if head != tail {
            is_pal = false;
        }
    }

    is_pal
}
