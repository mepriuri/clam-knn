use fake::{Dummy, Fake, Faker};

#[derive(Debug, Dummy)]
pub struct FakeCluster {
#[dummy(faker = "1000..2000")]
    delta0: u32,
    delta1: u32,
    delta2: u32,
}

fn fake_cluster() {

    let c: FakeCluster = Faker.fake();
    println!("{:?}", c);
    println!("{}", c.delta0); 
    println!("{}", c.delta1); 
    println!("{}", c.delta2); 

}

fn main() {
 
    fake_cluster(); 
    println!("hello"); 
}
