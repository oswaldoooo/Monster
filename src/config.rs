use serde::Deserialize;
#[derive(Default,Deserialize)]
pub struct Config{
    pub port:u16,
    pub match_interval:Option<u32>,//ms
    pub match_queue_size:Option<u32>
}
impl Config{
    pub fn new()->Self{
        Self::default()
    }
}