const PACKET_SIZE: usize = 8;

pub enum Commands {
    DOWN = 0x01,
    UP = 0x02,
    LEFT = 0x04,
    RIGHT = 0x08,
    FIRE = 0x10,
    STOP = 0x20,
}

pub struct Launcher {
    dev: hidapi::HidDevice
}

impl Launcher {
    pub fn new() -> Self {
        let api = match hidapi::HidApi::new() {
            Ok(api) => api,
            Err(_) => panic!("Can't init hidapi!"),
        };

        let (vid, pid) = (0x2123, 0x1010);
        let device = match api.open(vid, pid) {
            Ok(dev) => dev,
            Err(_) => panic!("Can't find missile launcher! Is it plugged in?"),
        };

        device.send_feature_report(&[0x00; PACKET_SIZE]);

        println!("{:#?}", device.get_manufacturer_string().unwrap());
        println!("{:#?}", device.get_product_string().unwrap());

        Launcher { dev: device }
    }

    pub fn execute_command(&self, cmd: Commands) {
        let payload = [0x00, 0x02, cmd as u8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        self.dev.write(&payload).expect("Can't write payload to device!");
    }
}