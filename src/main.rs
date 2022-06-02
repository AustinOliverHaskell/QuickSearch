use imgui::*;
use imgui::ImStr;

mod support;
mod file_search;

use file_search::FileSearch;

fn main() {
    let mut search_string: ImString = ImString ::with_capacity(300);
    let search_results: Vec<String> = (0..10000).map(|x| format!("Line {}", x)).collect();

    const WINDOW_WIDTH: u32 = 1080;
    const WINDOW_HEIGHT: u32 = 720;

    let system = support::init(file!(), WINDOW_WIDTH, WINDOW_HEIGHT);
    system.main_loop(move |_, ui| {

        Window::new(im_str!("Slicer Start"))
            .flags(WindowFlags::NO_RESIZE | WindowFlags::NO_TITLE_BAR | WindowFlags::NO_BACKGROUND)
            .size([WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32], Condition::Always)
            .position([0.0, 0.0], Condition::Always)
            .position_pivot([0.0, 0.0])
            .build(ui, || {
                ui.input_text(im_str!("Search String"), &mut search_string).build();

                let mut clipper = imgui::ListClipper::new(search_results.len() as i32)
                    .items_height(ui.current_font_size())
                    .begin(ui);
                while clipper.step() {
                    for row_num in clipper.display_start()..clipper.display_end() {
                        ui.button(&search_results[row_num as usize]);
                    }
                }
            });

        println!("{:}", search_string);

        // FileSearch::search(search_string.to_str());
    });
}