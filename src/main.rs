#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui_plot::{Bar, BarChart, Legend, Plot};

#[derive(PartialEq)]
enum Page {
    Visualizer,
    SortSelect
}

impl ToString for Page {
    fn to_string(&self) -> String {
        match self {
            Page::Visualizer => "Visualizer".to_string(),
            Page::SortSelect => "Sorting Methods".to_string(),
        }
    }
}

pub struct SortVis {
    cur_page: Page,
    sort_arr: Vec<f64>
}

impl Default for SortVis {
    fn default() -> Self {
        Self {
            cur_page: Page::Visualizer,
            sort_arr: (0..1000).map(|v| { v as f64 }).collect()
        }
    }
}

fn shuffle_array(arr: &mut Vec<f64>) {
    // shuffle
}

impl eframe::App for SortVis {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_menu").show(ctx, |ui|{
            ui.selectable_value(&mut self.cur_page, Page::Visualizer, "Visualizer");
            ui.selectable_value(&mut self.cur_page, Page::SortSelect, "Sorting Methods");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.cur_page.to_string());
            ui.separator();
            match self.cur_page {
                Page::Visualizer => {
                    let bar_chart = BarChart::new("Sorting Chart", self.sort_arr.iter().enumerate().map(|(i, v)| { Bar::new(i as f64, *v) }).collect());

                    Plot::new("Sorting Chart")
                        .legend(Legend::default())
                        .allow_boxed_zoom(false)
                        .allow_drag(false)
                        .allow_zoom(false)
                        .allow_scroll(false)
                        .show(ui, |chart_ui| { chart_ui.bar_chart(bar_chart); });
                },
                Page::SortSelect => {

                }
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Sorting Visualizer",
        eframe::NativeOptions::default(),
        Box::new(|_cc| {
            Ok(Box::new(SortVis::default()))
        }),
    )
}
