// 连续邮资问题
// 某国发行了 k 种不同面值的邮票
// 并规定每封信最多允许贴 h 张邮票
// 在这些约束下，为了能贴出 {1，2，3，... ，maxvalue} 连续整数集合的所有邮资，并使 maxvalue 的值最大
// 应该如何设计各邮票的面值？

// 例如，当 k=5、h=4 时，面值设计为{1，3，11，15，32}，可使 max_value 达到最大值70
// 或者说，用这些面值的1至4张邮票可以表示不超过70的所有邮资，但无法表示邮资71
// 而用其他面值的1至4张邮票如果可以表示不超过n的所有邮资，必有n<=70

const MAX_NM: usize = 10;
const MAX_POSTAGE: usize = 1024;
const INF: i32 = 2147483647;

#[derive(Debug)]
struct State {
    k: usize,
    n: usize,
    stamps: [i32; MAX_NM],
    ans: [i32; MAX_NM],
    y: [i32; MAX_POSTAGE],
    max_stamp: i32,
    r: i32,
}

impl State {
    fn new(kinds: usize, num: usize) -> Self {
        State {
            k: kinds,
            n: num,
            stamps: [0; MAX_NM],
            ans: [0; MAX_NM],
            y: [0; MAX_POSTAGE],
            max_stamp: 0,
            r: num as i32,
        }
    }

    fn backtrack(&mut self, i: usize) {
        let backup_y = self.y.clone();
        let backup_r = self.r;

        if i >= self.k {
            if self.r > self.max_stamp {
                self.max_stamp = self.r;
                for tmp in 0..self.k {
                    self.ans[tmp] = self.stamps[tmp];
                }
            }
            return;
        }

        for next_stamp_value in self.stamps[i - 1] + 1..=self.r + 1 {
            // update stamps[i]
            self.stamps[i] = next_stamp_value;

            // update y
            // ATTENTATION: for postage in 0..=self.r as i32  is Wrong
            for postage in 0..self.stamps[i-1] * self.n as i32 {
                if self.y[postage as usize] >= self.n as i32 {
                    continue;
                }

                for num in 1..=self.n - self.y[postage as usize] as usize {
                    let new_postage = postage + num as i32 * next_stamp_value;

                    if new_postage < MAX_POSTAGE as i32
                        && self.y[postage as usize] + (num as i32) < self.y[new_postage as usize]
                    {
                        self.y[new_postage as usize] = self.y[postage as usize] + num as i32;
                    }
                }
            }

            // update r
            while self.y[(self.r + 1) as usize] < INF {
                self.r += 1;
            }

            self.backtrack(i + 1);

            // restore
            self.r = backup_r;
            self.y = backup_y;
        }
    }
}

fn main() {
    let (kinds, nums) = (5, 4);
    let mut state = State::new(kinds, nums);

    state.stamps[0] = 1;

    for i in 0..=state.r as usize {
        state.y[i] = i as i32;
    }

    for i in (state.r + 1) as usize..MAX_POSTAGE {
        state.y[i] = INF;
    }

    state.backtrack(1);

    println!("max stamp is: {}", state.max_stamp);
    for i in 0..state.k {
        print!("{:4}", state.ans[i]);
    }
}