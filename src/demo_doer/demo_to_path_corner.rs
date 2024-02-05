use crate::open_demo;

use super::*;
use std::{fs::File, io::Write, path::Path};

pub fn demo_to_path_corner(demo: &Demo, output_name: &str, num_path_corners: usize) {
    let ghost = get_ghost::demo::demo_ghost_parse("path_corner", demo, 0., false);
    let mut mapfile = File::create(format!("{}.map", output_name)).unwrap();

    let total_frames = ghost.frames.len() as f32;
    let frames_per_path_corner = total_frames / num_path_corners as f32;

    let mut current_frame: f32 = 0.;

    let mut corner_counter = 0;

    // TODO: adjust the loop to exclude the target of the last iteration since there is no next path_corner
    for _ in 0..num_path_corners {
        let frame_index = current_frame.round() as usize;
        if let Some(frame) = ghost.frames.get(frame_index) {
            writeln!(
                mapfile,
r#"{{
"classname" "path_corner"
"yaw_speed" "0" 
"speed" "0"
"wait" "0"
"angles" "0 0 0"
"targetname" "path_frame_{}"
"target" "path_frame_{}"
"origin" "{} {} {}"
}}"#,
                corner_counter,
                corner_counter + 1,
                frame.origin[0],
                frame.origin[1],
                frame.origin[2]
            )
            .unwrap();
            corner_counter += 1;
        }

        current_frame += frames_per_path_corner;
    }
}

pub fn demo_to_path_corner_cli() {
    use std::env;

    let help = || {
        println!(
            "\
MAP file will be generated based on the demo file name.
If the demo file is 'bkz_goldbhop.dem', the output file will be 'bkz_goldbhop.map'

Usage:
  ./binary <path to demo> <num path_corner>
  
Example:
  ./binary <bkz_goldbhop.dem> <20>"
        )
    };

    let wrap = |args: Vec<String>| {
        if args.len() == 3 {
            let demo_file_name = Path::new(&args[1]);
            let demo = open_demo!(demo_file_name);
            let map_name = demo_file_name.file_stem().unwrap().to_str().unwrap();
            let num_path_corners: usize = args[2].parse().unwrap();
            demo_to_path_corner(&demo, &map_name, num_path_corners);
            println!("Map with path_corner successfully generated: {}", map_name);
        } else {
            help();
        }
    };

    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => wrap(args),
        _ => help(),
    }
}
