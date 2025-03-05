use rodeostat_rs::Rodeostat;

fn main() {

    let mut dev = Rodeostat::new("/dev/ttyACM0").expect("unable to open port");

    println!("dev: {dev:?}");
    println!();

    let variant = dev.get_hardware_variant().expect("error getting variant");
    println!("variant = {variant}");
    let version = dev.get_firmware_version().expect("error getting version");
    println!("version = {version}");

    let num = 10;

    let connected = dev.set_all_elect_connected(true)
        .expect("unable to connected electrodes");
    println!("connected = {connected}");

    let connected = dev.get_all_elect_connected()
        .expect("unable to get connection status");
    println!("connected = {connected}");

    for i in 0..num {
        let v = 0.5*(i as f32)/(num as f32);
        let v = dev.set_volt(v).expect("error getting voltage");
        let i = dev.get_curr().expect("error reading current");
        let v_ref = dev.get_ref_volt().expect("error reading reference voltage");
        println!("v = {v:0.3}, i = {i:0.3}, v_ref = {v_ref:0.3}");
    }

    let connected = dev.set_all_elect_connected(false)
        .expect("unable to connected electrodes");
    println!("connected = {connected}");

    let connected = dev.get_all_elect_connected()
        .expect("unable to get connection status");
    println!("connected = {connected}");

    let test_names = dev.get_test_names().expect("error gettting test names");
    println!("test_names: {test_names:?}");
}
