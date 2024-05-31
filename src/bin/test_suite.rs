/*
  Test Suite - basically runs basic tests on different sub-systems to see if everything is working as intended
  Due to the nature of Bevy and ECS, the tests will be fairly limited. General idea is to just make sure nothing breaks.
  One other thing to note - this is intended to be limited to only things I can't form a good unit test on (EG, weird runtime stuff)
*/

#![allow(unused)]

mod prelude {
  pub use babel_proto::data_structs::*;
}

use prelude::*;

fn main(){
    print!("Do Cargo Test instead of Cargo Build - wheee.\n")   
}


