pub fn prior_work_penalty(anvil_use_count: u8) -> u8 {
    2_u32.pow(anvil_use_count as u32) as u8
}
