use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader,BufRead};
use regex::Regex;
use std::io;

mod cmd_line;
use crate::cmd_line::CommandArgs;

mod cluster;
use crate::cluster::ClusteringInfo;

mod hammingcluster;

fn process_standard_cluster(file: &mut File, num_clusters : usize ) {

    let mut reader = BufReader::new(file);

    // read the first line
    let mut line = String::new();
    let _len = reader.read_line(&mut line).unwrap();

    let mut c = ClusteringInfo::new();

	let mut _count = 0;
    for line in reader.lines() {
		_count += 1;	
		let line_data = line.unwrap();
 //       println!("Processing {}",line_data);

        // split the line into the vertex and the list of adjacent vertexes/weight pairs
        let re_vertex = Regex::new(r"\s*(?P<src>\d+)\s+(?P<dest>\d+)\s+(?P<weight>-*\d+).*$").unwrap();
        // adjacent vertexes are in the format vertex,weight   - and regex below allows for
        // whitespace
        let caps = re_vertex.captures(&line_data).unwrap();
        //let mut iter = line_data.split_whitespace();
        //let src = iter.next().unwrap();
        //let dest = iter.next().unwrap();
        //let weight = iter.next().unwrap();
        let src_vertex = caps["src"].parse::<u32>().unwrap(); 
        let dest_vertex = caps["dest"].parse::<u32>().unwrap(); 
        let weight = caps["weight"].parse::<i32>().unwrap(); 
        //let src_vertex = src.parse::<u32>().unwrap(); 
        //let dest_vertex = dest.parse::<u32>().unwrap(); 
        //let weight = weight.parse::<i32>().unwrap(); 
        c.add_edge(src_vertex,dest_vertex,weight);
        if _count % 1000 == 0 {
 //           println!("Added Edge #{}: from {} - {} wgt: {} --  ",_count,src_vertex,dest_vertex,weight);
            print!(".");
            io::stdout().flush().unwrap();
        }
    }

    let (num_vertex,num_edges) = c.size();
    println!("Completed reading {} vertex and {} edges",num_vertex,num_edges);
    let distance = c.cluster(num_clusters);
    println!("Distance at {} clusters is {}",num_clusters,distance);

}


fn main() {


    let cmd_line = CommandArgs::new();

    println!("Hello, {:?}!",cmd_line);

    println!("Determining the distances for {} clusters",cmd_line.num_clusters);
  // Create a path to the desired file
    let path = Path::new(&cmd_line.filename);
    let display = path.display();


    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };


    process_standard_cluster(&mut file,cmd_line.num_clusters);

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

    fn setup_basic() -> ClusteringInfo {
            let basic_data = vec!( 
                (1,2,1),
                (1,3,4),
                (1,4,5),
                (1,5,10),
                (1,6,11),
                (1,7,12),
                (2,3,3),
                (2,4,4),
                (2,5,9),
                (2,6,10),
                (2,7,11),
                (3,4,1),
                (3,5,6),
                (3,6,7),
                (3,7,8),
                (4,5,5),
                (4,6,6),
                (4,7,7),
                (5,6,1),
                (5,7,2),
                (6,7,1),
           );	

		let mut c = ClusteringInfo::new();
        for e in basic_data {
            c.add_edge(e.0,e.1,e.2);
        }
		assert_eq!(c.size(),(7,21));
        c
	} 

    #[test]
    fn basic() {
        let mut c = setup_basic();
		assert_eq!(c.cluster(3),3);
		assert_eq!(c.cluster(2),5);
    }

 }
