use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::{File, self};

#[derive(Debug)]
pub struct Terrain {
    pub terrain: Vec<Vec<(f64, bool)>>,
    pub sources: Vec<(usize, usize)>,
    pub range: (f64, f64),
    pub size: (u32, u32),
}

fn get_data(data: &mut Vec<String>) -> Vec<(usize, usize)> {
    if data.remove(0).trim() == "local" {
        println!("Terrain file verified.");
        let num_sources: u8 = data.remove(0).trim().parse().expect("Failed to convert the number of water sources. Perhaps you made too many?");
        if num_sources > 0 {
            let mut sources: Vec<(usize, usize)> = Vec::new();
            for i in 0..(num_sources*2) {
                let source = &data[i as usize];
                let source: Vec<String> = source.split_whitespace().map(str::to_string).collect();
                let x: usize = source[0].trim().parse().expect("Failure to convert X position.");
                let y: usize = source[1].trim().parse().expect("Failure to convert Y position.");
                sources.push((x, y));
                data.remove(0);
            }
            sources
        } else {
            panic!("No water sources");
        }
    } else {
        panic!("Failed to verify terrain");
    }
}

fn setup_terrain(data: Vec<String>) -> (Vec<Vec<(f64, bool)>>, (f64, f64)) {
    let mut terrain: Vec<Vec<(f64, bool)>> = Vec::new();
    let mut min: f64 = 0.0;
    let mut max: f64 = 0.0;
    for row in data {
        let row = row.split_whitespace();
        let mut elevations: Vec<(f64, bool)> = Vec::new();
        for elevation in row {
            let elevation: f64 = elevation.trim().parse().expect("Could not convert terrain data to an f64.");
            if elevation < min {
                min = elevation;
            }
            if elevation > max {
                max = elevation;
            }
            elevations.push((elevation, false));
        }
        terrain.push(elevations);
    }
    (terrain, (max, min))
}

fn get_size(data: &Vec<Vec<(f64, bool)>>) -> (u32, u32) {
    ((data.len()-1) as u32, (data[0].len()-1) as u32)   
}

// Read terrain from file and pas it through to get_data and return the result from setup_terrain
pub fn read_terrain(path: &str) -> io::Result<Terrain> {
    let file = File::open(path)?;
    let file = BufReader::new(file);
    let mut data: Vec<String> = Vec::new();
    for line in file.lines() {
        data.push(line.unwrap());
    }

    let sources = get_data(&mut data);
    let (terrain, range) = setup_terrain(data);
    let size = get_size(&terrain);

    let terr = Terrain {
        terrain,
        sources,
        range,
        size
    };

    Ok(terr)
}

pub fn get_terrain_files(path: &str) -> Vec<String> {
    let mut terrains: Vec<String> = Vec::new();
    let paths = fs::read_dir(path).unwrap();
    for (i, path) in paths.enumerate() {
        let terr_path = path.unwrap().path().display().to_string();
        println!("{}. {:?}", i+1, terr_path);
        terrains.push(terr_path);
    }
    terrains
}




