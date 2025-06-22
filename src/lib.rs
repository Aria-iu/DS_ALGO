#![allow(unused)]
mod basic;
use basic::*;

mod sort;
use sort::*;

mod graph;
use graph::*;

mod backtrace;
use backtrace::*;

mod greedy;
use greedy::*;

mod dynamic_programming;
use dynamic_programming::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn queue_test() {
        basic();
        iter();
        let names = vec!["Mon", "Tom", "Kew", "Lisa", "Marry", "Bob"];
        let survivor = queue::hot_potato(names, 8);
        println!("The survival person is {survivor}");

        fn basic() {
            let mut q: Queue<i32> = Queue::new(5);
            let _r1 = q.enqueue(1);
            let _r2 = q.enqueue(2);
            let _r3 = q.enqueue(3);
            let _r4 = q.enqueue(4);
            if let Err(error) = q.enqueue(5) {
                println!("Enqueue error: {error}");
            }
            if let Some(data) = q.dequeue() {
                println!("dequeue data: {data}");
            } else {
                println!("empty queue");
            }

            println!("empty: {}, len: {}", q.is_empty(), q.len());

            println!("full: {}", q.is_full());

            println!("q: {:?}", q);

            q.clear();

            println!("{:?}", q);
        }

        fn iter() {
            let mut q: Queue<i32> = Queue::new(4);
            let _r1 = q.enqueue(1);
            let _r2 = q.enqueue(2);
            let _r3 = q.enqueue(3);
            let _r4 = q.enqueue(4);
            let sum1 = q.iter().sum::<i32>();
            let mut addend = 0;
            for item in q.iter_mut() {
                *item += 1;
                addend += 1;
            }
            let sum2 = q.iter().sum::<i32>();
            println!("{sum1} + {addend} = {sum2}");
            println!("sum = {}", q.into_iter().sum::<i32>());
        }
    }

    #[test]
    fn deque_test() {
        basic();
        iter();

        let pal = "rustsur";
        let is_pal = deque::palindrome_checker(pal);
        println!("{pal} is palindrome string: {is_pal}");

        let pal = "panda";
        let is_pal = deque::palindrome_checker(pal);
        println!("{pal} is palindrome string: {is_pal}");

        fn basic() {
            let mut d = Deque::new(4);
            let _r1 = d.add_front(1);
            let _r2 = d.add_front(2);
            let _r3 = d.add_rear(3);
            let _r4 = d.add_rear(4);
            if let Err(error) = d.add_front(5) {
                println!("add_front error: {error}");
            }
            println!("{:?}", d);

            match d.remove_rear() {
                Some(data) => println!("remove rear data {data}"),
                None => println!("empty deque"),
            }
            match d.remove_front() {
                Some(data) => println!("remove front data {data}"),
                None => println!("empty deque"),
            }
            println!("empty: {}, len: {}", d.is_empty(), d.len());
            println!("full: {}, {:?}", d.is_full(), d);

            d.clear();
            println!("{:?}", d);
        }

        fn iter() {
            let mut d = Deque::new(4);
            let _r1 = d.add_front(1);
            let _r2 = d.add_front(2);
            let _r3 = d.add_rear(3);
            let _r4 = d.add_rear(4);

            let sum1 = d.iter().sum::<i32>();
            let mut addend = 0;
            for item in d.iter_mut() {
                *item += 1;
                addend += 1;
            }
            let sum2 = d.iter().sum::<i32>();
            println!("{sum1} + {addend} = {sum2}");

            assert_eq!(14, d.into_iter().sum::<i32>());
        }
    }

    #[test]
    fn link_list_test() {
        basic_test();
        into_iter_test();
        iter_test();
        iter_mut_test();

        fn basic_test() {
            let mut list = List::new();
            list.push(1);
            list.push(2);
            list.push(3);

            assert_eq!(list.len(), 3);
            assert_eq!(list.is_empty(), false);
            assert_eq!(list.pop(), Some(3));
            assert_eq!(list.peek(), Some(&2));
            assert_eq!(list.peek_mut(), Some(&mut 2));

            list.peek_mut().map(|val| {
                *val = 4;
            });
            assert_eq!(list.peek(), Some(&4));

            list.clear();
            println!("basics test Ok!");
        }

        fn into_iter_test() {
            let mut list = List::new();
            list.push(1);
            list.push(2);
            list.push(3);

            let mut iter = list.into_iter();
            assert_eq!(iter.next(), Some(3));
            assert_eq!(iter.next(), Some(2));
            assert_eq!(iter.next(), Some(1));
            assert_eq!(iter.next(), None);
            println!("into_iter test Ok!");
        }

        fn iter_test() {
            let mut list = List::new();
            list.push(1);
            list.push(2);
            list.push(3);

            let mut iter = list.iter();
            assert_eq!(iter.next(), Some(&3));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next(), None);
            println!("iter test Ok!");
        }

        fn iter_mut_test() {
            let mut list = List::new();
            list.push(1);
            list.push(2);
            list.push(3);

            let mut iter = list.iter_mut();
            assert_eq!(iter.next(), Some(&mut 3));
            assert_eq!(iter.next(), Some(&mut 2));
            assert_eq!(iter.next(), Some(&mut 1));
            assert_eq!(iter.next(), None);
            println!("iter_mut test Ok!");
        }
    }

    #[test]
    fn vec_test() {
        basic();
        iter();

        fn basic() {
            let mut lvec1: LVec<i32> = LVec::new();
            lvec1.push(10);
            lvec1.push(11);
            lvec1.push(12);
            lvec1.push(13);
            lvec1.insert(0, 9);

            lvec1.print_lvec();

            let mut lvec2: LVec<i32> = LVec::new();
            lvec2.insert(0, 8);
            lvec2.append(&mut lvec1);

            println!("len: {}", lvec2.len());
            println!("pop {:?}", lvec2.pop().unwrap());
            println!("remove {:?}", lvec2.remove(0).unwrap());

            lvec2.print_lvec();
            lvec2.clear();
            lvec2.print_lvec();
        }

        fn iter() {
            let mut lvec: LVec<i32> = LVec::new();
            lvec.push(10);
            lvec.push(11);
            lvec.push(12);
            lvec.push(13);

            let sum1 = lvec.iter().sum::<i32>();
            let mut addend = 0;
            for item in lvec.iter_mut() {
                *item += 1;
                addend += 1;
            }
            let sum2 = lvec.iter().sum::<i32>();
            println!("{sum1} + {addend} = {sum2}");

            assert_eq!(50, lvec.into_iter().sum::<i32>());
        }
    }

    #[test]
    fn num2str_test() {
        let num = 100;
        use num2str::num2str_rec;
        let sb = num2str_rec(num, 2); // sb = str_binary
        let so = num2str_rec(num, 8); // so = str_octal
        let sh = num2str_rec(num, 16); // sh = str_hexdecimal
        println!("{num} = b{sb}, o{so}, x{sh}");

        let num = 1000;
        let so = num2str_rec(num, 8);
        let sh = num2str_rec(num, 16);
        println!("{num} = o{so}, x{sh}");
    }

    #[test]
    fn hanoi_test() {
        use basic::hanoi::*;
        hanoi(1, "A", "B", "C");
        println!("\n");
        hanoi(2, "A", "B", "C");
        println!("\n");
        hanoi(3, "A", "B", "C");
        println!("\n");
        hanoi(4, "A", "B", "C");
        println!("\n");
        hanoi(5, "A", "B", "C");
        println!("\n");
        hanoi(6, "A", "B", "C");
    }
}
