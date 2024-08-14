use base_skewb::Skewb;
use base_skewb::SkewbLayer;
use crate::base_skewb::BaseCube;

mod cube_bfs;
mod base_skewb;

fn main() {
    // Normal skewb optimal distribution
    let mut t = Skewb::construct();

    println!("{:?}", t);

    
    // let ov = cube_bfs::bfs::<Skewb>();
    
    // // println!("{:?}", ov);
    // for (index, value) in ov.iter().enumerate() {
        //     println!("Index: {}, Value: {}", index, value);
        // }
        
        let t = t.do_scramble("L R L R F R' F' B' L'".to_string());
        // let t = t.do_scramble("L".to_string());
        
        // let a = t.get_colours();
        // println!("{:?}",a);
    // // Layer distribution

    let mut t = SkewbLayer::construct();

    println!("{:?}", t);

     let ov = cube_bfs::bfs::<SkewbLayer>();

    // println!("{:?}", ov);
    for (index, value) in ov.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }


}


