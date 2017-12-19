use kay::{ActorSystem, External, World};
use stagemaster::{Interactable2d, Interactable2dID, UserInterfaceID, UserInterfaceLayer};
use imgui::ImGuiSetCond_FirstUseEver;

pub const BASE_LAYER: UserInterfaceLayer = UserInterfaceLayer(0);
pub const ROAD_LAYER: UserInterfaceLayer = UserInterfaceLayer(1);
pub const INFO_LAYER: UserInterfaceLayer = UserInterfaceLayer(2);

#[derive(Compact, Clone)]
pub struct LayerSelection {
    id: LayerSelectionID,
}

impl LayerSelection {
    pub fn spawn(
        id: LayerSelectionID,
        user_interface: UserInterfaceID,
        world: &mut World,
    ) -> LayerSelection {
        user_interface.add_2d(id.into(), world);
        LayerSelection { id }
    }
}

impl Interactable2d for LayerSelection {
    fn draw_ui_2d(
        &mut self,
        imgui_ui: &External<::imgui::Ui<'static>>,
        return_to: UserInterfaceID,
        world: &mut World,
    ) {
        let ui = imgui_ui.steal();

        ui.window(im_str!("UI Mode"))
            .size((200.0, 50.0), ImGuiSetCond_FirstUseEver)
            .collapsible(false)
            .build(|| {
                if ui.small_button(im_str!("Roads")) {
                    return_to.set_current_layer(Some(ROAD_LAYER), world);
                }
                if ui.small_button(im_str!("Info")) {
                    return_to.set_current_layer(Some(INFO_LAYER), world);
                }
            });

        return_to.ui_drawn(ui, world);
    }
}

pub fn setup(system: &mut ActorSystem, user_interface: UserInterfaceID) {
    system.register::<LayerSelection>();
    self::kay_auto::auto_setup(system);
    LayerSelectionID::spawn(user_interface, &mut system.world());
}

mod kay_auto;
pub use self::kay_auto::*;