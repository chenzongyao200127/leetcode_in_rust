// 1017. 二进制转换
pub fn base_2(n: i32) -> String {
    let mut res = String::new();
    let mut n = n;
    while n > 0 {
        res.push_str(&(n % 2).to_string());
        n /= 2;
    }
    res.chars().rev().collect()
}

pub fn base_neg2(n: i32) -> String {
    let n_base2 = base_2(n);
    let mut res = String::new();
    let len = n_base2.len();
    let mut carry = 0;
    println!("{:?}", n_base2);
    for i in (0..len).rev() {
        println!("carry: {:?}", carry);
        let mut num = n_base2.chars().nth(i).unwrap().to_digit(10).unwrap() + carry;
        println!("num {:?}", num);
        if (len - i) % 2 == 1 {
            num += 2;
        }
        carry = num / 2;
        res.push_str(&(num % 2).to_string());
    }
    if carry > 0 {
        res.push_str(&carry.to_string());
    }
    res.chars().rev().collect()
}

// 1329_将矩阵按对角线排序
pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut mat = mat;
    let m = mat.len();
    let n = mat[0].len();

    for i in 0..m {
        let mut temp = Vec::new();
        let mut x = i;
        let mut y = 0;
        while x < m && y < n {
            temp.push(mat[x][y]);
            x += 1;
            y += 1;
        }
        temp.sort();
        let mut x = i;
        let mut y = 0;
        let mut index = 0;
        while x < m && y < n {
            mat[x][y] = temp[index];
            x += 1;
            y += 1;
            index += 1;
        }
    }

    for i in 1..n {
        let mut temp = Vec::new();
        let mut x = 0;
        let mut y = i;
        while x < m && y < n {
            temp.push(mat[x][y]);
            x += 1;
            y += 1;
        }
        temp.sort();
        let mut x = 0;
        let mut y = i;
        let mut index = 0;
        while x < m && y < n {
            mat[x][y] = temp[index];
            x += 1;
            y += 1;
            index += 1;
        }
    }
    mat
}

/**
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */

fn main() {
    let res = base_neg2(3);
    println!("{:?}", res);
}
