const MAX_HIGH_THRESHOLD: u16 = 3000;
const MIN_HIGH_THRESHOLD: u16 = 2300;
const MAX_LOW_THRESHOLD: u16 = 2000;
const MIN_LOW_THRESHOLD: u16 = 1300;

pub enum Conductivity {
    High,
    Low
}

pub fn convert (level: u16) -> Option<bool> {
    if level < MAX_HIGH_THRESHOLD && level > MIN_HIGH_THRESHOLD {
        Some(true)
    } else if level < MAX_LOW_THRESHOLD && level > MIN_LOW_THRESHOLD {
        Some(false)
    } else {
        None
    }
    
}