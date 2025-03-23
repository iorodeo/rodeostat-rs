use rodeostat_rs::Rodeostat;
use rodeostat_rs::param::{
    ChronoampParam, 
    ConstantParam, 
    CyclicParam, 
    LinearSweepParam, 
    MultistepParam, 
    SinusoidParam,
    SquareWaveParam,
};


fn main() {

    let mut dev = Rodeostat::new("/dev/ttyACM0")
        .expect("unable to open port");

    println!("dev: {dev:?}");
    println!();

    let variant = dev.get_hardware_variant()
        .expect("error getting variant");
    println!("variant = {variant}");

    let version = dev.get_firmware_version()
        .expect("error getting version");
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

    let test_names = dev.get_test_names()
        .expect("error gettting test names");
    println!("test_names: {test_names:?}");

    let p: CyclicParam = dev.get_param()
        .expect("error unable to get cyclic test param");
    println!("{p:?}");

    let p: SinusoidParam = dev.get_param()
        .expect("error unable to get sinusoid test param");
    println!("{p:?}");

    let p: ConstantParam = dev.get_param()
        .expect("error unable to get constant test param"); 
    println!("{p:?}");

    let p: SquareWaveParam = dev.get_param()
        .expect("error unable to get square wave test param"); 
    println!("{p:?}");

    let p: LinearSweepParam = dev.get_param()
        .expect("error unable to get square wave test param"); 
    println!("{p:?}");

    let p: ChronoampParam = dev.get_param()
        .expect("error unable to get square wave test param"); 
    println!("{p:?}");

    let p: SquareWaveParam  = dev.get_param()
        .expect("error unable to get square wave test param"); 
    println!("{p:?}");

    let p: MultistepParam  = dev.get_param()
        .expect("error unable to get multistep test param"); 
    println!("{p:?}");

    let p: ConstantParam = dev.get_param()
        .expect("error unable to get constant test param"); 
    println!("{p:?}");

    let p : ConstantParam = ConstantParam {
        quiet_value: 0.5, 
        quiet_time: 300,
        value: -3.0, 
        duration: 2000,
    };
    let q = dev.set_param(p)
        .expect("error unable to set parameters");
    println!("{q:?}");

    let volt_range = dev.get_volt_range()
        .expect("error unable to get volt_range");
    println!("volt_range = {volt_range}");


    let all_volt_ranges = dev.get_all_volt_range()
        .expect("error unable to get volt ranges");
    println!("all_volt_ranges = {all_volt_ranges:?}");

    println!("loop over volt ranges");
    for volt_range in all_volt_ranges { 
        let x = dev.set_volt_range(&volt_range)
                .expect("error unable to set volt_range");
        println!("volt_range = {x}");
    }

    let volt_range = dev.set_volt_range("1V")
        .expect("error unable to set volt_range");
    println!("volt_range = {volt_range}");

    let volt_range = dev.get_volt_range()
        .expect("error unable to get volt_range");
    println!("volt_range = {volt_range}");

    let curr_range = dev.get_curr_range()
        .expect("error unable to get current range");
    println!("curr_range: {curr_range}");

    let curr_range = dev.set_curr_range("100uA")
        .expect("error unable to set current range");
    println!("curr_range: {curr_range}");

    let curr_range = dev.get_curr_range()
        .expect("error unable to get current range");
    println!("curr_range: {curr_range}");

    let all_curr_ranges = dev.get_all_curr_range()
        .expect("error unable to get all curr ranges");
    println!("all_curr_ranges = {all_curr_ranges:?}");
    for curr_range in all_curr_ranges {
        let x = dev.set_curr_range(&curr_range)
            .expect("error unable to set curr range");
        println!("curr_range = {x}");
    }
    let curr_range = dev.set_curr_range("10uA")
        .expect("error unable to set current range");
    println!("curr_range: {curr_range}");

    let device_id = dev.get_device_id()
        .expect("error unable to get device id");
    println!("device_id: {device_id}");
    let device_id = if device_id == 0 {
        1 
    } else {
        0
    };
    let device_id = dev.set_device_id(device_id)
        .expect("error unable to set device id");
    println!("device_id: {device_id}");

    let sample_period = dev.get_sample_period()
        .expect("error unable to get sample period");
    println!("sample_period: {sample_period}");

    let sample_period = if sample_period == 5 {
        10
    } else {
        5
    };
    let sample_period = dev.set_sample_period(sample_period)
        .expect("error unable to set sample_period");
    println!("sample_period: {sample_period}");

    let sample_rate = dev.get_sample_rate()
        .expect("error unable to get sample_rate");
    println!("sample_rate: {sample_rate}");
    let sample_rate = if sample_rate < 200.0f32 {
        200.0f32 
    } else {
        100.0f32
    };
    let sample_rate = dev.set_sample_rate(sample_rate)
        .expect("error unable to set sample rate");
    println!("sample_rate: {sample_rate}");

    let test_names = dev.get_test_names()
        .expect("error gettting test names");

    for name in test_names {
        let test_done_time = dev.get_test_done_time(&name)
            .expect("error unable to get test done time");
        println!("test: {name}, test_done_time: {test_done_time}");
    }

    let hardware_version = dev.get_hardware_version()
        .expect("error unable to get hardware version");
    println!("hardware_version: {hardware_version}");
        

}
