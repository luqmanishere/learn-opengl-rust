use _1_getting_started::{main_1_4_1, main_1_4_2};

use crate::_1_getting_started::{
    main_1_1_1, main_1_2_1, main_1_2_2, main_1_2_3, main_1_2_4, main_1_2_5, main_1_3_1, main_1_3_2,
    main_1_3_3,
};

// this is based on the web version
mod _1_getting_started;
mod shaders;

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Call with the number of the tutorial, eg:. 1_1_2 for _1_2_hello_window_clear.rs");
        std::process::exit(1);
    }

    let tutorial_id = &args[1];
    match tutorial_id.as_str() {
        "1_1_1" => main_1_1_1(),
        "1_2_1" => main_1_2_1(),
        "1_2_2" => main_1_2_2(),
        "1_2_3" => main_1_2_3(),
        "1_2_4" => main_1_2_4(),
        "1_2_5" => main_1_2_5(),
        "1_3_1" => main_1_3_1(),
        "1_3_2" => main_1_3_2(),
        "1_3_3" => main_1_3_3(),
        "1_4_1" => main_1_4_1(),
        "1_4_2" => main_1_4_2(),
        _ => {
            println!("Unknown chapter id");
            Ok(())
        }
    }
}
