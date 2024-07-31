use std::collections::VecDeque;
use std::hash::Hash;
// use std::collections::HashSet;
// use std::collections::HashMap;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;
use crate::base_skewb;
// mod base_cube;
// use base_cube::BaseCube;
use crate::base_skewb::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)] 
pub struct CubeBFSHelper<T: BaseCube>{
    pub cube: T,
    pub depth: usize,
    pub moves: [u8;20],
}

impl<T: BaseCube> CubeBFSHelper <T> {
    fn apply_move(mut self, movee:u8) -> Self{
        self.cube = self.cube.perform_move(movee);
        self.moves[self.depth] = movee;
        self.depth+=1;
        self
    }

    fn new() -> Self{
        Self { cube: T::new(), depth: 0, moves: [0;20] }
    }
}

pub fn bfs<T: BaseCube + Eq + PartialEq + Hash + Clone + Copy + std::fmt::Debug  >() -> [usize;20]{

    let start_cube = CubeBFSHelper::<T>::new();
    let mut solutions: FxHashMap<T,CubeBFSHelper<T>> = FxHashMap::default();
    let moves:Vec<u8> = vec![1, 2, 5, 6, 21, 22, 25, 26];
    let mut q:VecDeque<CubeBFSHelper<T>> = VecDeque::from(vec![start_cube]);
    let mut overview = [0;20];

    let mut i = 0;
    while let Some(nc) = q.pop_front(){
        i+=1;
        if i%1_000_000 == 0{
            println!("{},{}",i, solutions.len())
        }
        if !solutions.contains_key(&nc.cube) & !solutions.contains_key(&nc.cube.y2()){ // The very initial state of length 0
            overview[nc.depth as usize] +=1;
            solutions.insert(nc.cube,nc.clone());
        }

        for movee in &moves{
            let new_state: &mut CubeBFSHelper<T> = &mut nc.apply_move(*movee);
            if !solutions.contains_key(&new_state.cube) & !solutions.contains_key(&new_state.cube.y2()){
                // if new_state.depth == 7{
                //     println!("{:?} {:?} {:?}",nc.cube,new_state.cube, new_state.moves);
                // }
                overview[new_state.depth as usize] +=1;
                solutions.insert(new_state.cube,new_state.clone());
                q.push_back(*new_state);
            }
        }
    }

    println!("{}",solutions.len());

    return overview
}

fn int_moves_to_str(moves:Vec<u8>) -> String{
    let mut str_solution = String::new();
    for imove in &moves{
        str_solution.push_str(match imove {
            1 => "R ", 
            2 => "L ", 
            5 => "F ", 
            6 => "B ", 
            21 => "R' ", 
            22 => "L' ", 
            25 => "F' ", 
            26 => "B' ", 
            0 => "",
                _ => unreachable!()
            })
    }
    str_solution
}

fn invert_moves(moves:[u8;20])-> Vec<u8>{
    let mut reverse_sol = Vec::new();
    for movee in moves.iter().rev(){
        if *movee >0{
            if *movee < 10{
                reverse_sol.push(*movee + 20);
            }
            else if *movee > 20{
                reverse_sol.push(*movee - 20);
            }
            else{
                reverse_sol.push(*movee);
            }
        }
    }
    reverse_sol
}

