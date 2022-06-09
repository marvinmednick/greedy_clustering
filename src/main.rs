use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;
use std::io;

mod cmd_line;
use crate::cmd_line::CommandArgs;

mod unionfind;
use crate::unionfind::ClusteringInfo;


fn main() {


    let cmd_line = CommandArgs::new();

    println!("Hello, {:?}!",cmd_line);

    println!("Determining the distances for {} clusters",cmd_line.num_clusters);
  // Create a path to the desired file
    let path = Path::new(&cmd_line.filename);
    let display = path.display();


    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

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
    let distance = c.cluster(cmd_line.num_clusters);
    println!("Distance at {} clusters is {}",cmd_line.num_clusters,distance);


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
    use crate::graph::Edge;

	fn setup_basic1() -> Graph {
		let mut g = Graph::new();
		assert_eq!(g.add_edge(1,2,1),Some(1));
        assert_eq!(g.add_edge(1,3,1),Some(2));
        assert_eq!(g.add_edge(2,3,1),Some(1));
        assert_eq!(g.add_edge(2,4,1),Some(2));
        assert_eq!(g.add_edge(3,4,1),Some(1));
        assert_eq!(g.get_outgoing(1),&[Edge::new(2,1),Edge::new(3,1)]);
		assert_eq!(g.get_outgoing(2),&[Edge::new(3,1),Edge::new(4,1)]);
		assert_eq!(g.get_outgoing(3),&[Edge::new(4,1)]);
		assert_eq!(g.get_outgoing(4),&[]);
		g
	} 

    #[test]
    fn basic() {
		let mut g = Graph::new();
		assert_eq!(g.create_vertex(&1),Some(1));
		assert_eq!(g.create_vertex(&2),Some(2));
		assert_eq!(g.add_edge(1,2,1),Some(1));
		assert_eq!(g.get_vertexes(),vec!(1,2));
		assert_eq!(g.create_vertex(&3),Some(3));
		assert_eq!(g.add_edge(1,3,1),Some(2));
		assert_eq!(g.add_edge(2,3,1),Some(1));
		assert_eq!(g.get_vertexes(),vec!(1,2,3));
		assert_eq!(g.add_edge(1,4,1),Some(3));
		assert_eq!(g.get_vertexes(),vec!(1,2,3,4));
		println!("{:?}",g);

    }

	#[test]
	fn test_add() {
		let mut g = Graph::new();
		assert_eq!(g.add_edge(1,2,1),Some(1));
		assert_eq!(g.get_outgoing(1),&[Edge::new(2,1)]);
		assert_eq!(g.get_incoming(2),&[Edge::new(1,1)]);
		assert_eq!(g.add_edge(1,3,1),Some(2));
		assert_eq!(g.get_outgoing(1),&[Edge::new(2,1),Edge::new(3,1)]);
		assert_eq!(g.get_incoming(2),&[Edge::new(1,1)]);
	}

	#[test]
	fn test_add_del() {
		let mut g = setup_basic1();
		assert_eq!(g.get_outgoing(1),&[Edge::new(2,1),Edge::new(3,1)]);
		assert_eq!(g.add_edge(1,2,1),Some(3));
		assert_eq!(g.get_outgoing(1),&[Edge::new(2,1),Edge::new(3,1)]);
		assert_eq!(g.get_outgoing(2),&[Edge::new(3,1),Edge::new(4,1)]);
		assert_eq!(g.get_outgoing(3),&[Edge::new(4,1)]);
		assert_eq!(g.delete_edge(1,2,1),Ok(()));
		assert_eq!(g.get_outgoing(1),&[Edge::new(2,1),Edge::new(3,1)]);
		assert_eq!(g.delete_edge(1,2,1),Ok(()));
		assert_eq!(g.get_outgoing(1),&[Edge::new(3,1)]);
		
	}


 }
