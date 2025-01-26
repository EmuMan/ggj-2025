use bevy::prelude::*;

use crate::resources::bubbles::*;
use crate::systems::bubbles::spawning::*;
use crate::systems::bubbles::combat::*;
use crate::systems::bubbles::shockwave::*;
use crate::systems::bubbles::movement::*;

pub struct BubblesPlugin;

impl Plugin for BubblesPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<BubbleSpawnTimer>()
            .init_resource::<BubbleChances>()
            .add_event::<BubbleDestroyedEvent>()
            .add_systems(Startup, init_bubble_spawner)
            .add_systems(Update, spawn_bubbles)
            .add_systems(Update, update_bubble_velocity)
            .add_systems(Update, bubble_clicked)
            .add_systems(Update, spawn_shockwaves)
            .add_systems(Update, expand_shockwaves)
            .add_systems(Update, wobble_black_holes)
            .add_systems(Update, bubble_hit_by_shockwave)
            .add_systems(Update, bubble_in_black_hole)
            .add_systems(Update, spawn_scatter_shot_shockwaves)
            .add_systems(Update, advance_bubble_collapse
                .after(bubble_clicked)
                .after(bubble_hit_by_shockwave)
                .after(wobble_black_holes));
    }

}
