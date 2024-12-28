use serde::{Deserialize, Serialize};

use super::MainState;

use crate::game_object::Update;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GUIState
{
    pub shown: bool,
}

impl Update<ggegui::Gui> for MainState
{
    fn update(&mut self, context: &mut ggez::Context) -> ggez::GameResult 
    {
        if !self.gui_state.shown
        {
            return Ok(())
        }

        let gui_context = self.gui.ctx();

        use ggegui::egui;
        egui::Window::new("Dragging Window")
        .show(&gui_context, 
        |ui| {
            ui.label("label");
            if ui.button("button").clicked()
            {
                println!("button clicked");
            }

            ui.add_enabled(false, egui::Button::new("test"));
        });
        self.gui.update(context);

        // egui::SidePanel::new(egui::panel::Side::Left, "left_panel")
        // .show(&gui_context,
        // |ui| {
        //     ui.label("left panel");
        // });

    

    

        Ok(())    
    }
}
