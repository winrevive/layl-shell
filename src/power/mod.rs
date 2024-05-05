
mod normal;
mod force;
mod powerperms;




pub fn power_management(data: Vec<&str>){
    if data.len() <= 1 {
        println!("usage: power [type] [isForce]");
        return;
    }
    if powerperms::give_power_permissions() == false {
        println!("Failed Giving Permission To Get Power Control Of Your Computer");
        return
    }
    if data.len() > 2 {
        match data[2].to_lowercase().as_str() {
            "1" => {
                force::force_power_procedures(data);
            }
            _ => {
                normal::normal_power_procedures(data);
            }
        }
    }
    else {
        normal::normal_power_procedures(data);
    }
}