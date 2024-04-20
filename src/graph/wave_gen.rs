use std::{fs::File, io::Write};






pub struct WaveGen{
    length: u32,
    factor:u32,
    offset:u32
}

impl WaveGen {
    pub fn new(factor: u32,offset:u32) -> WaveGen{
        WaveGen{
            length: 10000,
            factor,
            offset,
        }
    }

    pub fn generate(&mut self) -> Vec<f32>{
        let mut wave = Vec::new();
        for i in self.offset..(self.length + self.offset){
            wave.push((i as f32/1000.0).sin() * self.factor as f32 + self.factor as f32);
        }
        wave
    }
}


pub fn gen_wave_file(){
    let mut data_file = File::create("test_wave.txt").expect("creation failed");
    let wave1 = WaveGen::new(300,40).generate();
    let wave2 = WaveGen::new(300,100).generate();

    for (w1,w2) in wave1.iter().zip(wave2.iter()){

        data_file.write("0.0,0.0,0.0,0.0,0.0,0.0,".as_bytes()).unwrap();
        data_file.write((w1.to_string() + ","+ &w2.to_string()).as_bytes()).unwrap();
        data_file.write(",0,0".as_bytes()).unwrap();

        data_file.write("\n".as_bytes()).unwrap();
    }


}



