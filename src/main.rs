use base_skewb::Skewb;
use crate::base_skewb::BaseCube;

mod cube_bfs;
mod base_skewb;

fn main() {
    let mut t = Skewb::construct();

    println!("{:?}", t);

    let ov = cube_bfs::bfs::<Skewb>();

    // println!("{:?}", ov);
    for (index, value) in ov.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }

    // let (t,_) = t.do_scramble("L R L R F R' F' B' L'".to_string());


    // for i in 0..4{
    //     for i in 0..2{
    //         t = t.fp().l();
    //     }

    //     println!("first {:?}", t);
    //     for i in 0..2
    //         {
    //             t = t.l().bp();
    //         }
        
        
    //     println!("second {:?}", t);
    // }
    // t = t.lp();
    // println!("{:?}", t);

    // t = t.l();
    // println!("{:?}", t);

    // t = t.r();
    // println!("{:?}", t);
}


