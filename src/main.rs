use guacraft::gamehost::GameHost;

fn main() {
    println!("阿弥陀佛!");

    let mut host = GameHost::new().unwrap();
    host.run().unwrap();
}
