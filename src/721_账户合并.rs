// 721_账户合并
// https://leetcode.cn/problems/accounts-merge/description/

use std::collections::HashMap;

// union find
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let size = vec![1; n];
        UnionFind { parent, size }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            if self.size[root_x] < self.size[root_y] {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            } else {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
        }
    }
}

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut email_to_id = HashMap::new();
        let mut email_to_name = HashMap::new();
        let mut id = 0;
        for account in accounts.iter() {
            let name = &account[0];
            for i in 1..account.len() {
                let email = &account[i];
                if !email_to_id.contains_key(email) {
                    email_to_id.insert(email.to_string(), id);
                    email_to_name.insert(email.to_string(), name.to_string());
                    id += 1;
                }
            }
        }

        let mut uf = UnionFind::new(id);
        for account in accounts.iter() {
            let first_email = &account[1];
            let first_id = *email_to_id.get(first_email).unwrap();
            for i in 2..account.len() {
                let next_email = &account[i];
                let next_id = *email_to_id.get(next_email).unwrap();
                uf.union(first_id, next_id);
            }
        }

        let mut id_to_email = HashMap::new();
        for (email, &id) in email_to_id.iter() {
            let root_id = uf.find(id);
            id_to_email.entry(root_id).or_insert(Vec::new()).push(email);
        }

        let mut ans = Vec::new();
        for (_, emails) in id_to_email {
            let mut account = Vec::new();
            let name = email_to_name.get(emails[0]).unwrap();
            account.push(name.to_string());
            let mut emails = emails.iter().map(|s| s.to_string()).collect::<Vec<_>>();
            emails.sort_unstable();
            account.append(&mut emails);
            ans.push(account);
        }

        ans
    }
}
