use std::collections::{HashMap};

#[derive(Debug)]
struct BitMaskingTable {
    bitmasks: Vec<u32>

}


impl BitMaskingTable {
    pub fn new(entries: u32) -> Self {
        let mut cur_mask = 0x1;
        let mut count = 0;
        let mut bt = BitMaskingTable { bitmasks:  Vec::<u32>::new() };

        // for each bitmask needed
        // add it to the vector and then shift it left by one
        while count < entries {
            bt.bitmasks.push(cur_mask);
            cur_mask =  cur_mask << 1;
            count +=1;
        }
        bt
    }

    pub fn get_mask(&self,entry: usize) -> u32 {
        if entry < self.bitmasks.len() {
            self.bitmasks.get(entry).unwrap().clone()
        }
        else {
            0
        }
    }

}
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
        let mut new_group = VertexGroup {
            hamming_code: hamming_code,
            vertex_list:  Vec::<u32>::new(),
            group_id: hamming_code,
            rank: 1
        };
        new_group.vertex_list.push(id);
        new_group
    }

    // adds a new vertex Id to an exist VertexGroup
    pub fn add(&mut self, id: u32) {
        self.vertex_list.push(id);
    }

    pub fn update_rank(&mut self, new_rank: u32) {
        self.rank = new_rank;
    }

    pub fn get_rank(&self) -> u32 {
        self.rank
    }

}

#[derive(Debug)]
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
                let vg = self.hamming_clusters.get_mut(&hamming_code).unwrap();
                vg.add(vertex_id);
            }
        }
        else { 
            println!("vertex {} already exists...",vertex_id)
        }
    }

    pub fn sizes(&self) -> (usize,usize,usize) {
        (self.vertex_map.len(),self.hamming_clusters.len(),self.groups)
    }


    pub fn get_rank(&self,hamming_code: u32) -> u32 {
        self.hamming_clusters[&hamming_code].get_rank()
    }


    pub fn incr_rank(&mut self,hamming_code: u32) {
        let mut cluster_info = self.hamming_clusters.get_mut(&hamming_code).unwrap();
        cluster_info.rank += 1;
    }

    pub fn update_group(&mut self,hamming_code: u32, new_group: u32) {
        let mut cluster_info = self.hamming_clusters.get_mut(&hamming_code).unwrap();
        cluster_info.group_id = new_group;
    }


    //  cluster 
    //
    //  going to find and combine all the groups
    //  whose hamming distance is either 1 or 2  
    pub fn cluster(&mut self, k : usize) {

    }

    pub fn find_group(&mut self,vertex: u32) -> Option<u32> {
        if let Some(hamming_code) = self.vertex_map.get(&vertex) {
            let hc = hamming_code.clone();
            Some(self.find_grouping(hc))
        }
        else {
            None
        }
    }
    
    // recursively traverse the tree until it finds the top 
    // as a result, each member of the tree visisted will also be updated 
    // shortining the number of checks as time proceeds
    pub fn find_grouping(&mut self, hamming_code: u32) -> u32 {

        // remove the current entry from the list as we need to modify it
        // (it will be re-inserted later)
        if let Some(mut current_vertex_group) = self.hamming_clusters.remove(&hamming_code) {

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
        else {
            println!("No Entry found for hamming code {}",hamming_code);
            0
        }
    

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
            println!("> New group for {} is {} - groups={}",group2, group1,self.groups);
        }
        else if rank1 < rank2 {
            self.update_group(code2,code1);
            self.groups -= 1;
            println!("> New group for {} is {} - groups={}",group1, group2,self.groups);
        }
        else {
            // code1 and code2 rank are the same...  picking code 1 as the collection to add to
    
            //update the head vertex of code2 group(which is the group number) group of group1
            self.update_group(code2,code1);
            self.incr_rank(code1);
            self.groups -= 1;
            println!("> New group for {} is {} - groups={}",group1, group2,self.groups);
        }

    }

}



/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;

    fn setup_basic() -> HammingClusteringInfo {
        let mut c = HammingClusteringInfo::new();
        c.add_vertex(1,0x00);
        c.add_vertex(2,0x01);
        c.add_vertex(3,0x02);
        println!("Initial Setup {:#?}",c);
        c

    }

    #[test]
    fn initial_setup_test() {
        let mut c = setup_basic();
        assert_eq!(c.sizes(),(3,3,3));
        assert_eq!(c.find_group(1),Some(0));
        assert_eq!(c.find_group(2),Some(1));
        assert_eq!(c.find_group(3),Some(2));
        assert_eq!(c.same_group(0,1),false);
        assert_eq!(c.same_group(0,2),false);
        assert_eq!(c.same_group(1,2),false);
        assert_eq!(c.get_rank(0),1);
        assert_eq!(c.get_rank(1),1);
        assert_eq!(c.get_rank(2),1);
        c.incr_rank(2);
        assert_eq!(c.get_rank(2),2);
    }

    #[test]
    fn vertex_test() {
        let mut v = VertexGroup::new(1,0xAAA);
        assert_eq!(v.get_rank(),1);
        v.update_rank(3);
        assert_eq!(v.get_rank(),3);
    }


    #[test]
    fn union_test() {
        let mut c = setup_basic();
        c.union(0,1);
        println!("After Union {:#?}",c);
        assert_eq!(c.find_grouping(1),0);
        assert_eq!(c.same_group(0,1),true);
        assert_eq!(c.get_rank(0),2);
        assert_eq!(c.sizes(),(3,3,2));
    }

    #[test]
    fn bitmask_table_test() {
        let b = BitMaskingTable::new(3);
        assert_eq!(b.get_mask(0),0b1);
        assert_eq!(b.get_mask(1), 0b10);
        assert_eq!(b.get_mask(2),0b100);
        assert_eq!(b.get_mask(3),0);
    }

}
