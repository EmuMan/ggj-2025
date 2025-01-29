use bevy::prelude::*;

#[derive(Resource, Debug, Default, Clone)]
pub struct MeshCache {
    pub circle_mesh: Handle<Mesh>,
    pub long_rectangle_mesh: Handle<Mesh>,
}

#[derive(Resource, Debug, Default, Clone)]
pub struct FontCache {
    pub coolvetica_rg: Handle<Font>,
}
