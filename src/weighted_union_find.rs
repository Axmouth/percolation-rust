pub struct WeightedUnionFind {
    id: Box<[u32]>,
    sz: Box<[u32]>,
}

impl WeightedUnionFind {
    pub fn new(n: u32) -> Self {
        if n < 1 {
            panic!("n too small");
        }
        return Self {
            id: (0..n).collect(),
            sz: (0..n)
                .into_iter()
                .map(|_i: u32| -> u32 {
                    return 1;
                })
                .collect(),
        };
    }

    pub fn find(&mut self, i: u32) -> u32 {
        return self.root(i);
    }

    fn root(&mut self, i: u32) -> u32 {
        let mut i = i;
        while i != self.id[i as usize] {
            self.id[i as usize] = self.id[self.id[i as usize] as usize];
            i = self.id[i as usize];
        }
        return i;
    }

    pub fn connected(&mut self, p: u32, q: u32) -> bool {
        return self.root(p) == self.root(q);
    }

    pub fn union(&mut self, p: u32, q: u32) {
        let i: u32 = self.root(p);
        let j: u32 = self.root(q);
        if i == j {
            return;
        }
        if self.sz[i as usize] < self.sz[j as usize] {
            self.id[i as usize] = j;
            self.sz[j as usize] += self.sz[i as usize];
        } else {
            self.id[j as usize] = i;
            self.sz[i as usize] += self.sz[j as usize];
        }
    }
}
