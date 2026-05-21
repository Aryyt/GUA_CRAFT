use guacraft::gamehost::GameHost;

fn main() {
    println!("阿弥陀佛!");

    let host = GameHost::new().unwrap();
    host.run().unwrap();
}
