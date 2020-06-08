use super::weighted_union_find::*;

pub struct Percolation {
    n: u32,
    uf_grid: WeightedUnionFind,
    uf_fullness: WeightedUnionFind,
    open_sites: u32,
    is_open_arr: Box<[bool]>,
}

impl Percolation {
    pub fn new(n: u32) -> Self {
        if n < 1 {
            panic!("n too small");
        }
        return Self {
            uf_grid: WeightedUnionFind::new((n * n) + 2),
            uf_fullness: WeightedUnionFind::new((n * n) + 1),
            n,
            open_sites: 0,
            is_open_arr: (0..(n * n))
                .into_iter()
                .map(|i: u32| -> bool {
                    return false;
                })
                .collect(),
        };
    }

    pub fn get_n(&self) -> u32 {
        return self.n;
    }

    fn validate_inputs(&self, row: u32, col: u32) {
        if row > self.n || col > self.n {
            panic!("Illegal arguments, too big");
        }
        if row < 1 || col < 1 {
            panic!("Illegal arguments, too small");
        }
    }

    fn get_entry_pseudo_element_index(&self) -> u32 {
        return self.n * self.n;
    }

    fn get_exit_pseudo_element_index(&self) -> u32 {
        return (self.n * self.n) + 1;
    }

    fn convert_indice(&self, row: u32, col: u32) -> u32 {
        self.validate_inputs(row, col);
        return (row - 1) * self.n + col - 1;
    }

    // opens the site (row, col) if it is not open already
    pub fn open(&mut self, row: u32, col: u32) {
        self.validate_inputs(row, col);
        if self.is_open_arr[self.convert_indice(row, col) as usize] {
            return;
        }
        if col < self.n && self.is_open(row, col + 1) {
            self.uf_grid.union(
                self.convert_indice(row, col),
                self.convert_indice(row, col + 1),
            );
            self.uf_fullness.union(
                self.convert_indice(row, col),
                self.convert_indice(row, col + 1),
            );
        }
        if col > 1 && self.is_open(row, col - 1) {
            self.uf_grid.union(
                self.convert_indice(row, col),
                self.convert_indice(row, col - 1),
            );
            self.uf_fullness.union(
                self.convert_indice(row, col),
                self.convert_indice(row, col - 1),
            );
        }
        if row > 1 && self.is_open(row - 1, col) {
            self.uf_grid.union(
                self.convert_indice(row - 1, col),
                self.convert_indice(row, col),
            );
            self.uf_fullness.union(
                self.convert_indice(row - 1, col),
                self.convert_indice(row, col),
            );
        }
        if row < self.n && self.is_open(row + 1, col) {
            self.uf_grid.union(
                self.convert_indice(row + 1, col),
                self.convert_indice(row, col),
            );
            self.uf_fullness.union(
                self.convert_indice(row + 1, col),
                self.convert_indice(row, col),
            );
        }
        if row == 1 {
            self.uf_grid.union(
                self.get_entry_pseudo_element_index(),
                self.convert_indice(row, col),
            );
            self.uf_fullness.union(
                self.get_entry_pseudo_element_index(),
                self.convert_indice(row, col),
            );
        }
        if row == self.n {
            self.uf_grid.union(
                self.get_exit_pseudo_element_index(),
                self.convert_indice(row, col),
            );
        }
        self.is_open_arr[self.convert_indice(row, col) as usize] = true;
        self.open_sites += 1;
    }

    // is the site (row, col) open?
    pub fn is_open(&self, row: u32, col: u32) -> bool {
        self.validate_inputs(row, col);
        return self.is_open_arr[self.convert_indice(row, col) as usize];
    }

    // is the site (row, col) full?
    pub fn is_full(&mut self, row: u32, col: u32) -> bool {
        self.validate_inputs(row, col);
        if !self.is_open_arr[self.convert_indice(row, col) as usize] {
            return false;
        }
        return self.uf_fullness.find(self.convert_indice(row, col))
            == self.uf_fullness.find(self.get_entry_pseudo_element_index());
    }

    // returns the number of open sites
    pub fn number_of_open_sites(&self) -> u32 {
        return self.open_sites;
    }

    // does the system percolate?
    pub fn percolates(&mut self) -> bool {
        return self.uf_grid.find(self.get_entry_pseudo_element_index())
            == self.uf_grid.find(self.get_exit_pseudo_element_index());
    }
}
