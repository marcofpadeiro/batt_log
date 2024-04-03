const CAPACITY_PATH: &str = "/sys/class/power_supply/BAT0/capacity";
const STATUS_PATH: &str = "/sys/class/power_supply/BAT0/status";
const POWERDRAW_PATH: &str = "/sys/class/power_supply/BAT0/power_now";

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    Charging,
    Discharging,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Status::Charging => write!(f, "Charging"),
            Status::Discharging => write!(f, "Discharging"),
        }
    }
}

pub struct Power {
    pub capacity: u32,
    pub status: Status,
    pub power_draw: u32,
}

impl Power {
    pub fn default() -> Self {
        Self {
            capacity: 0,
            status: Status::Discharging,
            power_draw: 0,
        }
    }

    pub fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let capacity = std::fs::read_to_string(CAPACITY_PATH)?.trim().parse()?;
        let status = match std::fs::read_to_string(STATUS_PATH).unwrap().trim() {
            "Charging" => Status::Charging,
            "Discharging" => Status::Discharging,
            _ => return Err("Unknown status".into()),
        };
        let power_draw = std::fs::read_to_string(POWERDRAW_PATH)?.trim().parse()?;

        self.capacity = capacity;
        self.status = status;
        self.power_draw = power_draw;

        Ok(())
    }
}
