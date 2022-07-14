use partition_non_generic::find_kth;
use partition_non_generic::make_fake_cluster_vec;
use partition_non_generic::sort_by_delta0;
use partition_non_generic::sort_by_delta1;
use partition_non_generic::sort_by_delta2;
use partition_non_generic::Cluster;
use partition_non_generic::Delta;

fn main() {
    let mut v: Vec<Cluster> = make_fake_cluster_vec();
    let r = v.len() - 1;

    println!("kth delta0: {}", find_kth(&mut v, 0, r, 2, &Delta::Delta0));
    println!("kth delta1: {}", find_kth(&mut v, 0, r, 3, &Delta::Delta1));
    println!("kth delta2: {}", find_kth(&mut v, 0, r, 4, &Delta::Delta2));
    sort_by_delta0(&mut v);
    println!("sorted by delta0: {:?}", v);
    sort_by_delta1(&mut v);
    println!("sorted by delta1: {:?}", v);
    sort_by_delta2(&mut v);
    println!("sorted by delta2: {:?}", v);
}
