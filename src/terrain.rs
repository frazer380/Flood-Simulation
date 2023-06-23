use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;


// Get initial data such as # of rows/cols, and position X and Y of water source
fn get_data(data: &mut Vec<String>) -> (usize, usize) {
    if data.remove(0) == "local" {
        let _: u32 = data.remove(0).trim().parse().expect("Failed to convert rows to a useable size");
        let _: u32 = data.remove(0).trim().parse().expect("Failed to convert cols to a useable size");
        let _ = data.remove(0);
        let pos_x: usize = data.remove(0).trim().parse().expect("Failed to convert source pos_x to a useable size");
        let pos_y: usize = data.remove(0).trim().parse().expect("Failed to convert pos_y to a useable size");
        (pos_x, pos_y)
    } else {
        panic!("Terrain data is not verified.");
    }
}

// Create vector that will be used in calculating which tiles are flooded
// We go through each tile (elevation) and assign it with false (if the tile is flooded)
fn setup_terrain(terr: Vec<String>) -> Vec<Vec<(f64, bool)>> {
    let mut out: Vec<Vec<(f64, bool)>> = Vec::new();
    let mut contents: Vec<(f64, bool)> = Vec::new();
    for row in terr {
        let items = row.split_whitespace();
        for elevation in items {
            let elevation: f64 = elevation.trim().parse().expect("Could not convert elevation value.");
            contents.push((elevation, false));
        }
        out.push(contents.clone());
        contents.clear(); 
    }
    out
}

fn get_size(data: &Vec<Vec<(f64, bool)>>) -> (u32, u32) {
    ((data.len()-1) as u32, (data[0].len()-1) as u32)   
}

// Read terrain from file and pas it through to get_data and return the result from setup_terrain
pub fn read_terrain(path: &str) -> io::Result<(Vec<Vec<(f64, bool)>>, (u32, u32), (usize, usize))> {
    let file = File::open(path)?;
    let file = BufReader::new(file);

    let mut data: Vec<String> = Vec::new();

    for line in file.lines() {
        data.push(line.unwrap());
    }


    let source = get_data(&mut data);
    let terrain = setup_terrain(data);
    let size = get_size(&terrain);

    Ok((terrain, size, source))
}

