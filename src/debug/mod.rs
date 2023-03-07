use bevy::prelude::*;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, _app: &mut App) {
        info!("Loading Debug Plugin");
        trace!("Enabling Tracing");
    }
}
