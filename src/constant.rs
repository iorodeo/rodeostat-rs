// Serial communications parameters
pub const SERIAL_BAUDRATE: u32 = 115_200;
pub const SERIAL_TIMEOUT: u64 = 500;

// Voltammetric test names
pub const DUMMY_TEST_STR: &str = "dummy";
pub const CYCLIC_TEST_STR: &str = "cyclic";
pub const SINUSOID_TEST_STR: &str = "sinusoid";
pub const CONSTANT_TEST_STR: &str = "constant";
pub const SQUARE_WAVE_TEST_STR: &str = "squareWave";
pub const LINEAR_SWEEP_TEST_STR: &str = "linearSweep";
pub const CHRONOAMP_TEST_STR: &str = "chronoamp";
pub const MULTISTEP_TEST_STR: &str = "multiStep";

// Serial command strings
//pub const RUN_TEST_STR: &str = "runTest";
//pub const STOP_TEST_STR: &str = "stopTest";
pub const GET_VOLT_STR: &str = "getVolt";
pub const SET_VOLT_STR: &str = "setVolt";
pub const GET_CURR_STR: &str = "getCurr";
pub const GET_REF_VOLT_STR: &str = "getRefVolt";
pub const GET_PARAM_STR: &str = "getParam";
pub const SET_PARAM_STR: &str = "setParam";
pub const GET_VOLT_RANGE_STR: &str = "getVoltRange";
pub const SET_VOLT_RANGE_STR: &str = "setVoltRange";
pub const GET_CURR_RANGE_STR: &str = "getCurrRange";
pub const SET_CURR_RANGE_STR: &str = "setCurrRange";
pub const GET_DEVICE_ID_STR: &str = "getDeviceId";
pub const SET_DEVICE_ID_STR: &str = "setDeviceId";
pub const GET_SAMPLE_PERIOD_STR: &str = "getSamplePeriod";
pub const SET_SAMPLE_PERIOD_STR: &str = "setSamplePeriod";
pub const GET_TEST_DONE_TIME_STR: &str = "getTestDoneTime";
pub const GET_TEST_NAMES_STR: &str = "getTestNames";
pub const GET_VERSION_STR: &str = "getVersion";
pub const GET_VARIANT_STR: &str = "getVariant";
//pub const SET_MUX_ENABLED_STR: &str = "setMuxEnabled";
//pub const GET_MUX_ENABLED_STR: &str = "getMuxEnabled";
//pub const SET_ENABLED_MUX_CHANNELS_STR: &str = "setEnabledMuxChannels";
//pub const GET_ENABLED_MUX_CHANNELS_STR: &str = "getEnabledMuxChannels";
//pub const GET_MUX_TEST_NAMES_STR: &str = "getMuxTestNames";
//pub const SET_MUX_REF_ELECT_CONNECTED_STR: &str = "setMuxRefElectConnected";
//pub const GET_MUX_REF_ELECT_CONNECTED_STR: &str = "getMuxRefElectConnected";
//pub const SET_MUX_CTR_ELECT_CONNECTED_STR: &str = "setMuxCtrElectConnected";
//pub const GET_MUX_CTR_ELECT_CONNECTED_STR: &str = "getMuxCtrElectConnected";
//pub const SET_MUX_WRK_ELECT_CONNECTED_STR: &str = "setMuxWrkElectConnected";
//pub const GET_MUX_WRK_ELECT_CONNECTED_STR: &str = "getMuxWrkElectConnected";
//pub const DISCONNECT_ALL_MUX_ELECT_STR: &str = "disconnectAllMuxElect";
//pub const SET_REF_ELECT_CONNECTED_STR: &str = "setRefElectConnected";
//pub const GET_REF_ELECT_CONNECTED_STR: &str = "getRefElectConnected";
//pub const SET_CTR_ELECT_CONNECTED_STR: &str = "setCtrElectConnected";
//pub const GET_CTR_ELECT_CONNECTED_STR: &str = "getCtrElectConnected";
//pub const SET_WRK_ELECT_CONNECTED_STR: &str = "setWrkElectConnected";
//pub const GET_WRK_ELECT_CONNECTED_STR: &str = "getWrkElectConnected";
pub const SET_ALL_ELECT_CONNECTED_STR: &str = "setAllElectConnected";
pub const GET_ALL_ELECT_CONNECTED_STR: &str = "getAllElectConnected";
//pub const SET_ELECT_AUTO_CONNECT_STR: &str = "setElectAutoConnect";
//pub const GET_ELECT_AUTO_CONNECT_STR: &str = "getElectAutoConnect";
//pub const SET_REF_ELECT_VOLT_RANGE_STR: &str = "setRefElectVoltRange";
//pub const GET_REF_ELECT_VOLT_RANGE_STR: &str = "getRefElectVoltRange";
pub const GET_HARDWARE_VERSION_STR: &str = "getHardwareVersion";

// Voltage Ranges
pub const VOLT_RANGES_8V: [&str; 4] = ["1V", "2V", "4V", "8V"];
pub const VOLT_RANGES_10V: [&str; 4] = ["1V", "2V", "5V", "10V"];

// Current Ranges
pub const CURR_RANGES_NANO: [&str; 4] = ["1uA", "10uA", "100nA", "60nA"];
pub const CURR_RANGES_MICRO: [&str; 4] = ["1uA", "10uA", "100uA", "1000uA"];
pub const CURR_RANGES_MILLI_10: [&str; 4] = ["10uA", "100uA", "1000uA", "10000uA"];
pub const CURR_RANGES_MILLI_24: [&str; 4] = ["100uA", "1000uA", "12000uA", "24000uA"];

//pub const COMMAND: &str = "command";
//pub const RESPONSE: &str = "response";
//pub const MESSAGE: &str = "message";
//pub const SUCCESS: &str = "success";
//pub const TEST: &str = "test";
//pub const PARAM: &str = "param";
//pub const TIME: &str = "t";
//pub const VOLT: &str = "v";
//pub const CURR: &str = "i";
//pub const CHAN: &str = "n";
//pub const REF_VOLT: &str = "r";
//pub const VOLT_RANGE: &str = "voltRange";
//pub const CURR_RANGE: &str = "currRange";
//pub const DEVICE_ID: &str = "deviceId";
//pub const SAMPLE_PERIOD: &str = "samplePeriod";
//pub const TEST_DONE_TIME: &str = "testDoneTime";
//pub const STEP: &str = "step";
//pub const TEST_NAMES: &str = "testNames";
//pub const VERSION: &str = "version";
//pub const VARIANT: &str = "variant";
//pub const MUX_ENABLED: &str = "muxEnabled";
//pub const MUX_CHANNEL: &str = "muxChannel";
//pub const CONNECTED: &str = "connected";
//pub const AUTO_CONNECT: &str = "autoConnect";
