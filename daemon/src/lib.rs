use battery::{Battery, Manager};

pub fn refresh_battery_info(
    manager: &mut Manager,
    batteries: &mut Vec<Battery>,
) -> Result<(), battery::Error> {
    for battery in batteries.iter_mut() {
        manager.refresh(battery)?;
    }
    Ok(())
}

pub fn get_current_battery(batteries: &[Battery]) -> u32 {
    batteries
        .iter()
        .map(|b| (b.state_of_charge().value as f32 * 100.0).round() as u32)
        .sum()
}

pub fn get_powerdraw(batteries: &[Battery]) -> f32 {
    batteries.iter().map(|b| b.energy_rate().value as f32).sum()
}
