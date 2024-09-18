
mod createreg;
mod writereg;
mod deletereg;

pub fn start_mreg(data: Vec<&str>){
    if data.len() <= 1 {
        println!("usage: mreg [type] (args for all types)");
        return
    }
    match data[1].to_lowercase().as_str() {
        "-d" => {
            deletereg::delete_reg(data);
        }
        "-w" => {
            writereg::write_registry(data);
        }
        "-c" => {
            createreg::create_registry(data);
        }
        _ => {
            println!("incorrect type");
        }
    }
}
