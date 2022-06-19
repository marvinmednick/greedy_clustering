 use std::collections::{HashMap};

#[derive(Debug)]
struct VertexGroup {
    hamming_code: u32,
    vertex_list: Vec<u32>,
    group_id:  u32,
    rank:  u32,
}

impl VertexGroup {

    // create a new vertex group
    pub fn new(id: u32, hamming_code: u32) -> Self {
        VertexGroup {
            hamming_code: hamming_code,
            vertex_list:  Vec::<u32>::new(),
            group_id: id,
            rank: 1
        }
    }

    // adds a new vertex Id to an exist VertexGroup
    pub fn add(&mut self, id: u32) {
        self.vertex_list.push(id);
    }

    pub fn update_rank(&mut self, new_rank: u32) {
        self.rank = new_rank;
    }

}

pub struct HammingClusteringInfo {
    // hamming clusters map a hamming code to vertex info,
    // which is list of vertexes that belong in the same group or cluster
    // Note vertexes with the same hamming code will start out in the
    // the same cluster;  later clusters will be merged
    // based on their hamming code distance
    hamming_clusters : HashMap<u32,VertexGroup>,
    // vertex maps a vertex_id to its hamming code
    vertex_map : HashMap<u32,u32>,

    //count of number of groups
    groups: usize,
}


impl HammingClusteringInfo {

    pub fn new() -> Self { 
        HammingClusteringInfo { 
            hamming_clusters: HashMap::<u32,VertexGroup>::new(),
            vertex_map: HashMap::<u32,u32>::new(),
            groups: 0 
        }
    }

    // Adding a vertex requires
    // Adding it to the vertex tables which maps the vertex to its hamming code 
    // Adding it to the hamming table which is orginized by hamming code 
    //    which entails either adding a new entry, or updating an existing one with the new vertex
    //    id
    pub fn add_vertex(&mut self,vertex_id: u32,hamming_code: u32)  {
        if !self.vertex_map.contains_key(&vertex_id) {
            self.vertex_map.insert(vertex_id,hamming_code);
            if !self.hamming_clusters.contains_key(&hamming_code) {
                let _err = self.hamming_clusters.insert(hamming_code,VertexGroup::new(vertex_id,hamming_code));
                self.groups += 1;
            }
            else {
                let mut vg = self.hamming_clusters.get_mut(&hamming_code).unwrap();
                vg.add(vertex_id);
            }
        }
        else { 
            println!("vertex {} already exists...",vertex_id)
        }
    }


    pub fn get_rank(&self,hamming_code: u32) -> u32 {
        self.hamming_clusters[&hamming_code].rank
    }


    pub fn incr_rank(&self,hamming_code: u32) {
        let mut cluster_info = self.hamming_clusters.get_mut(&hamming_code).unwrap();
        cluster_info.rank += 1;
    }

    pub fn update_group(&self,hamming_code: u32, new_group: u32) {
        let mut cluster_info = self.hamming_clusters.get_mut(&hamming_code).unwrap();
        cluster_info.group_id = new_group;
    }


    //  cluster 
    //
    //  going to find and combine all the groups
    //  whose hamming distance is either 1 or 2  
    pub fn cluster(&mut self, k : usize) {

    }

    // recursively traverse the tree until it finds the top 
    // as a result, each member of the tree visisted will also be updated 
    // shortining the number of checks as time proceeds
    pub fn find_grouping(&mut self, hamming_code: u32) -> u32 {

        // remove the current entry from the list as we need to modify it
        // (it will be re-inserted later)
        let mut current_vertex_group = self.hamming_clusters.remove(&hamming_code).unwrap();

        // get a copy of the current group id
        let mut current_group = current_vertex_group.group_id.clone();
        
        // check if this node is at the the top of the tree
        if current_group != current_vertex_group.hamming_code {
            // if not, set the group to the grouping of my parent
            current_group = self.find_grouping(current_group);
        }
        // update the group and then re-insert into the hashmap
        current_vertex_group.group_id = current_group;
        self.hamming_clusters.insert(hamming_code,current_vertex_group);
        current_group

    }

    pub fn same_group(&mut self, code1: u32, code2: u32) -> bool {
        let group1 = self.find_grouping(code1).clone();
        let group2 = self.find_grouping(code2).clone();
        group1 == group2
    }

    pub fn union(&mut self, code1: u32, code2: u32) {
        let group1 = self.find_grouping(code1).clone();
        let group2 = self.find_grouping(code2).clone();
        let rank1 = self.get_rank(code1).clone();
        let rank2 = self.get_rank(code2).clone();

        println!("Union of {} and {} - Groups are {} and {}",code1,code2,group1,group2);
        if group1 == group2 {
            //Nothing to do, already are in the same group
        }
        else if rank1 > rank2 {
            self.update_group(code1,code2);
            // update the number of remaining groups
            self.groups -= 1;
            println!("New group for {} is {}",group2, group1);
        }
        else if rank1 < rank2 {
            self.update_group(code2,code1);
            self.groups -= 1;
            println!("New group for {} is {}",group1, group2);
        }
        else {
            // code1 and code2 rank are the same...  picking code 1 as the collection to add to
    
            //update the head vertex of code2 group(which is the group number) group of group1
            self.update_group(code2,code1);
            self.incr_rank(code1);
            println!("New group for {} is {}",group2, group1);
        }

    }

}

