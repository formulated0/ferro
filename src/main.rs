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
        Box::new(|cc| Ok(Box::new(FerroApp::new(cc)))),
    )
}

pub struct FerroApp {
	search_query: String,
    all_apps: Vec<String>,
    selected_index: usize,
    centered: bool,
}

impl FerroApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self { 
			search_query: String::new(),
			all_apps: Vec::new(),
			selected_index: 0,
			centered: false 
		}
    }

	fn handle_keys(&mut self, ctx: &egui::Context, filtered_apps: &[String]) {
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            if self.selected_index + 1 < filtered_apps.len() {
                self.selected_index += 1;
            }
        }

        if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            self.selected_index = self.selected_index.saturating_sub(1);
        }

        if let Some(app) = filtered_apps.get(self.selected_index) {
				println!("run {app}");
        }
        
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            println!("hide window triggered");
        }
    }
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
            ],
            selected_index: 0,
			centered: false
        }
    }
}

impl eframe::App for FerroApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.centered {
            if let Some(monitor_size) =
                ctx.input(|i| i.viewport().monitor_size)
            {
                if monitor_size.x > 0.0 && monitor_size.y > 0.0 {
                    self.centered = true;

                    let pos = egui::Pos2::new(
                        (monitor_size.x - WINDOW_WIDTH) / 2.0,
                        (monitor_size.y - WINDOW_HEIGHT) / 2.0,
                    );

                    ctx.send_viewport_cmd(
                        egui::ViewportCommand::OuterPosition(pos),
                    );
                }
            }
        }

        let filtered_apps: Vec<String> = self.all_apps
            .iter()
            .filter(|app| app.to_lowercase().contains(&self.search_query.to_lowercase()))
			.cloned()
            .collect();

        if self.selected_index >= filtered_apps.len() && !filtered_apps.is_empty() {
            self.selected_index = filtered_apps.len() - 1;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // add shit here
            // ui.label("hello :3");
            let input_response = ui.add_sized(
                [ui.available_width(), 30.0],
                egui::TextEdit::singleline(&mut self.search_query).hint_text("> search")
            );
            input_response.request_focus();

            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
				for (index, app) in filtered_apps.iter().enumerate() {
					let is_selected = index == self.selected_index;
					
					let bg_color = if is_selected {
						egui::Color32::from_gray(60)
					} else {
						egui::Color32::TRANSPARENT
					};
					
					egui::Frame::default().fill(bg_color).inner_margin(8.0).show(ui, |ui| {
						ui.add_sized([ui.available_width(), 20.0], egui::Label::new(app));
					});
				}
			});
        });

		self.handle_keys(ctx, &filtered_apps);
    }
}