

use std::fs::File;
use std::{io, env};
use std::io::{BufReader, BufRead, Error, ErrorKind};

fn read_module_mass(path: &str) -> Result<Vec<i32>, io::Error>{
    let curr_dir = env::current_dir()?;
    println!("The current directory is {}", curr_dir.display());
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut v = Vec::new();

    for line in br.lines(){
        let line = line?;
        let n:i32 = line.trim().parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        v.push(n);
    }
    Ok(v)
}

fn part1(all_masses: Vec<i32>) {
    let mut fuel_for_all_modules= 0;
    for mass in all_masses.iter() {
        let fuel_required = (*mass as f32/ 3.0).floor() as i32 - 2;
        fuel_for_all_modules += fuel_required;
    }
    println!(" PART1 :: Fuel required for all modules  is {}", fuel_for_all_modules);
}

fn part2(all_masses: Vec<i32>) {
    let mut fuel_for_all_modules= 0;
    for mass in all_masses.iter() {
        let mut fuel_required = (*mass as f32/ 3.0).floor() as i32 - 2;
        while fuel_required >= 0 {
            fuel_for_all_modules += fuel_required;
            fuel_required =  (fuel_required as f32/ 3.0).floor() as i32 - 2;
        }
    }
    println!(" PART2 :: Fuel required for all modules  is {}", fuel_for_all_modules);
}

fn main() {
    let all_masses = read_module_mass("src/input/module_mass.txt").unwrap();
    part1(all_masses.clone());
    part2(all_masses.clone());
}
