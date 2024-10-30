use std::{f32::consts::PI, vec, collections::HashMap};
use hound::{self, WavSpec};

const _NUM_FILES: i32 = 10;
const SAMPLERATE: u32 = 44100;

//TODO: implement multichannel functionality so that itteration becomes possible (generating multiple files, etc)
#[derive(Debug)]
struct Buffer {
    length: u32,
    data: Vec<f32>
} 

fn main(){
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLERATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut cache:HashMap<String, Buffer> = HashMap::new();


    let modulator_freq = constant(&mut cache, "modulator_freq", 44100, 220.0);
    let modulator = sine(&mut cache, "modulator", modulator_freq);
    let modulator_scaled = scale(&mut cache, "modulator_scaled", modulator, -1.0, 1.0, 200.0, 400.0);
    let carrier = sine(&mut cache, "carrier", modulator_scaled);
    
    make_file(&mut cache, carrier, spec, "test")

}

// fn ramp(len: f32, start: f32, end: f32) -> Vec<f32>{}

fn constant<'a>(cache: &mut HashMap<String, Buffer>, name: &'a str, len: u32, val: f32) -> &'a str {
    let data = vec![val; len as usize];
    
    let buffer = Buffer {
        length: len,
        data: data
    };
    cache.insert(name.to_string(), buffer);
    return name;
}

fn scale<'a>(cache: &mut HashMap<String, Buffer>, name: &'a str, input: &'a str, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> &'a str {
    let mut output: Vec<f32> = Vec::new();

    let mut i: f32 = 0.0;
    // println!("{}");
    
    for val in cache[input].data.clone() {
        let sample = (val - in_min)*((out_max-out_min)/(in_max-in_min)) + out_min;
        i = i + val;
        output.push(sample);
    }

    let buffer = Buffer {
        length: cache[input].length,
        data: output
    };
    cache.insert(name.to_string(), buffer);
    return name;
}

fn sine<'a>(cache: &mut HashMap<String, Buffer>, name: &'a str, freq: &'a str) -> &'a str {
    let mut output: Vec<f32> = Vec::new();

    let mut i = 0.0;
    // println!("{}");
    
    for val in cache[freq].data.clone() {
        let sample: f32 = ((i * 2.0 * PI) / (SAMPLERATE as f32)).sin();
        i += val;
        output.push(sample);
    }

    let buffer = Buffer {
        length: cache[freq].length,
        data: output
    };
    cache.insert(name.to_string(), buffer);
    return name;
}

fn make_file(cache: &mut HashMap<String, Buffer>, input: &str, spec:WavSpec, file_name: &str) {

    let mut writer = hound::WavWriter::create(format!("{}{}",file_name, ".wav"), spec).unwrap();
    for s in cache[input].data.clone() {
        let amplitude = i16::MAX as f32;
        writer.write_sample((s * amplitude) as i16).unwrap();
    }      
}
