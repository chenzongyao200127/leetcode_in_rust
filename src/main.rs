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
                self.ans = self.stamps.clone();
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

    // init
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