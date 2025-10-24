use crate::constants;

#[derive(Debug)]
pub struct FixedLengthDialog {
    id: egui::Id,
    pub value: f32,
    pub applied: bool,
}

impl Default for FixedLengthDialog {
    fn default() -> Self {
        Self {
            id: constants::ID_FIXED_LEN_DIALOG.into(),
            value: 0.0,
            applied: false,
        }
    }
}

impl FixedLengthDialog {
    pub fn open(&mut self, ui: &mut egui::Ui, init_value: f32) {
        ui.memory_mut(|m| m.toggle_popup(self.id));
        self.value = init_value;
    }

    pub fn render(&mut self, ui: &mut egui::Ui, response: &egui::Response) {
        egui::popup_below_widget(
            ui,
            self.id,
            response,
            egui::PopupCloseBehavior::CloseOnClickOutside,
            |ui| {
                ui.horizontal(|ui| {
                    ui.label("Length:");
                    ui.add(
                        egui::DragValue::new(&mut self.value).range(
                            constants::SIZE_MIN_EDGE_LENGTH..=constants::SIZE_MAX_EDGE_LENGTH,
                        ),
                    );
                });
                if ui.button("Apply").clicked() {
                    ui.memory_mut(|m| m.toggle_popup(self.id));
                    self.applied = true;
                }
            },
        );
    }
}
