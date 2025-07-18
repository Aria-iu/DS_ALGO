type Adj = [Vec<usize>];

const IN_DECOMPOSITION: u64 = 1 << 63;

pub struct CentroidDecomposition {
    /// The root of the centroid tree, should _not_ be set by the user
    pub root: usize,
    /// The result. decomposition[`v`] is the parent of `v` in centroid tree.
    /// decomposition[`root`] is 0
    pub decomposition: Vec<usize>,
    /// Used internally to save the big_child of a vertex, and whether it has
    /// been added to the centroid tree.
    vert_state: Vec<u64>,
    /// Used internally to save the subtree size of a vertex
    vert_size: Vec<usize>,
}

impl CentroidDecomposition {
    pub fn new(mut num_vertices: usize) -> Self {
        num_vertices += 1;
        CentroidDecomposition {
            root: 0,
            decomposition: vec![0; num_vertices],
            vert_state: vec![0; num_vertices],
            vert_size: vec![0; num_vertices],
        }
    }
    #[inline]
    fn put_in_decomposition(&mut self, v: usize, parent: usize) {
        self.decomposition[v] = parent;
        self.vert_state[v] |= IN_DECOMPOSITION;
    }
    #[inline]
    fn is_in_decomposition(&self, v: usize) -> bool {
        (self.vert_state[v] & IN_DECOMPOSITION) != 0
    }
    fn dfs_size(&mut self, v: usize, parent: usize, adj: &Adj) -> usize {
        self.vert_size[v] = 1;
        let mut big_child = 0_usize;
        let mut bc_size = 0_usize; // big child size
        for &u in adj[v].iter() {
            if u == parent || self.is_in_decomposition(u) {
                continue;
            }
            let u_size = self.dfs_size(u, v, adj);
            self.vert_size[v] += u_size;
            if u_size > bc_size {
                big_child = u;
                bc_size = u_size;
            }
        }
        self.vert_state[v] = big_child as u64;
        self.vert_size[v]
    }
    fn dfs_centroid(&self, v: usize, size_thr: usize) -> usize {
        // recurse until big child's size is <= `size_thr`
        match self.vert_state[v] as usize {
            u if self.vert_size[u] <= size_thr => v,
            u => self.dfs_centroid(u, size_thr),
        }
    }
    fn decompose_subtree(
        &mut self,
        v: usize,
        centroid_parent: usize,
        calculate_vert_size: bool,
        adj: &Adj,
    ) -> usize {
        // `calculate_vert_size` determines if it is necessary to recalculate
        // `self.vert_size`
        if calculate_vert_size {
            self.dfs_size(v, centroid_parent, adj);
        }
        let v_size = self.vert_size[v];
        let centroid = self.dfs_centroid(v, v_size >> 1);
        self.put_in_decomposition(centroid, centroid_parent);
        for &u in adj[centroid].iter() {
            if self.is_in_decomposition(u) {
                continue;
            }
            self.decompose_subtree(
                u,
                centroid,
                self.vert_size[u] > self.vert_size[centroid],
                adj,
            );
        }
        centroid
    }
    pub fn decompose_tree(&mut self, adj: &Adj) {
        self.decompose_subtree(1, 0, true, adj);
    }
}
