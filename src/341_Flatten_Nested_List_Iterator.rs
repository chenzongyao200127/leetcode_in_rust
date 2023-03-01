// https://leetcode.cn/problems/flatten-nested-list-iterator/

// 给你一个嵌套的整数列表 nestedList 。
// 每个元素要么是一个整数，要么是一个列表；该列表的元素也可能是整数或者是其他列表。
// 请你实现一个迭代器将其扁平化，使之能够遍历这个列表中的所有整数。

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>)
}


// 思路2 调用`hesNext()`或者`next()`方法的时候扁平化当前的嵌套的子列表
// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }
struct NestedIterator {
    revNetedList: Vec<NestedInteger>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {

    fn new(nestedList: Vec<NestedInteger>) -> Self {
        let mut rev_list = vec![];
        nestedList.into_iter().rev().for_each(|x| {
            rev_list.push(x);
        });

        NestedIterator {
            revNetedList: rev_list,
        }
    }
    
    fn next(&self) -> i32 {
        let cur = self.revNetedList.pop().unwrap();
        cur as i32
    }
    
    fn has_next(&self) -> bool {
        while (self.revNetedList.len() > 0) {
            let cur = self.revNetedList[self.revNetedList.len()-1];
            match cur {
                NestedInteger::Int(cur) => { return true; },
                NestedInteger::List(cur) => {
                    self.revNetedList.pop().unwrap();
                    cur.into_iter().rev().for_each(|x| {
                        self.revNetedList.push(x);
                    });
                }
            }
        }
        
        false
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

// /** 
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
// struct NestedIterator(Vec<i32>);

// impl NestedIterator {
//     fn new(nestedList: Vec<NestedInteger>) -> Self {
//         let mut v=collect(nestedList);
//         v.reverse();
//         Self(v)
//     }

//     #[inline(always)]
//     fn next(&mut self) -> i32 {
//         self.0.pop().unwrap()
//     }

//     #[inline(always)]
//     fn has_next(&self) -> bool {
//         self.0.len()!=0
//     }
//  }

// fn collect(nestedList: Vec<NestedInteger>)->Vec<i32>{
//     nestedList.into_iter().map(|x|match x{
//         NestedInteger::Int(x)=>vec![x],
//         NestedInteger::List(x)=>collect(x)
//     }).flatten().collect()
// }
//  作者：qweytr_1
//  链接：https://leetcode.cn/problems/flatten-nested-list-iterator/solution/zhi-jie-ya-ping-bu-hao-ma-by-qweytr_1-wdom/
//  来源：力扣（LeetCode）
//  著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

fn main() {
    let obj = NestedIterator::new(nestedList);
    let ret_1: i32 = obj.next();
    let ret_2: bool = obj.has_next();
 }


