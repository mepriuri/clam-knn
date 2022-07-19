use rand::Rng;
use std::cmp::Ordering;
use arbitrary::Arbitrary; 
#[cfg(test)]
extern crate quickcheck;
//#[cfg(test)]
//use quickcheck::{Arbitrary,Gen};
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;


#[derive(Debug, Clone, Arbitrary)]
pub struct Cluster {
    delta0: u32,
    delta1: u32,
    delta2: u32,
}


#[derive(Debug)]
pub enum Delta {
    Delta0,
    Delta1,
    Delta2,
}

impl Cluster {
    fn get_cluster_delta(&self, delta: &Delta) -> u32 {

        //! `get_cluster_delta()` gets the value of the given delta type (0, 1, or 2)
        //! for a cluster

        match delta {
            Delta::Delta0 => self.delta0,
            Delta::Delta1 => self.delta1,
            Delta::Delta2 => self.delta2,
        }
    }
}


pub fn sort_by_delta0(v: &mut [Cluster]) {
    v.sort_by(|a, b| a.delta0.cmp(&b.delta0));
}

pub fn sort_by_delta1(v: &mut [Cluster]) {
    v.sort_by(|a, b| a.delta1.cmp(&b.delta1));
}

pub fn sort_by_delta2(v: &mut [Cluster]) {
    v.sort_by(|a, b| a.delta2.cmp(&b.delta2));
}

pub fn sort_by_delta(v: &mut [Cluster], delta: &Delta) {
    v.sort_by(|a, b| a.get_cluster_delta(delta).cmp(&b.get_cluster_delta(delta)));
}


pub fn partition(v: &mut [Cluster], l: usize, r: usize, delta: &Delta) -> usize {
    
    //! Called by `random_partition()`. Takes the same arguments as `find_kth()` and returns the "true"
    //! index (i.e., index if `v` were sorted) of the Cluster currently at the rightmost possible index
    //! that could still have the `k`th Cluster (i.e.,`r`th index). Cluster at this `r`th index is based
    //! on random `pivot` from `random_partition()`.
    //!
    //! `j` tracks the index of the Cluster being evaluated at each iteration of the while loop. `i`
    //! counts the number of Clusters whose delta value is less than that of the `r`th Cluster.
    //!
    //! If a Cluster has a delta value greater than or equal to that of the `r`th Cluster, it  
    //! swaps position with the next Cluster whose delta is less than that of the `r`th Cluster.
    //!
    //! Since `i` counts the number of Clusters with a delta less than the `r`th Cluster, the final
    //! swap of the `i`th and `r`th Clusters puts that `r`th Cluster in its correct position (as if
    //! `v` were sorted).

    let mut i = l;
    let mut j = l;

    while j < r {
        if v[j].get_cluster_delta(delta) < v[r].get_cluster_delta(delta) {
            v.swap(i, j);
            i += 1;
        }

        j += 1;
    }

    v.swap(i, r);
    i
}

pub fn random_partition(v: &mut [Cluster], l: usize, r: usize, delta: &Delta) -> usize {
    //! Takes the same arguments as `find_kth()` and returns the "true" index (index if `v` were sorted)
    //! of the Cluster currently at a randomly selected `pivot` index.
    //!
    //! Chooses a random `pivot` value within the range plausible indices
    //! for the `k`th Cluster and swaps the `r`th Cluster with the Cluster in the pivot position.
    //! Calls `partition()` which evaluates Clusters in `v` against the `r`th Cluster (which
    //! is the Cluster formerly at the `pivot` index).
    //!
    //! `partition()` returns a count of Clusters with a delta value less than that at the `r`th
    //! (formerly, `pivot`) index (i.e., the index of `r`th Cluster if `v` were sorted); `random_partition()`
    //! returns this same value

    let pivot = rand::thread_rng().gen_range(l..=r);
    v.swap(pivot, r);
    partition(v, l, r, delta)
}

pub fn find_kth(v: &mut [Cluster], l: usize, r: usize, k: usize, delta: &Delta) -> u32 {
    //! `find_kth()` finds the kth Cluster in vector without completely sorting it
    //! Based on the QuickSelect Algorithm found here: https://www.geeksforgeeks.org/quickselect-algorithm/
    //!
    //! Takes mutable reference to a vector of Clusters (`v`), the leftmost index in `v` that could
    //! possibly have the `k`th cluster (`l`), the rightmost index in `v` that could possibly
    //! have the `k`th cluster (`r`), the desired index (`k`), and the desired delta type of the Cluster in the `k`th
    //! position if `v` were sorted (`delta`).
    //!
    //! With each recursive call, a random pivot is chosen and `partition_index` reflects the number of
    //! Clusters in `v` with a delta value less than that of the Cluster at the random pivot index. If
    //! `partition_index < k`, `find_kth()` calls itself with `l` adjusted to reflect the new possible indices
    //! for the `k`th Cluster (any index greater than `partition_index`). If `partition_index < k` `find_kth()` calls
    //! itself with `r` similarly adjusted.
    //!
    //! Recursion ceases when 'partition_index == k', at which point the desired delta value of the `k`th Cluster
    //! (`kth_delta`) is returned.

    let mut kth_delta = v[0].get_cluster_delta(delta);

    if l <= r {
        let partition_index = random_partition(v, l, r, delta);

        match partition_index.cmp(&k) {
            Ordering::Less => {
                return find_kth(v, partition_index + 1, r, k, delta);
            }
            Ordering::Greater => {
                return find_kth(v, l, partition_index - 1, k, delta);
            }
            Ordering::Equal => {
                kth_delta = v[partition_index].get_cluster_delta(delta);
                return kth_delta;
            }
        }
    }
    kth_delta
}


#[cfg(test)]
mod tests {

    use crate::find_kth; 
    use crate::Cluster; 
    use crate::Delta; 

    #[test]
    fn find_2nd_delta0() {

        let mut v: Vec<Cluster> = vec![
        Cluster{delta0: 523, delta1: 990, delta2: 431},
        Cluster{delta0: 371, delta1: 499, delta2: 212},
        Cluster{delta0: 490, delta1: 1097, delta2: 117},
        Cluster{delta0: 242, delta1: 947, delta2: 198},
        Cluster{delta0: 761, delta1: 866, delta2: 514},
        Cluster{delta0: 241, delta1: 281, delta2: 131},
        Cluster{delta0: 520, delta1: 824, delta2: 378}, ];

        let r = v.len() - 1;

        assert_eq!(find_kth(&mut v, 0, r, 2, &Delta::Delta0), 371);
    }

    #[test]
    fn find_3rd_delta1() {

        let mut v: Vec<Cluster> = vec![
        Cluster{delta0: 523, delta1: 990, delta2: 431},
        Cluster{delta0: 371, delta1: 499, delta2: 212},
        Cluster{delta0: 490, delta1: 1097, delta2: 117},
        Cluster{delta0: 242, delta1: 947, delta2: 198},
        Cluster{delta0: 761, delta1: 866, delta2: 514},
        Cluster{delta0: 241, delta1: 281, delta2: 131},
        Cluster{delta0: 520, delta1: 824, delta2: 378}, ];

        let r = v.len() - 1;
        
        assert_eq!(find_kth(&mut v, 0, r, 3, &Delta::Delta1), 866);
    }

    #[test]
    fn find_4th_delta2() {

        let mut v: Vec<Cluster> = vec![
            Cluster{delta0: 523, delta1: 990, delta2: 431},
            Cluster{delta0: 371, delta1: 499, delta2: 212},
            Cluster{delta0: 490, delta1: 1097, delta2: 117},
            Cluster{delta0: 242, delta1: 947, delta2: 198},
            Cluster{delta0: 761, delta1: 866, delta2: 514},
            Cluster{delta0: 241, delta1: 281, delta2: 131},
            Cluster{delta0: 520, delta1: 824, delta2: 378}, ];
    
            let r = v.len() - 1;

        assert_eq!(find_kth(&mut v, 0, r, 4, &Delta::Delta2), 378);
    }

    use fake::{Dummy, Fake, Faker};

    #[derive(Debug, Dummy)]
    pub struct FakeCluster {
    #[dummy(faker = "1000..2000")]
        delta0: u32,
        delta1: u32,
        delta2: u32,
}
    #[test]
    fn fake_cluster() {

        let c: FakeCluster = Faker.fake();
        println!("{:?}", c);
        println!("{}", c.delta0); 
        println!("{}", c.delta1); 
        println!("{}", c.delta2); 


    }
    // use crate::sort_by_delta0; 
    // use crate::sort_by_delta1; 
    // use crate::sort_by_delta2; 

    // #[test]
    // fn sort_delta0() { 

    //     let mut v: Vec<Cluster> = vec![
    //         Cluster{delta0: 523, delta1: 990, delta2: 431},
    //         Cluster{delta0: 371, delta1: 499, delta2: 212},
    //         Cluster{delta0: 490, delta1: 1097, delta2: 117},
    //         Cluster{delta0: 242, delta1: 947, delta2: 198},
    //         Cluster{delta0: 761, delta1: 866, delta2: 514},
    //         Cluster{delta0: 241, delta1: 281, delta2: 131},
    //         Cluster{delta0: 520, delta1: 824, delta2: 378}, ];

    //     sort_by_delta0(&mut v);
    //     assert_eq!(v, []);   

    // } 

    // #[test]
    // fn sort_delta1() {

    //     let mut v: Vec<Cluster> = vec![
    //         Cluster{delta0: 523, delta1: 990, delta2: 431},
    //         Cluster{delta0: 371, delta1: 499, delta2: 212},
    //         Cluster{delta0: 490, delta1: 1097, delta2: 117},
    //         Cluster{delta0: 242, delta1: 947, delta2: 198},
    //         Cluster{delta0: 761, delta1: 866, delta2: 514},
    //         Cluster{delta0: 241, delta1: 281, delta2: 131},
    //         Cluster{delta0: 520, delta1: 824, delta2: 378}, ];
        
    //     sort_by_delta2(&mut v);  
    //     assert_eq!(v, []);
    //  } 

    // #[test]
    // fn sort_delta2() {

    //     let mut v: Vec<Cluster> = vec![
    //         Cluster{delta0: 523, delta1: 990, delta2: 431},
    //         Cluster{delta0: 371, delta1: 499, delta2: 212},
    //         Cluster{delta0: 490, delta1: 1097, delta2: 117},
    //         Cluster{delta0: 242, delta1: 947, delta2: 198},
    //         Cluster{delta0: 761, delta1: 866, delta2: 514},
    //         Cluster{delta0: 241, delta1: 281, delta2: 131},
    //         Cluster{delta0: 520, delta1: 824, delta2: 378}, ];
            
    //     sort_by_delta2(&mut v); 

    //     assert_eq!(v[0..v.len()], []);
    //  } 
    use rand::Rng;
    use crate::sort_by_delta; 
    #[quickcheck]
    fn prop(v: &mut [Cluster], delta: Delta) -> bool {
        sort_by_delta(v, &delta); 
        let ind1 = rand::thread_rng().gen_range(1..=v.len()); 
        let ind2 = rand::thread_rng().gen_range(ind1..=v.len()); 
        let slice1 = &v[0..ind1]; 
        let slice2 = &v[0..ind2]; 
        let av1 = slice1.collect().iter().sum::<u32>() as f32 / slice1.len() as f32; 
        let av2 = slice2.collect().iter().sum::<u32>() as f32 / slice2.len() as f32; 
        // let mut av1 = 0;
        // let mut av2 = 0; 
        // let mut i = 0; 
        // for i in v[0..ind2] {
        //     if i <= ind1{
        //         av1 += v[i].get_cluster_delta(delta);
        //         av2 += v[i].get_cluster_delta(delta); 
        //     }
        //     else{av2 += v[i].get_cluster_delta(delta); }
            
        //     i += 1;
        // }
        
        av1 <= av2

        
    }

    // quickcheck! {
    //     fn prop(v: &[Cluster], delta: Delta) -> bool {

    //         let ind1 = rand::thread_rng().gen_range(1..=v.len()); 
    //         let ind2 = rand::thread_rng().gen_range(ind1..=v.len()); 
    //         let slice1 = &v[0..ind1]; 
    //         let slice2 = &v[0..ind2]; 
    //         let av1 = slice1.collect().iter().sum::<u32>() as f32 / slice1.len() as f32; 
    //         let av2 = slice2.collect().iter().sum::<u32>() as f32 / slice2.len() as f32; 
    //         // let mut av1 = 0;
    //         // let mut av2 = 0; 
    //         // let mut i = 0; 
    //         // for i in v[0..ind2] {
    //         //     if i <= ind1{
    //         //         av1 += v[i].get_cluster_delta(delta);
    //         //         av2 += v[i].get_cluster_delta(delta); 
    //         //     }
    //         //     else{av2 += v[i].get_cluster_delta(delta); }
                
    //         //     i += 1;
    //         // }
            
    //         av1 <= av2

            
    //     }

    //     }
    }

