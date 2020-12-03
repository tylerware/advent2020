use std::str::FromStr;
use std::string::ParseError;

pub struct Slope {
    pub down: usize,
    pub right: usize
}


pub struct TreeLine {
    pub trees: Vec<usize>,
    pub line: Vec<bool>
}

impl FromStr for TreeLine {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line: Vec<bool> = s.chars()
                                   .map(|x| x == '#')
                                   .collect();

        let mut trees = Vec::<usize>::new();
        for (i,is_tree) in line.iter().enumerate() {
            if *is_tree {
                trees.push(i + 1)
            }
        }

        Ok(TreeLine {
            line: line,
            trees: trees
        })
    }
}

pub struct TreeMap {
    pub tree_lines: Vec<TreeLine>,
    pattern_width: usize,
    map_height: usize,
}

impl TreeMap {
    pub fn new(tree_lines: Vec<TreeLine>) -> Self {
        let width = tree_lines.get(0).unwrap().line.len();
        let height = tree_lines.len();

        Self {
            tree_lines: tree_lines,
            pattern_width: width,
            map_height: height,
        }
    }

    // Assumes that the slope is reduced (relatively prime)
    pub fn count_trees_on_slope(&self, slope: Slope) -> usize {
        let lines_to_check = self.map_height / slope.down;
        let mut tree_count = 0;
        for i in 0..lines_to_check {
            let y = i * slope.down;
            let tree_line = &self.tree_lines.get(y).unwrap().line;
            let x = (i * slope.right) % self.pattern_width;
            let has_tree = tree_line.get(x).unwrap();
            if *has_tree {
                tree_count += 1;
            }
        }

        tree_count
    }
}
