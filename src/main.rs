use partition_non_generic::find_kth_delta0; 
use partition_non_generic::find_kth_delta1; 
use partition_non_generic::find_kth_delta2; 
use partition_non_generic::sort_by_delta0; 
use partition_non_generic::sort_by_delta1; 
use partition_non_generic::sort_by_delta2; 
use partition_non_generic::make_fake_cluster_vec; 
use partition_non_generic::Cluster; 

fn main() {
    let mut v: Vec<Cluster> = make_fake_cluster_vec(); 
    
    println!("kth delta0: {}", find_kth_delta0(&mut v, 2));
    println!("kth delta1: {}", find_kth_delta1(&mut v, 3));
    println!("kth delta2: {}", find_kth_delta2(&mut v, 4));
    println!("sorted by delta0: {:?}", sort_by_delta0(&mut v));
    println!("sorted by delta1: {:?}", sort_by_delta1(&mut v));
    println!("sorted by delta2: {:?}", sort_by_delta2(&mut v));

}
