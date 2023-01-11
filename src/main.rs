mod curses; use curses::*;
mod filter; use filter::*;
mod state; use state::*;
use std::env;    
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut default_settings: Vec<usize> =vec![50,100,7,5,20,3000];
    for i in 0..default_settings.len() {
        if i < args.len() - 1 {
            let parsed = match args[i+1].parse::<usize>() {
                Ok(u) => u,
                Err(_) => 0,
            };
            if parsed > 0 && i < default_settings.len() {
                default_settings[i] = parsed;
            }
        }
    }
    let rows = default_settings[0];
    let cols = default_settings[1];
    let filters = default_settings[2];
    let span = default_settings[3] as i32;
    let flux = default_settings[4];
    let updates = default_settings[5];
    let right = 10;
    let down = 10;
    clear_screen();
    hide_cursor();
    let mut filter_system = simple_random_filters(filters,span,span,2.0);
    let mut red_state = random_state(rows, cols);
    let mut green_state = random_state(rows, cols);
    let mut blue_state = random_state(rows, cols);
    cursor_to(down,right);
    print!("fflo {} {} {} {} {} {}", rows, cols, filters, span, flux, updates);
    loop {
        for _ in 0..updates {
            let row =  rand::random::<usize>() % rows;
            let col =  rand::random::<usize>() % cols;
            for i in 0..filter_system.len() {
                match i % 3 {
                    0 => filter_state_mutate_cell(&filter_system[i], &mut red_state, row, col, rows, cols),
                    1 => filter_state_mutate_cell(&filter_system[i], &mut green_state, row, col, rows, cols),
                    2 => filter_state_mutate_cell(&filter_system[i], &mut blue_state, row, col, rows, cols),
                    _ => (),
                }
            }
        }
        display_color(&red_state,&green_state,&blue_state, rows, cols, down, right);
        if rand::random::<usize>()%1000 < flux {
            let which = rand::random::<usize>()%filters;
            filter_system[which] = random_filter(span, span,0.4 + 2.8*rand::random::<f64>() );
            //filter_system = simple_random_filters(filters,span,span,0.4 + 2.8*rand::random::<f64>());
        }
    }
}
