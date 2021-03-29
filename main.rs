use std::sync::{Arc, Mutex};
use std::thread;
use std::f64::INFINITY;

//using u8s as vertex labels and f64s as edge costs
type Vertex = usize;
type Cost = f64;
type Graph = Vec<Vec<(Vertex, Cost)>>;

fn concurrent_prims(s: Vertex, g: &Graph) -> Vec<Vertex> {
	//currently found costs of edge connecting to a vertex to its successor
	let costs = Arc::new(Mutex::new(vec![INFINITY; g.len()]));
	//ensures start vertex is considered first
	costs.lock().unwrap()[s] = 0.0;
	//mst stored as vector associates an index vertex with a neighbour in the tree
	let mut mst = vec![0; g.len()];
	//vertices left to visit
	let verts: Arc<Mutex<Vec<Vertex>>> = Arc::new(Mutex::new((0..g.len()).collect()));
	//fetch number of logical cores on this computer - this is how many threads are used
	let num_of_threads = num_cpus::get();
	while verts.lock().unwrap().len() > 0 {
		//used for slicing verts to distribute elements to threads most efficiently
		let quot = verts.lock().unwrap().len() / num_of_threads;
		let rem = verts.lock().unwrap().len() % num_of_threads;
		//store a ref to threads so we can let the main thread make sure they've finished before continuing
		let mut handles = Vec::new();
		//index of slices of verts used by threads
		let mut slice_idx = 0;
		//the vertex with the minimum costs-value in this iteration (Global Minimum Vertex)
		let gmv = Arc::new(Mutex::new(None));
		//CONCURRENT PART: each thread is given a slice of verts and finds the vertex in that slice with the minimum costs-value, gmv mutex is then compared to the local minimum vertex (lmv) and set accordingly
		for thread_idx in 0..num_of_threads {
			handles.push(thread::spawn({
				//add to reference count of data we use in threads
				let verts_clone = Arc::clone(&verts);
				let costs_clone = Arc::clone(&costs);
				let gmv_clone = Arc::clone(&gmv);
				move || {
						//this thread is given verts[slice_idx..end_slice_idx] to find local min from
						let end_slice_idx = slice_idx + quot + if thread_idx < rem {1} else {0};
						//Local Minimum Vertex (lmv)
						let mut lmv: Option<Vertex> = None;
						//this thread gets the locks for verts and costs to find the thread's lmv
						let verts_access = verts_clone.lock().unwrap();
						let costs_access = costs_clone.lock().unwrap();
						//iterate over each vertex in the slice given to this thread to determine the lmv
						for &vertex in (&verts_access[slice_idx..end_slice_idx]).iter() {
							lmv = match lmv {
								None => Some(vertex),
								Some(cur_lmv) => if costs_access[vertex] < costs_access[cur_lmv] {
												     Some(vertex)
												 } else {
													 lmv
												 },
							};
						}
						//each thread accesses the global minimum vertex (gmv) to see if their lmv has a smaller costs-value (and therefore this thread's lvm should be the new gmv)
						let mut gmv_access = gmv_clone.lock().unwrap();
						*gmv_access = match *gmv_access {
							None => lmv,
							Some(cur_gmv) => match lmv {
								None => *gmv_access,
								Some(lmv_val) => if costs_access[lmv_val] < costs_access[cur_gmv] {
													 lmv
												 } else {
												 	 *gmv_access
												 },
							},
						};
					}}));
			//update the slice index, the first rem threads are given the 'leftover' vertices when num_of_threads doesn't divide verts.len()
			slice_idx += quot + if thread_idx < rem {1} else {0};
		}
		//wait for all the threads to finish
		for handle in handles {
			handle.join().unwrap();
		}
		//shadow gmv as its integer value without the arc and mutexes
		let gmv = gmv.lock().unwrap().unwrap();
		//new accesses to costs and verts because so we don't have to keep getting the lock
		let mut costs_access = costs.lock().unwrap();
		let mut verts_access = verts.lock().unwrap();
		//remove the gmv from verts
		verts_access.retain(|&x| x != gmv);
		//update the mst and costs by iterating over the vertices connected to the gmv
		for &(vertex, cost) in g[gmv].iter() {
			//if vertex is still in verts and has a smaller than what is recorded in costs
			if verts_access.iter().any(|&v| v == vertex) && cost < costs_access[vertex] {
				costs_access[vertex] = cost;
				mst[vertex] = gmv;
			}
		}
	}
	mst
}

fn main() {
	let s: Vertex = 0;
	let g: Graph = 
		vec![vec![(1, 6.0), (2, 1.0), (3, 5.0)],
			 vec![(0, 6.0), (2, 5.0), (4, 3.0)],
			 vec![(0, 1.0), (1, 5.0), (3, 5.0), (4, 6.0), (5, 4.0)],
			 vec![(0, 5.0), (2, 5.0), (5, 2.0)],
			 vec![(1, 3.0), (2, 6.0), (5, 6.0)],
			 vec![(2, 4.0), (3, 2.0), (4, 6.0)]];
	//run prims algorithm + time execution in seconds
	let start = std::time::Instant::now();
    let e = concurrent_prims(s, &g);
    let end = std::time::Instant::now();
    let time_taken = end.checked_duration_since(start)
    	.unwrap();
	//output result
	println!("edges:");
	for (u, &v) in e.iter().enumerate() {
		println!("{} - {}", u, v);
	}
   	println!("took {:?}", time_taken);
}
