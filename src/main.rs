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

fn main() {
    println!("Hello, world!")
}
