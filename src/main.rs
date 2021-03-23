
mod disjoint_set;
mod options;
use structopt::StructOpt;
use rand::prelude::SliceRandom;
use disjoint_set::DisjointSet;
use std::ops::Index;
use rand::Rng;
use std::path::PathBuf;

fn main() {
    //get options
    let opt = options::Opt::from_args();
    let (w, h) = (opt.width as usize, opt.height as usize);

    //init cells
    let mut walls = vec![true; 2 * w * h + w + h];
    let mut cells = DisjointSet::with_size(w * h);
    let mut rng = rand::thread_rng();

    //remove walls
    let mut indexes = (0..cells.len()).collect::<Vec<usize>>();
    indexes.shuffle(&mut rng);
    for i in &indexes {
        remove_wall(*i, &mut cells, &mut walls, w, h, &mut rng);
    }
    indexes.shuffle(&mut rng);
    for i in indexes {
        remove_wall(i, &mut cells, &mut walls, w, h, &mut rng);
    }

    //open openings
    let opening = (rng.gen::<usize>() % h) * w;
    walls[opening + w * h + w + opening / w] = false;
    let opening = (rng.gen::<usize>() % h) * w + (w - 1);
    walls[opening + w * h + w + opening / w + 1] = false;

    let maze = render(&walls, w, h);

    println!("{}", maze);
    assert_eq!(cells.size(0) as usize, cells.len());
}

fn remove_wall<R>(cell:usize, cells:&mut DisjointSet, walls:&mut Vec<bool>, w:usize, h:usize, rng:&mut R)
    where R: Rng {
    let mut cell_walls = Vec::with_capacity(4);
    if !(cell < w) { //northern border
        cell_walls.push(0);
    }
    if !((cell + 1) % w == 0) { //eastern border
        cell_walls.push(1);
    }
    if !(cell + 1 > w * h - w) { //southern border
        cell_walls.push(2);
    }
    if !((cell + 1) % w == 1) { //western border
        cell_walls.push(3)
    }
    cell_walls.shuffle(rng);

    for relative_wall in cell_walls {
        let (other_cell, wall) = match relative_wall {
            0 => (cell - w, cell),
            1 => (cell + 1, cell + w * h + w + cell / w + 1),
            2 => (cell + w, cell + w),
            3 => (cell - 1, cell + w * h + w + cell / w),
            _ => panic!()
        };
        if cells.try_union(cell, other_cell) {
            walls[wall] = false;
            break;
        }
    }
}

fn render(walls:&Vec<bool>, w:usize, h:usize) -> String {
    let mut maze = String::with_capacity(walls.len() * 2);
    //the top
    maze.push(' ');
    for s in walls.iter().take(w).map(|wall| if *wall { "_ " } else { "  " }) {
        maze.push_str(s);
    }
    maze.push('\n');

    //the rest
    let mut cell = 0;
    for row in 0..h {
        if walls[cell + w * h + w + cell / w] {
            maze.push('|');
        } else {
            maze.push(' ');
        }
        while cell < w * (row + 1) {
            if walls[cell + w] {
                maze.push('_');
            } else {
                maze.push(' ');
            }
            if walls[cell + w * h + w + cell / w + 1] {
                maze.push('|');
            } else {
                maze.push(' ');
            }
            cell += 1;
        }
        maze.push('\n');
    }
    maze
}