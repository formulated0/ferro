use eframe::egui;

const WINDOW_WIDTH: f32 = 400.0;
const WINDOW_HEIGHT: f32 = 400.0;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_decorations(false)
            .with_taskbar(false)
            .with_always_on_top(),
        ..Default::default()
    };

    eframe::run_native(
        "app",
        native_options,
        Box::new(|_cc| Ok(Box::new(FerroApp::default()))),
    )
}

pub struct FerroApp {
    search_query: String,
    all_apps: Vec<String>,
    selected_index: usize,
    centered: bool,
}

impl Default for FerroApp {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            all_apps: vec![
                "Calculator".to_string(),
                "Chrome".to_string(),
                "Firefox".to_string(),
                "Notepad".to_string(),
                "Spotify".to_string(),
                "Steam".to_string(),
                "Terminal".to_string(),
                "VS Code".to_string(),
                "Visual Studio".to_string(),
                "VLC Media Player".to_string(),
                "Word".to_string(),
                "Xbox".to_string(),
                "Zoom".to_string(),
            ],
            selected_index: 0,
            centered: false,
        }
    }
}

impl eframe::App for FerroApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		// center window
		if !self.centered {
            if let Some(monitor_size) = ctx.input(|i| i.viewport().monitor_size) {
                if monitor_size.x > 0.0 && monitor_size.y > 0.0 {
                    self.centered = true;
                    let pos = egui::Pos2::new(
                        (monitor_size.x - WINDOW_WIDTH) / 2.0,
                        (monitor_size.y - WINDOW_HEIGHT) / 2.0,
                    );
                    ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(pos));
                }
            }
        }

        // filter apps
        let filtered_apps: Vec<String> = self.all_apps
            .iter()
            .filter(|app| app.to_lowercase().contains(&self.search_query.to_lowercase()))
            .cloned()
            .collect();

		// handle input
        let mut move_up = false;
        let mut move_down = false;
        let mut enter_pressed = false;

        ctx.input_mut(|i| {
            if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) {
                move_down = true;
            }
            if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp) {
                move_up = true;
            }
            if i.consume_key(egui::Modifiers::NONE, egui::Key::Enter) {
                enter_pressed = true;
            }
            if i.consume_key(egui::Modifiers::NONE, egui::Key::Escape) {
                println!("hide window triggered");
            }
        });

        // keyboard
        if move_down && self.selected_index + 1 < filtered_apps.len() {
            self.selected_index += 1;
        } else if move_up {
            self.selected_index = self.selected_index.saturating_sub(1);
        }

        // clamp index just in case
        if self.selected_index >= filtered_apps.len() && !filtered_apps.is_empty() {
            self.selected_index = filtered_apps.len() - 1;
        }

        if enter_pressed {
            if let Some(app) = filtered_apps.get(self.selected_index) {
                println!("run {app}");
            }
        }

        // ui
        egui::CentralPanel::default().show(ctx, |ui| {
			ui.style_mut().interaction.selectable_labels = false;
            
            // search
            let input_response = ui.add_sized(
                [ui.available_width(), 30.0],
                egui::TextEdit::singleline(&mut self.search_query).hint_text("> search"),
            );
            input_response.request_focus();

            ui.separator();

            // list
            egui::ScrollArea::vertical()
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .drag_to_scroll(false)
                .show(ui, |ui| {
                    for (index, app) in filtered_apps.iter().enumerate() {
                        let is_selected = index == self.selected_index;

                        let bg_color = if is_selected {
                            egui::Color32::from_gray(60)
                        } else {
                            egui::Color32::TRANSPARENT
                        };

                        let response = egui::Frame::default()
                            .fill(bg_color)
                            .inner_margin(8.0)
                            .show(ui, |ui| {
                                ui.add_sized([ui.available_width(), 20.0], egui::Label::new(app));
                            })
                            .response;

                        // auto scroll
                        if is_selected {
                            response.scroll_to_me(Some(egui::Align::Center)); 
                        }
                    }
                });
        });
    }
}