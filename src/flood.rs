fn neighbors(terr: &Vec<Vec<(f64, bool)>>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    let pos_y = pos.0;
    let pos_x = pos.1;

    if pos_y > 0 {
        neighbors.push((pos_y-1, pos_x));
    }

    if pos_x > 0 {
        neighbors.push((pos_y, pos_x-1));
    }


    if pos_y < terr.len()-2 {
        neighbors.push((pos_y+1, pos_x))
    }

    if pos_x < terr[0].len()-2 {
        neighbors.push((pos_y, pos_x+1))
    }


    neighbors 
}

pub fn flood(terr: &mut Vec<Vec<(f64, bool)>>, source: (usize, usize), height: f64) {
    let mut queue: Vec<(usize, usize)> = Vec::new();
    println!("Starting Source: {}/{} :: {}", source.0, source.1, terr[source.0][source.1].0);
    terr[source.0][source.1].1 = true;
    queue.push(source);
    
    while queue.len() > 0 {
        let pos = queue.remove(0);
        for neighbor in neighbors(&terr, pos) {
            let data: (f64, bool) = terr[neighbor.0][neighbor.1];
            let elevation = data.0;
            let flooded = data.1;
            if elevation <= height && !flooded {
                terr[neighbor.0][neighbor.1].1 = true;
                queue.push(neighbor);
            }
        }
    }
}