use bevy::{ecs::schedule::Condition, prelude::*};
use bevy_egui::{EguiInput, EguiRenderToTexture};
use bevy_mod_picking::{
    backend::PointerHits,
    events::{Click, Down, Move, Pointer, Up},
    focus::PickingInteraction,
    picking_core::Pickable,
    prelude::{ListenerInput, On},
    PickableBundle,
};

#[derive(Clone, Copy, Component, Debug)]
pub struct WorldUI {
    size_x: f32,
    size_y: f32,
}

#[derive(Event, Clone, Copy, Debug)]
pub struct UIPointerClick {
    target: Entity,
    position: Option<Vec3>,
    normal: Option<Vec3>,
}

#[derive(Event, Clone, Copy, Debug)]
pub struct UIPointerMove {
    target: Entity,
    position: Option<Vec3>,
    normal: Option<Vec3>,
}

impl From<ListenerInput<Pointer<Click>>> for UIPointerClick {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        Self {
            target: event.target,
            position: event.hit.position,
            normal: event.hit.normal,
        }
    }
}

impl From<ListenerInput<Pointer<Move>>> for UIPointerMove {
    fn from(event: ListenerInput<Pointer<Move>>) -> Self {
        Self {
            target: event.target,
            position: event.hit.position,
            normal: event.hit.normal,
        }
    }
}

#[derive(Bundle)]
pub struct WorldSpaceUI {
    on_move: On<Pointer<Move>>,
    on_click: On<Pointer<Click>>,
    render_texture: EguiRenderToTexture,
    pub pickable: Pickable,
    pub interaction: PickingInteraction,
    world_ui: WorldUI,
}
impl WorldSpaceUI {
    pub fn new(texture: Handle<Image>, size_x: f32, size_y: f32) -> Self {
        WorldSpaceUI {
            on_move: On::<Pointer<Move>>::send_event::<UIPointerMove>(),
            on_click: On::<Pointer<Click>>::send_event::<UIPointerClick>(),
            render_texture: EguiRenderToTexture(texture),
            pickable: Pickable::default(),
            interaction: PickingInteraction::default(),
            world_ui: WorldUI { size_x, size_y },
        }
    }
}

pub struct PickabelEguiPlugin;
impl Plugin for PickabelEguiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UIPointerMove>();
        app.add_event::<UIPointerClick>();
        app.add_systems(
            Update,
            ui_interactions
                .run_if(on_event::<UIPointerMove>().or_else(on_event::<UIPointerClick>())),
        );
    }
}

pub fn ui_interactions(
    mut inputs: Query<(
        &mut EguiInput,
        &WorldUI,
        &GlobalTransform,
        &EguiRenderToTexture,
    )>,
    mut move_events: EventReader<UIPointerMove>,
    mut click_events: EventReader<UIPointerClick>,
    textures: Res<Assets<Image>>,
) {
    for UIPointerMove {
        target,
        position,
        normal,
    } in move_events.read()
    {
        if let (Ok((mut input, ui, transform, texture)), Some(position), Some(normal)) =
            (inputs.get_mut(*target), position, normal)
        {
            let rotated_point = transform
                .to_scale_rotation_translation()
                .1
                .inverse()
                .mul_vec3(*position);
            let local_pos = rotated_point - transform.translation();
            let mut uv = local_pos.xz() + Vec2::splat(0.5);
            uv.x /= ui.size_x;
            uv.y /= ui.size_y;
            // Whem the texture exists then it must be in the assets i think
            let image = textures.get(texture.0.clone()).unwrap();
            input.events.push(bevy_egui::egui::Event::PointerMoved(
                bevy_egui::egui::Pos2 {
                    x: uv.x * image.width() as f32,
                    y: uv.y * image.height() as f32,
                },
            ));
            info!("uv: {}", uv);
        }
    }
    for UIPointerClick {
        target,
        position,
        normal,
    } in click_events.read()
    {
        if let (Ok((mut input, ui, transform, texture)), Some(position), Some(normal)) =
            (inputs.get_mut(*target), position, normal)
        {
            let rotated_point = transform
                .to_scale_rotation_translation()
                .1
                .inverse()
                .mul_vec3(*position);
            let local_pos = rotated_point - transform.translation();
            let mut uv = local_pos.xz() + Vec2::splat(0.5);
            uv.x /= ui.size_x;
            uv.y /= ui.size_y;
            // Whem the texture exists then it must be in the assets i think
            let image = textures.get(texture.0.clone()).unwrap();
            input.events.push(bevy_egui::egui::Event::PointerButton {
                pos: bevy_egui::egui::Pos2 {
                    x: uv.x * image.width() as f32,
                    y: uv.y * image.height() as f32,
                },
                button: bevy_egui::egui::PointerButton::Primary,
                pressed: true,
                modifiers: bevy_egui::egui::Modifiers::NONE,
            });
            input.events.push(bevy_egui::egui::Event::PointerButton {
                pos: bevy_egui::egui::Pos2 {
                    x: uv.x * image.width() as f32,
                    y: uv.y * image.height() as f32,
                },
                button: bevy_egui::egui::PointerButton::Primary,
                pressed: false,
                modifiers: bevy_egui::egui::Modifiers::NONE,
            });
            info!("uv: {}", uv);
        }
    }
}
