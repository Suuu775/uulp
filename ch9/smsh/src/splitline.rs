pub mod splitline{
    pub fn splitline(line:String)->Vec<String>{
        line
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
    }
}