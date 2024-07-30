use md5;


pub fn mine_santacoin() {
    let secret_key = "bgvyzdsv";
    let mut appended = 1usize;
    loop {
        let s = format!("{secret_key}{appended}");
        let hash = md5::compute(s);
        let hex_representation = format!("{hash:x}");
        if hex_representation[0..6] == *"000000".to_owned() {
            println!("\n============{appended} = {hex_representation}============\n");
            break;
        }
        if appended % 10000 == 0 {
            println!("LN:{appended} - HEX:{hex_representation}");
        }
        appended += 1;
        
    } 
}