const PACKET_SIZE: usize = 8;
const VID: u16 = 0x2123;
const PID: u16 = 0x1010;

#[derive(PartialEq, Clone)]
pub enum Commands {
    DOWN = 0x01,
    UP = 0x02,
    LEFT = 0x04,
    RIGHT = 0x08,
    FIRE = 0x10,
    STOP = 0x20,
}

pub struct Launcher {
    dev: hidapi::HidDevice,
    last_cmd: Commands,
}

impl Launcher {
    pub fn new() -> Self {
        let api = match hidapi::HidApi::new() {
            Ok(api) => api,
            Err(_) => panic!("Can't init hidapi!"),
        };

        let device = match api.open(VID, PID) {
            Ok(dev) => dev,
            Err(_) => panic!("Can't find missile launcher! Is it plugged in?"),
        };

        device.send_feature_report(&[0x00; PACKET_SIZE]);

        Launcher { dev: device, last_cmd: Commands::STOP }
    }

    pub fn execute_command(&mut self, mut cmd: Commands) {
        if cmd == self.last_cmd {
            cmd = Commands::STOP;
        }

        self.last_cmd = cmd.clone();

        let payload = [0x00, 0x02, cmd as u8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        self.dev.write(&payload).expect("Can't write payload to device!");
    }
}