use std::collections::{HashMap,BTreeMap}; 
#[derive(Debug)]
struct BitMaskingTable {
    bitmasks: Vec<u32>

}

impl BitMaskingTable {
    pub fn new_one_bit(num_bits: u32) -> Self {
        let mut cur_mask = 0x1;
        let mut count = 0;
        let mut bt = BitMaskingTable { bitmasks:  Vec::<u32>::new() };

        // for each bitmask needed
        // add it to the vector and then shift it left by one
        while count < num_bits {
            bt.bitmasks.push(cur_mask);
            cur_mask =  cur_mask << 1;
            count +=1;
        }
        bt
    }

    pub fn new_two_bit(num_bits: u32) -> Self {
        let mut cur_first_bit_mask = 0x1;
        let mut first_bit_count = 0;
        let mut bt = BitMaskingTable { bitmasks:  Vec::<u32>::new() };

        // this set of loops will create all the two bit combintations
        // There will be Combination(n,2) entries
        // This will iterate through all the single bit options one by one
        // and then iterate through all the remaining options for a 2nd bit
        // Note that since the bits will be walking from right to left
        // (first bit 001, then 010, then 100) the inner loop only needs to look
        // at bits left of the starting bit, since bits to the right will already been 
        // covered by the previous starting bits (i.e.  011 will be found by
        // with starting bit 001 and therefore starting bit 010 doesn't need
        // to add it)  as a result, the inner loop only needs to loop for the 
        // number of bits remaining to the left of the starting bit
        while first_bit_count < num_bits {
            let mut cur_second_bit_mask = cur_first_bit_mask << 1;
            // num_bits -count is the remaining number of bits to process
            // we're going set the number of second bits for inner loop 
            // so that we process all the 2nd bit options remaining
            // one less than the remaining number of bits 
            let num_second_bits = num_bits - first_bit_count - 1;
            let mut second_bit_count = 0;
            while second_bit_count < num_second_bits {
                // combine the first and second bits together to create
                // a two bit mask and add it to the table
                let new_entry = cur_first_bit_mask | cur_second_bit_mask;
                bt.bitmasks.push(new_entry);
                //shift the seond bit to the left
                cur_second_bit_mask = cur_second_bit_mask << 1;
                second_bit_count += 1;
            }

            cur_first_bit_mask =  cur_first_bit_mask << 1;
            first_bit_count +=1;
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

    pub fn len(&self) -> usize {
        self.bitmasks.len()
    }


}
#[derive(Debug)]
struct VertexGroup {
    orig_hamming_code: u32,
    vertex_list: Vec<u32>,
    group_id:  u32,
    rank:  u32,
}

impl VertexGroup {

    // create a new vertex group
    pub fn new(id: u32, hamming_code: u32) -> Self {
        let mut new_group = VertexGroup {
            orig_hamming_code: hamming_code,
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
    hamming_clusters : BTreeMap<u32,VertexGroup>,
    // vertex maps a vertex_id to its hamming code
    vertex_map : HashMap<u32,u32>,

    //count of number of groups
    groups: usize,
    hamming_size: u32,
}


impl HammingClusteringInfo {

    pub fn new(hamming_size: u32) -> Self { 
        HammingClusteringInfo { 
            hamming_clusters: BTreeMap::<u32,VertexGroup>::new(),
            vertex_map: HashMap::<u32,u32>::new(),
            groups: 0,
            hamming_size: hamming_size,
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

    // moves all the vertex in the list from the 'from group' to the "to group"
    pub fn combine_groups(&mut self,from_group: u32, to_group: u32) {
        println!("Moving {} to {}",from_group,to_group);
        let mut from_vg = self.hamming_clusters.remove(&from_group).unwrap();
        let mut to_vg = self.hamming_clusters.remove(&to_group).unwrap();
        to_vg.vertex_list.append(&mut from_vg.vertex_list);
        from_vg.group_id = to_group;
        self.hamming_clusters.insert(from_group,from_vg);
        self.hamming_clusters.insert(to_group,to_vg);
    }

    // caclulate the spacing between two vertexes
    pub fn vertex_spacing(&self, id1: u32, id2:u32) -> Option<u32> {

        let result1 = self.vertex_map.get(&id1);
        let result2 = self.vertex_map.get(&id2);

        match (result1, result2) {
            (Some(code1), Some(code2)) =>  Some({ 
                // xor the values to find the bit differences
                let bit_diff = code1 ^ code2;
                // return the number of one bits in the result
                bit_diff.count_ones() }),
            _ => None
        }

    }



    pub fn vertex_cluster_spacing(&mut self, v1: u32, v2 :u32) -> Option<(u32,u32)>{
        let result1 = self.find_group(v1);
        let result2 = self.find_group(v2);

        match (result1, result2) {
            (Some(group1),Some(group2)) => self.hamming_cluster_spacing(group1,group2),
            _ => return None,
        }

    }


    // caclulate the min and max cluster spacing between two groups
    // The cluster spacings and the min and max spacing between any two vertexes
    // in the cluster
    pub fn hamming_cluster_spacing(&self, group1: u32, group2 :u32) -> Option<(u32,u32)>{
        let vg1 = self.hamming_clusters.get(&group1);
        let vg2 = self.hamming_clusters.get(&group2);

        match (vg1,vg2) {
            (Some(vgroup1),Some(vgroup2)) => Some ({
                let iter_g1 = vgroup1.vertex_list.iter();
                let iter_g2 = vgroup2.vertex_list.iter();
                let mut max_spacing = 0;
                let mut min_spacing = u32::MAX;
                for v1 in iter_g1 {
                    println!("Checking {}",v1);
                    let iter_g2_copy = iter_g2.clone();
                    for v2 in iter_g2_copy {
                        println!("..Checking {} {}",v1,v2);
                        // skip checking spacing for min/max between a vector and itself..
                        if *v1 == *v2 {
                            continue;
                        }
                        if let Some(spacing) = self.vertex_spacing(*v1,*v2)  {
                            println!("HC Spacing between v{} and v{} is {}",v1,v2,spacing);
                            if spacing > max_spacing {
                                max_spacing = spacing;
                            }
                            if spacing < min_spacing {
                                min_spacing = spacing;
                            }
                        }
                        else {
                            return None
                        }
                    }
                }
                println!("HC Spacing between {} and {} is ({},{})",group1,group2,min_spacing,max_spacing);
                (min_spacing,max_spacing)

            }),
            _ => None
        }

    }


    //  cluster 
    //
    //  going to find and combine all the groups
    //  whose hamming distance is either 1 or 2  
    pub fn do_cluster(&mut self, max_dist: u32) {
        let one_bit_bitmask =  BitMaskingTable::new_one_bit(self.hamming_size);
        let two_bit_bitmask =  BitMaskingTable::new_two_bit(self.hamming_size);
        let mut hamming_code_list = Vec::<u32>::new();

        for key in self.hamming_clusters.keys() {
            hamming_code_list.push(key.clone());
        }

        for key in hamming_code_list {
            // skip any hamming codes that are empty
            if self.hamming_clusters[&key].vertex_list.len() == 0 {
                continue
            }
            let current_hamming_code = key.clone();
            for i in 0..one_bit_bitmask.len() {
                let mask = one_bit_bitmask.get_mask(i);
                let dest_hamming_code = (current_hamming_code ^ mask).clone();
                println!("Checking {} and {} result dest -> {}",current_hamming_code,mask,dest_hamming_code);
                if self.hamming_clusters.contains_key(&dest_hamming_code) && 
                    self.hamming_clusters[&dest_hamming_code].vertex_list.len() > 0 {
                    let (_,spacing) = self.hamming_cluster_spacing(current_hamming_code,dest_hamming_code).unwrap() ;
                    println!(" ... Spacing between {} and {} is {}",current_hamming_code,dest_hamming_code,spacing);
                    if spacing < max_dist {
                        self.union_by_code(current_hamming_code,dest_hamming_code);
                    }
                }
            }
            for i in 0..two_bit_bitmask.len() {
                let mask = two_bit_bitmask.get_mask(i);
                let dest_hamming_code = (current_hamming_code ^ mask).clone();
                println!("Checking {} and {} result dest -> {}",current_hamming_code,mask,dest_hamming_code);
                if self.hamming_clusters.contains_key(&dest_hamming_code) &&
                    self.hamming_clusters[&dest_hamming_code].vertex_list.len() > 0 {
                    let (_,spacing) = self.hamming_cluster_spacing(current_hamming_code,dest_hamming_code).unwrap() ;
                    println!(" ___ Spacing between {} and {} is {}",current_hamming_code,dest_hamming_code,spacing);
                    if spacing < max_dist {
                        self.union_by_code(current_hamming_code,dest_hamming_code);
                    }
                }
            }
        }

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
            // if the group hasn't changed from the initial setup
            // this is the top of the tree
            if current_group != current_vertex_group.orig_hamming_code {
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

    pub fn vertex_same_group(&mut self, v1: u32, v2: u32) -> bool {
        let group1 = self.find_group(v1).clone();
        let group2 = self.find_group(v2).clone();
        group1 == group2
    }

    pub fn code_same_group(&mut self, code1: u32, code2: u32) -> bool {
        let group1 = self.find_grouping(code1).clone();
        let group2 = self.find_grouping(code2).clone();
        group1 == group2
    }


    pub fn union_by_vertex(&mut self, v1: u32, v2: u32) {
        let lookup1 = self.vertex_map.get(&v1);
        let lookup2 = self.vertex_map.get(&v2);

        match (lookup1,lookup2) {
            (Some(code1),Some(code2)) => {
                // clone the codes to elminiate borrown issues
                let c1 = code1.clone();
                let c2 = code2.clone();
                self.union_by_code(c1,c2);
            }
            _ => println!("Union by Vertex: Invalid Vertex"),
        }

    } 

    pub fn union_by_code(&mut self, code1: u32, code2: u32) {
        let group1 = self.find_grouping(code1).clone();
        let group2 = self.find_grouping(code2).clone();
        let rank1 = self.get_rank(code1).clone();
        let rank2 = self.get_rank(code2).clone();

        println!("Union of {} and {} - Groups are {} and {}",code1,code2,group1,group2);
        if group1 == group2 {
            //Nothing to do, already are in the same group
        }
        else if rank1 > rank2 {
            self.combine_groups(code2,code1);
            // update the number of remaining groups
            self.groups -= 1;
            println!("> New group for {} is {} - groups={}",group2, group1,self.groups);
            println!("G1 {:#?} G2 {:#?}",self.hamming_clusters[&group1],self.hamming_clusters[&group2]);
        }
        else if rank1 < rank2 {
            self.combine_groups(code1,code2);
            self.groups -= 1;
            println!("< New group for {} is {} - groups={}",group1, group2,self.groups);
            println!("G1 {:#?} G2 {:#?}",self.hamming_clusters[&group1],self.hamming_clusters[&group2]);
        }
        else {
            // code1 and code2 rank are the same...  picking code 1 as the collection to add to
    
            //update the head vertex of code2 group(which is the group number) group of group1
            self.combine_groups(code2,code1);
            self.incr_rank(code1);
            self.groups -= 1;
            println!("= New group for {} is {} - groups={}",group1, group2,self.groups);
            println!("G1 {:#?} G2 {:#?}",self.hamming_clusters[&group1],self.hamming_clusters[&group2]);
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

    fn setup_basic(num: u32) -> HammingClusteringInfo {
        let num_bits = (num as f64).log(2.0).ceil() as u32;
        let mut c = HammingClusteringInfo::new(num_bits);
        for i in 0..num {
            c.add_vertex(i,i);
        }
        println!("Initial Setup {:#?}",c);
        c

    }


    #[test]
    fn initial_setup_test() {
        let mut c = setup_basic(3);
        assert_eq!(c.sizes(),(3,3,3));
        assert_eq!(c.find_group(1),Some(0));
        assert_eq!(c.find_group(2),Some(1));
        assert_eq!(c.find_group(3),Some(2));
        assert_eq!(c.code_same_group(0,1),false);
        assert_eq!(c.code_same_group(0,2),false);
        assert_eq!(c.code_same_group(1,2),false);
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
        let mut c = setup_basic(3);
        c.union_by_code(0,1);
        println!("After Union {:#?}",c);
        assert_eq!(c.find_grouping(1),0);
        assert_eq!(c.find_grouping(0),0);
        println!("group {:#?} {:#?}", c.find_grouping(0),c.find_grouping(1));
        assert_eq!(c.code_same_group(0,1),true);
        assert_eq!(c.get_rank(0),2);
        assert_eq!(c.sizes(),(3,3,2));
    }

    #[test]
    fn bitmask_one_bit_table_test() {
        let b = BitMaskingTable::new_one_bit(3);
        assert_eq!(b.get_mask(0),0b1);
        assert_eq!(b.get_mask(1), 0b10);
        assert_eq!(b.get_mask(2),0b100);
        assert_eq!(b.get_mask(3),0);
    }

    #[test]
    fn bitmask_two_bit_table_test() {
        let b = BitMaskingTable::new_two_bit(3);
        assert_eq!(b.get_mask(0),0b011);
        assert_eq!(b.get_mask(1),0b101);
        assert_eq!(b.get_mask(2),0b110);
        assert_eq!(b.get_mask(3),0);
        let b = BitMaskingTable::new_two_bit(4);
        assert_eq!(b.get_mask(0),0b0011);
        assert_eq!(b.get_mask(1),0b0101);
        assert_eq!(b.get_mask(2),0b1001);
        assert_eq!(b.get_mask(3),0b0110);
        assert_eq!(b.get_mask(4),0b1010);
        assert_eq!(b.get_mask(5),0b1100);
        assert_eq!(b.get_mask(6),0);
    }

    #[test]
    fn vertex_spacing_test() {
        let c = setup_basic(8);
        assert_eq!(c.vertex_spacing(1,2),Some(1));
        assert_eq!(c.vertex_spacing(1,4),Some(2));
        assert_eq!(c.vertex_spacing(8,1),Some(3));

    }

    #[test]
    fn cluster_spacing_test() {
        let mut c = setup_basic(8);
        println!("group for v8 {:#?}", c.find_group(8));
        assert_eq!(c.vertex_cluster_spacing(1,8),Some((3,3)));
        c.union_by_vertex(1,8);
        c.union_by_vertex(1,2);
        println!("Final {:#?}",c);
        assert_eq!(c.vertex_cluster_spacing(1,8),Some((1,3)));

    }

    #[test]
    fn cluster_test() {
        let mut c = setup_basic(16);
        c.do_cluster(3);
        println!("Final {:#?}",c);
        assert_eq!(c.sizes(),(16,16,2));
    }

    #[test]
    fn hamming_cluster_spacing_test() {
        let mut c = setup_basic(16);
        assert_eq!(c.hamming_cluster_spacing(0,4),Some((1,1)))
    }

}
