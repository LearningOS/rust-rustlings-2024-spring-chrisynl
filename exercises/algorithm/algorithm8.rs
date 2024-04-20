/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T: Copy> {
    elements: Vec<T>,
}

impl<T: Copy> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T: Copy> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T: Copy>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T: Copy> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        if self.q1.is_empty() {
            self.q2.enqueue(elem);
        } else {
            self.q1.enqueue(elem);
        }
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        if self.q1.is_empty() && self.q2.is_empty() {
            return Err("Stack is empty");
        }
        if self.q1.is_empty() {
            let n = self.q2.size();
            for _i in 0..(n - 1) {
                let val = *self.q2.peek().unwrap();
                self.q2.dequeue();
                self.q1.enqueue(val);
            }
            let val = *self.q2.peek().unwrap();
            self.q2.dequeue();
            return Ok(val);
        } else {
            let n = self.q1.size();
            for _i in 0..(n - 1) {
                let val = *self.q1.peek().unwrap();
                self.q1.dequeue();
                self.q2.enqueue(val);
            }
            let val = *self.q1.peek().unwrap();
            self.q1.dequeue();
            return Ok(val);
        }
    }
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}
