use super::weighted_union_find::*;

pub struct Percolation {
    uf_grid: WeightedUnionFind,
    uf_fullness: WeightedUnionFind,
    open_sites: u32,
    is_open_arr: Box<[bool]>,
    rows: u32,
    cols: u32,
}

impl Percolation {
    pub fn new(rows: u32, cols: u32) -> Self {
        if rows < 1 || cols < 1 {
            panic!("n too small");
        }
        return Self {
            uf_grid: WeightedUnionFind::new((rows * cols) + 2),
            uf_fullness: WeightedUnionFind::new((rows * cols) + 1),
            rows,
            cols,
            open_sites: 0,
            is_open_arr: (0..(rows * cols))
                .into_iter()
                .map(|_i: u32| -> bool {
                    return false;
                })
                .collect(),
        };
    }

    pub fn get_rows(&self) -> u32 {
        return self.rows;
    }

    pub fn get_cols(&self) -> u32 {
        return self.cols;
    }

    fn get_entry_pseudo_element_index(&self) -> u32 {
        return self.rows * self.cols;
    }

    fn get_exit_pseudo_element_index(&self) -> u32 {
        return (self.rows * self.cols) + 1;
    }

    fn convert_indice(&self, row: u32, col: u32) -> u32 {
        return (row - 1) * self.cols + col - 1;
    }

    // opens the site (row, col) if it is not open already
    pub fn open(&mut self, row: u32, col: u32) {
        if self.is_open_arr[self.convert_indice(row, col) as usize] {
            return;
        }
        if col < self.cols && self.is_open(row, col + 1) {
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
        if row < self.rows && self.is_open(row + 1, col) {
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
        if row == self.rows {
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
        return self.is_open_arr[self.convert_indice(row, col) as usize];
    }

    // is the site (row, col) full?
    pub fn is_full(&mut self, row: u32, col: u32) -> bool {
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
