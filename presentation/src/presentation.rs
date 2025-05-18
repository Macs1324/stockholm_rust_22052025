use bevy::{ecs::hierarchy, prelude::*};

#[derive(Resource)]
pub struct CurrentSlideNumber(usize);

#[derive(Event)]
pub struct TransitionEvent {
    pub slide_number: usize,
}

#[derive(Event)]
pub struct SlideSwitchEvent {
    pub slide_number: usize,
}

pub struct PresentationPlugin {}

impl Plugin for PresentationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentSlideNumber(0))
            .add_event::<SlideSwitchEvent>()
            .add_event::<TransitionEvent>();
    }
}
