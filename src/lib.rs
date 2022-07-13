use rand::Rng;

#[derive(Debug)]
pub struct Cluster {
    delta0: i32,
    delta1: i32,
    delta2: i32,
}

impl Cluster {
    fn get_cluster_delta(&self, delta: &str) -> i32 {
        if delta == "delta2" {
            return self.delta2; 
        }
        else if delta == "delta1" {
            return self.delta1; 
        }
        else {
            return self.delta0; 
        }
    } 
}

fn build_cluster(delta0: i32, delta1: i32, delta2: i32) -> Cluster {
    Cluster{
        delta0: delta0, 
        delta1: delta1, 
        delta2: delta2
    }
}

pub fn make_fake_cluster_vec()-> Vec<Cluster>{
    let v: Vec<Cluster> = vec![build_cluster(523, 990, 431), 
                                build_cluster(371, 499, 212), 
                                build_cluster(490, 1097, 117), 
                                build_cluster(242, 947, 098), 
                                build_cluster(761, 866, 514), 
                                build_cluster(241, 281, 131), 
                                build_cluster(520, 824, 378)]; 

    v
}

pub fn find_kth_delta0(v: &mut Vec<Cluster>, k: usize) -> i32 {
    let l = 0; 
    let r = v.len()-1;

    find_kth(v, l, r, k, "delta0")
}

pub fn find_kth_delta1(v: &mut Vec<Cluster>, k: usize) -> i32 {
    let l = 0; 
    let r = v.len()-1;

    find_kth(v, l, r, k, "delta1")
}

pub fn find_kth_delta2(v: &mut Vec<Cluster>, k: usize) -> i32 {
    let l = 0; 
    let r = v.len()-1;

    find_kth(v, l, r, k, "delta2")
}

pub fn sort_by_delta0(v: &mut Vec<Cluster>) ->  &mut Vec<Cluster> {
    v.sort_by(|a, b| a.delta0.cmp(&b.delta0));
    v
}
pub fn sort_by_delta1(v: &mut Vec<Cluster>) -> &mut Vec<Cluster> {
    v.sort_by(|a, b| a.delta1.cmp(&b.delta1));
    v
}
pub fn sort_by_delta2(v: &mut Vec<Cluster>) -> &mut Vec<Cluster> {
    v.sort_by(|a, b| a.delta2.cmp(&b.delta2));
    v
}


pub fn partition(v: &mut Vec<Cluster>, l: usize, r: usize, delta: &str) -> usize {
    let mut i = l; 
    let mut j = l;

    while j < r {
        if v[j].get_cluster_delta(delta) < v[r].get_cluster_delta(delta) {
            v.swap(i, j);
            i += 1; }
            
        j += 1; }
    
    v.swap(i, r);
    i }

pub fn random_partition(v: &mut Vec<Cluster>, l: usize, r: usize, delta: &str) -> usize {
    let n = r - l;
    let pivot = rand::thread_rng().gen_range(0..=(n));
    v.swap(l+pivot, r); 
    partition(v, l, r, delta) 
}

pub fn find_kth(v: &mut Vec<Cluster>, l: usize, r: usize, k: usize, delta: &str) -> i32 {
    let mut b = v[0].get_cluster_delta(delta); 

    if l <= r {
        let partition_index = random_partition(v, l, r, delta); 

        if partition_index == k {
            b = v[partition_index].get_cluster_delta(delta); 
            return b; 
        } else if partition_index > k {
            return find_kth(v, l, partition_index-1, k, delta); 
        } else if partition_index < k {
            return find_kth(v, partition_index+1, r, k, delta); 
        }

    }
    return b; 
}
