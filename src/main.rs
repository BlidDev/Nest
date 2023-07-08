use std::{io::{BufReader, BufRead}, fs::File, env, collections::HashMap};

mod util;
mod structs;
mod macros;
mod ops;
mod canvas;
mod types;
pub mod commands;

use device_query::DeviceState;
use structs::{Stack, Globals, QueryW};
use util::*;
use commands::wrappers::*;
use crate::structs::{CommandQuery, GError};



fn main() -> Result<(), Box<dyn std::error::Error>>{

    let env = env::args();
    if env.len() != 2 {
        return gerr!("Error: file path not provided, usage: lang.exe path/to/script.glg");
    }
    let name = env.last().unwrap();

    
    let reader = BufReader::new(File::open(&name)?);

    let lines : Vec<String> = reader.lines().map(|l| l.unwrap().trim().to_string()).collect();

    let roots = find_root_scopes(&lines)?;


    let (roots, args_list) = {
        let mut v = HashMap::new();
        let mut an = HashMap::new();

        for (name, (start, end, args)) in roots.iter() {

            let temp = root_to_scope_tree(&lines, &(*start, *end))?;
            an.insert(name.clone(), args.clone());
            v.insert(name.clone(),temp);
        }
        (v, an)
    };

    //let main = roots.get("MAIN").expect("Error: Root scope [MAIN] not found");

    let _code : Vec<String> = lines.iter().filter(|l| 
        { 
            let l2 = (*l).clone();
            !(balanced_braces(&[l2], '[', ']').is_empty())
        }).map(|l| l.to_string()).collect();

    //println!("\nCall Order:\n");

    let mut query = CommandQuery::new();

    register_commands(&mut query);


    
    let mut glb = Globals {
        stack : Stack::default(),
        curr : 0,
        s : " ",
        device_state : DeviceState::new(),
        keys : vec![],
        canvas_should_close : false,
        args_list
    };
    let mut cnv = None;
    traverse_root_scope("MAIN", &roots, &QueryW(query.clone()), &mut glb,  &mut cnv)?;

    Ok(())
}


fn register_commands(query : &mut CommandQuery)
{
    commands![
        (*query),
        {
            set =>      (set_w, Some(2)),
            release =>  (release_w, Some(1)),
            reset =>    (reset_w, Some(0)),

            cal =>      (cal_w,Some(3)),
            op =>       (op_w,Some(3)),
            sqrt =>     (sqrt_w, Some(1)),

            post =>     (post_w,Some(0)),
            print =>    (print_w,None),
            format =>    (format_w,None),
            input =>    (input_w,None),
            inputcast =>(inputcast_w,None),

            if =>       (ifcommand_w,Some(3)),
            singleif=>       (singleif_w,Some(4)),
            while =>    (whilecommand_w,Some(1)),

            init => (init_w, Some(5)),
            set_clear => (set_clear_w, Some(3)),
            clear => (clear_w, Some(0)),
            display => (display_w, Some(0)),
            apply => (apply_pixels_w, Some(0)),
            set_pixel => (set_pixel_w, Some(5)),
            set_area => (set_area_w, Some(7)),
            get_pixel => (get_pixel_w, Some(5)),

            handle_input => (handle_input_w, Some(0)),
            key_pressed => (key_pressed_w, Some(1)),

            sleep => (sleep_w, Some(1)),
            rng => (rng_w, Some(2)),
            exit => (exit_w, Some(0)),

            run => (run_w, None),

            ovid => (ovid_w, Some(0)),
            dorbell => (dorbell_w, Some(1)),
            badduck => (badduck_w, Some(0)),
            zayther => (zayther_w, Some(0)),
            astro   => (astro_w, Some(0)),
            blid    => (blid_w, Some(0))
        }
    ];
}
