use craft::Package;

fn main() {
    let pkg = Package::new("craft@>=4.0.0 <4.1.0");

    println!("{:?}", pkg)
}
