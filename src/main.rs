use rodeostat::Rodeostat;

fn main() {

    let mut dev = Rodeostat::new("/dev/ttyACM0").expect("unable to open port");

    println!("dev: {dev:?}");
    println!();

    let variant = dev.get_variant().expect("error getting variant");
    println!("variant = {variant}");
    let version = dev.get_version().expect("error getting version");
    println!("version = {version}");

    let num = 10;

    for i in 0..num {
        let v = (i as f32)/(num as f32);
        let v = dev.set_volt(v).expect("error getting voltage");
        let i = dev.get_curr().expect("error reading current");
        println!("v = {v}, i = {i}");
    }

    let test_names = dev.get_test_names().expect("error gettting test names");
    println!("test_names: {test_names:?}");
}
