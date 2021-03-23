
pub struct DisjointSet {
    forest:Vec<i32>
}

impl DisjointSet {
    pub fn new() -> Self {
        Self {
            forest: Vec::new()
        }
    }

    pub fn with_size(size: usize) -> Self {
        Self {
            forest: vec![-1; size]
        }
    }

    pub fn same_set(&mut self, object1: usize, object2: usize) -> bool {
        self.find_set(object1) == self.find_set(object2)
    }

    pub fn find_set(&mut self, object: usize) -> usize {
        //if this object is a member of a set
        if self.forest[object] > -1 {
            //find that set, and compress the paths so we don't have to follow all of them next time
            self.forest[object] = self.find_set(self.forest[object] as usize) as i32;
            self.forest[object] as usize
        } else {
            //this is the set
            object
        }
    }

    pub fn try_union(&mut self, object1: usize, object2: usize) -> bool {
        let set1 = self.find_set(object1);
        let set2 = self.find_set(object2);
        if set1 != set2 {
            self.union(set1, set2);
            true
        } else {
            false
        }
    }

    fn union(&mut self, set1: usize, set2: usize) {
        //if set1 has more members (members are negative) than set2, set1 becomes the parent
        //reducing the number of hops for find() later
        if self.forest[set1] < self.forest[set2] {
            self.forest[set1] += self.forest[set2];
            self.forest[set2] = set1 as i32;
        } else {
            self.forest[set2] += self.forest[set1];
            self.forest[set1] = set2 as i32;
        }
    }

    pub fn size(&mut self, set: usize) -> i32 {
        let set = self.find_set(set);
        -self.forest[set]
    }

    pub fn len(& self) -> usize {
        self.forest.len()
    }
}

#[cfg(test)]
mod tests {
    use super::DisjointSet;

    #[test]
    fn new() {
        let mut d = DisjointSet::with_size(10);
        for i in 0..d.len() {
            assert_eq!(d.find_set(i), i);
            assert_eq!(d.size(i), 1)
        }
    }

    #[test]
    fn union() {
        let mut d = DisjointSet::with_size(10);
        for i in 1..6 {
            assert!(d.try_union(i, i - 1));
        }
        for i in 7..10 {
            assert!(d.try_union(i, i - 1));
        }
        assert_eq!(d.size(0), 6);
        assert_eq!(d.size(9), 4);
        assert!(d.try_union(3, 8));
        for i in 0..d.len() {
            assert_eq!(d.size(i), 10);
        }
    }
}