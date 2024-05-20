mod monorepo;
mod version;

fn main() {
    let is = monorepo::check_is_monorepo();
    println!("is mono {}", is);
}
