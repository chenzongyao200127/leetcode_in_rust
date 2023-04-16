// 1157_Online_Majority_Element_In_Subarray
// https://leetcode.cn/problems/online-majority-element-in-subarray/


/// 设计一个数据结构，有效地找到给定子数组的 多数元素
/// 子数组的 多数元素 是在子数组中出现 threshold 次数或次数以上的元素
/// 摩尔投票 + 线段树
use std::collections::{HashMap};
struct MajorityChecker {
    t:HashMap<i32,Vec<i32>>,
    a:HashMap<i32,i32>,
    s:HashMap<i32,Vec<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MajorityChecker {

    fn new(arr: Vec<i32>) -> Self {
        let mut t=HashMap::new();
        let a:HashMap<i32,i32>=arr.iter().enumerate().map(|(i,&v)|(i as i32,v)).collect();
        fn build(i:i32,l:i32,r:i32,a:&HashMap<i32,i32>,t:&mut HashMap<i32,Vec<i32>>){
            if l==r{
                t.insert(i,vec![*a.get(&l).unwrap_or(&0),1]);
                return 
            }
            let mid=(l+r)/2;
            build(i*2,l,mid,a,t);
            build(i*2+1,mid+1,r,a,t);
            let v=if let (Some(a),Some(b))=(t.get(&(i*2)),t.get(&(i*2+1))){
if a[0]==b[0]{
                    vec![a[0],a[1]+b[1]]
                }else if a[1]<b[1]{
                         vec![b[0],b[1]-a[1]]
                }else{
                     vec![a[0],a[1]-b[1]]
                }
            }else{vec![0;2]};
            t.insert(i,v);
        }
        build(1,0,arr.len() as i32-1,&a,&mut t);
        let mut s=HashMap::new();
        for (i,&v) in arr.iter().enumerate(){
            s.entry(v).or_insert(Vec::new()).push(i as i32);
        }
        Self{t,a,s}
    }
    
    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
          fn ask(i:i32,l:i32,r:i32,ll:i32,rr:i32,t:&HashMap<i32,Vec<i32>>)->Vec<i32>{
            if l==ll && rr==r{
                return t[&i].clone()
            }
            let mid=(l+r)/2;
            if rr<=mid{
                ask(i*2,l,mid,ll,rr,t)
            }else if ll>mid{
 ask(i*2+1,mid+1,r,ll,rr,t)
            }else{
               let (a,b) = (ask(i*2,l,mid,ll,mid,t), ask(i*2+1,mid+1,r,mid+1,rr,t));
                if a[0]==b[0]{
                    vec![a[0],a[1]+b[1]]
                }else if a[1]<b[1]{
                         vec![b[0],b[1]-a[1]]
                }else{
                     vec![a[0],a[1]-b[1]]
                }
            }
            
        }
        let mut ans=ask(1,0,self.a.len() as i32-1,left,right,&self.t)[0];
        if (self.s[&ans].partition_point(|&v| v<=right) as i32)-(self.s[&ans].partition_point(|&v| v<left) as i32)<threshold{
           -1
        }else{
            ans
        }
    }
}

/**
 * Your MajorityChecker object will be instantiated and called as such:
 * let obj = MajorityChecker::new(arr);
 * let ret_1: i32 = obj.query(left, right, threshold);
 */


//  I will explain the code line by line:

//  use std::collections::{HashMap};: Importing the HashMap type from the std::collections module.
 
//  struct MajorityChecker {: Definition of the MajorityChecker struct.
 
//  t: HashMap<i32,Vec<i32>>, a: HashMap<i32,i32>, s: HashMap<i32,Vec<i32>>: Declaration of the fields t, a, and s. t is a segment tree, a maps indices to values in the input array, and s maps values to their indices in the input array.
 
//  impl MajorityChecker {: Implementing methods for the MajorityChecker struct.
 
//  fn new(arr: Vec<i32>) -> Self {: Definition of the new constructor method that takes a Vec<i32> as input and returns a new MajorityChecker object.
 
//  let mut t=HashMap::new();: Initializing an empty HashMap for the t field.
 
//  let a:HashMap<i32,i32>=arr.iter().enumerate().map(|(i,&v)|(i as i32,v)).collect();: Creating the a field by mapping indices to the corresponding values in the input array.
 
//  fn build(i:i32,l:i32,r:i32,a:&HashMap<i32,i32>,t:&mut HashMap<i32,Vec<i32>>){: Definition of a helper function build that constructs the segment tree t. It takes the index i, the left and right boundaries l and r, and mutable references to the a and t fields.
 
//  10-20. The build function is a recursive function that constructs the segment tree. It first checks for the base case when l == r (line 11). If true, it inserts the value at index l and its count (1) into t (line 12-13) and returns (line 14). If not, it calculates the mid index (line 15) and calls itself recursively for the left and right children of the current node (lines 16-17). It then computes the aggregate information for the current node based on its children and inserts it into t (lines 18-22).
 
//  build(1,0,arr.len() as i32-1,&a,&mut t);: Calling the build function to construct the segment tree t.
 
//  let mut s=HashMap::new();: Initializing an empty HashMap for the s field.
 
//  23-25. Populating the s field by iterating over the input array, and for each value v, adding its index to the corresponding Vec<i32> in s.
 
//  Self{t,a,s}: Returning the constructed MajorityChecker object.
 
//  fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {: Definition of the query method that takes left, right, and threshold as input and returns an i32.
 
//  fn ask(i:i32,l:i32,r:i32,ll:i32,rr:i32,t:&HashMap<i32,Vec<i32>>)->Vec<i32>{: Definition of a helper function ask that recursively queries the segment tree. It takes the index i, the left and right boundaries l and r, the query boundaries ll and rr, and a reference to the t field.
 
//  30-44. The ask function works similarly to the build function but for querying the segment tree. It first checks if the current node's range matches the queried range (line 31). If true, it returns the aggregate information for that node (line 32). If not, it calculates the mid index (line 33) and calls itself recursively for the left and right children, depending on the queried range (lines 34-42). It then computes the aggregate information for the queried range based on the children's results (lines 43-44).
 
//  let mut ans=ask(1,0,self.a.len() as i32-1,left,right,&self.t)[0];: Calling the ask function to find the majority element in the queried range.
//  46-48. Using the partition_point method on the s field to find the count of the majority element in the queried range. If the count is less than the threshold, it returns -1 (line 47); otherwise, it returns the majority element (line 49).
 
//  51-53. Commented example code showing how to instantiate and use a MajorityChecker object.