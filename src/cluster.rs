use std::collections::{HashMap};
use log::{ info, error, debug, warn,trace };

#[derive(Debug)]
struct VertexInfo {
    vertex_id: u32,
    group_id:  u32,
    rank:  u32,
}

impl VertexInfo {

    pub fn new(id: u32) -> Self {
        VertexInfo {
            vertex_id: id,
            group_id: id,
            rank: 1
        }
    }

}

#[derive(Debug,PartialOrd,PartialEq,Ord,Eq)]
struct Edge {
    //Note that this currenly relies on default sort based on first value (weight)
    // TOOD:  Implement PartialOrd and PartialEq to specfically use the weight field
    weight: i32,
    start_id: u32,
    end_id: u32,
}

impl Edge {

    pub fn new(start: u32, end: u32, weight: i32) -> Self {
        Edge { start_id: start, end_id: end, weight: weight }
    }
    // TODO implement specific ordering

}

pub struct ClusteringInfo {
    vertex_map : HashMap<u32,VertexInfo>,
    edges : Vec<Edge>,
    groups: usize,
}


impl ClusteringInfo {

    pub fn new() -> Self { 
        ClusteringInfo { 
            vertex_map: HashMap::<u32,VertexInfo>::new(),
            edges : Vec::<Edge>::new(),
            groups: 0 
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.vertex_map.len(),self.edges.len())
    }

    pub fn add_vertex(&mut self,vertex_id: u32)  {
        if ! self.vertex_map.contains_key(&vertex_id) {
            let _err = self.vertex_map.insert(vertex_id,VertexInfo::new(vertex_id));
            self.groups += 1;
        }
    }

    /* cluster -- main entry point for dividing the entry points into k clusters.
     * Overall it will follow the same path as kruskals algorithm for MST
     * by attempting to the smallest avaialble edge that isn't already in the same 
     * cluster (Analagous to not taking an edge that creates a cycle when building an MST
     * The code does make use of a Union-Find data structure to allow the lookup of whether
     * an vertex is already in the same cluster.)
     */
    pub fn cluster(&mut self, k : usize) -> i32 {
        // edges are sorted so we will address them in increasing order
        // this allows us to always take the smallest avaiable edge
        self.edges.sort();

        for edge in &self.edges {
            debug!("Edge: {:?}",edge);
        }
        let mut cur_distance = 0;
        let num_edges = self.edges.len();

        // loop through each of the edges (which are in increasing weight) 
        // and add the smallest edge to its appropriate cluster
        for index in 0..num_edges {
            // since the edges are sorted in increasing order, then
            // the distance to the cluster will be the length of the current edge
            cur_distance = self.edges[index].weight.clone();
            debug!("#{} groups {} dist {} ",index, self.groups, cur_distance);
            // find the two vertexes at the ends this edge (start, end)
            let start = self.edges[index].start_id.clone();
            let end = self.edges[index].end_id.clone();

            //provided we haven't already reached our desired number of clusters, 
            // try to combine the start and end vertexes into the same cluster
            // (union will detect if they are already in the same cluster)
            if self.groups > k {
                self.union(start, end);
            }
            // we've reached the desireed number of groups and we have the next edge
            // which is potentialy the mininum distance between two clusters -- assuming
            // that edge is between two vertexes in different clusters -- however since 
            // we haven't checked it yet, both ends of this edge could be in the same cluster
            // and then isn't a valid distance between two clusters.   
            // So we need to to continue sarching through edges until we find the first one
            // where the two vertexes are in differnt clusters
            else if !self.same_group(start,end) {
                break
            }

        }
        cur_distance

    }

    pub fn add_edge(&mut self, start: u32, end: u32, weight: i32 )  {
        self.add_vertex(start);
        self.add_vertex(end);
        let edge = Edge::new(start,end,weight);
        self.edges.push(edge);
    }

    // recursively traverse the tree until it finds the top 
    // as a result, each member of the tree visisted will also be updated 
    pub fn find_grouping(&mut self, vertex_id: u32) -> u32 {

        let mut current_vertex = self.vertex_map.remove(&vertex_id).unwrap();
//        let mut current = self.vertex_map.get_mut(&vertex_id).unwrap();
        let mut current_group = current_vertex.group_id.clone();
        // check if this node is at the the top of the tree
        if current_group != current_vertex.vertex_id {
            // if not, set the group to the grouping of my parent
            current_group = self.find_grouping(current_group);
        }
        current_vertex.group_id = current_group;
        self.vertex_map.insert(vertex_id,current_vertex);
        current_group

    }

    pub fn same_group(&mut self, node1: u32, node2: u32) -> bool {
        let group1 = self.find_grouping(node1).clone();
        let group2 = self.find_grouping(node2).clone();
        group1 == group2
    }

    pub fn union(&mut self, node1: u32, node2: u32) {
        let group1 = self.find_grouping(node1).clone();
        let group2 = self.find_grouping(node2).clone();

        debug!("Union of {} and {} - Groups are {} and {}",node1,node2,group1,group2);
        if group1 == group2 {
            //Nothing to do, already are in the same group
        }
        else if self.vertex_map[&node1].rank > self.vertex_map[&node2].rank {
            let mut v_info = self.vertex_map.get_mut(&group2).unwrap();
            v_info.group_id = group1;
            // update the number of remaining groups
            self.groups -= 1;
            debug!("New group for {} is {}",group2, group1);
        }
        else if self.vertex_map[&node1].rank < self.vertex_map[&node2].rank {
            // node2 rank must be higher than node1's
            let mut v_info = self.vertex_map.get_mut(&group1).unwrap();
            v_info.group_id = group2;
            // update the number of remaining groups
            self.groups -= 1;
            debug!("New group for {} is {}",group1, group2);
        }
        else {
            // node1 and node2 rank are the same...  picking node 1 as the collection to add to
    
            //update the head vertex of node2 group(which is the group number) group of group1
            let mut v_info = self.vertex_map.get_mut(&group2).unwrap();
            v_info.group_id = group1;
            // since the two groups were the same size, the rank of the lead of the remaining
            // groupsing must increase by one
            v_info.rank += 1;
            // update the number of remaining groups
            self.groups -= 1;
            debug!("New group for {} is {}",group2, group1);
        }

    }

}

