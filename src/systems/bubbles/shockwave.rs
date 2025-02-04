use std::f32::consts::PI;
use std::time::Duration;

use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand_core::RngCore;

use crate::components::bubbles::*;
use crate::components::physics::Collider;
use crate::resources::audio::AudioLimiter;
use crate::resources::bubbles::*;
use crate::resources::cache::{MeshCache, AudioCache};
use crate::util::{self, ActionTimer};

pub fn spawn_shockwaves(
    mut commands: Commands,
    mesh_cache: Res<MeshCache>,
    audio_cache: Res<AudioCache>,
    mut audio_limiter: ResMut<AudioLimiter>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut random: ResMut<GlobalEntropy<WyRand>>,
    mut bubble_destroyed_event: EventReader<BubbleDestroyedEvent>,
) {
    for event in bubble_destroyed_event.read() {
        match event.bubble_type {
            BubbleType::Normal => {
                spawn_normal_shockwave(
                    &mut commands,
                    &mesh_cache,
                    &audio_cache,
                    &mut audio_limiter,
                    &mut materials,
                    &mut random,
                    event.position,
                    event.radius,
                    event.color,
                );
            }
            BubbleType::Mega => {
                spawn_mega_shockwave(
                    &mut commands,
                    &mesh_cache,
                    &audio_cache,
                    &mut audio_limiter,
                    &mut materials,
                    &mut random,
                    event.position,
                    event.radius,
                    event.color,
                );
            }
            BubbleType::ScatterShot => {
                spawn_scatter_shot(
                    &mut commands,
                    event.position,
                    200.0,
                    50.0,
                    event.color,
                );
            }
            BubbleType::Beam => {
                spawn_beam(
                    &mut commands,
                    &mesh_cache,
                    &audio_cache,
                    &mut audio_limiter,
                    &mut materials,
                    &mut random,
                    event.position,
                    50.0,
                    event.color,
                )
            }
            BubbleType::BlackHole => {
                spawn_black_hole(
                    &mut commands,
                    &mesh_cache,
                    &audio_cache,
                    &mut audio_limiter,
                    &mut materials,
                    &mut random,
                    event.position,
                    300.0,
                    event.color,
                );
            }
        }
    }
}

fn spawn_normal_shockwave(
    mut commands: &mut Commands,
    mesh_cache: &MeshCache,
    audio_cache: &AudioCache,
    audio_limiter: &mut AudioLimiter,
    materials: &mut Assets<ColorMaterial>,
    random: &mut GlobalEntropy<WyRand>,
    position: Vec2,
    radius: f32,
    color: Color,
) {
    let mut shockwave_color = color.clone();
    shockwave_color.set_alpha(0.3);
    commands.spawn(BubbleShockwaveBundle {
        mesh: Mesh2d(mesh_cache.circle_mesh.clone()),
        mesh_material: MeshMaterial2d(materials.add(shockwave_color)),
        transform: Transform::from_translation(position.extend(-(position.x / 1000.0 + position.y)))
            .with_scale(Vec3::splat(radius)),
        timed_effect: TimedEffect::new(Duration::from_secs_f32(0.2)),
        bubble_shockwave: BubbleShockwave::new(radius, 50.0),
        collider: Collider {
            radius,
            ..Default::default()
        },
    });

    audio_limiter.play_if_allowed(
        &mut commands,
        audio_cache.bubble_pop.clone(),
        PlaybackSettings {
            speed: util::random_f32(random.next_u64(), 0.7, 1.3),
            mode: PlaybackMode::Despawn,
            ..default()
        },
    );
}

fn spawn_mega_shockwave(
    mut commands: &mut Commands,
    mesh_cache: &MeshCache,
    audio_cache: &AudioCache,
    audio_limiter: &mut AudioLimiter,
    materials: &mut Assets<ColorMaterial>,
    random: &mut GlobalEntropy<WyRand>,
    position: Vec2,
    radius: f32,
    color: Color,
) {
    let mut shockwave_color = color.clone();
    shockwave_color.set_alpha(0.5);
    commands.spawn(BubbleShockwaveBundle {
        mesh: Mesh2d(mesh_cache.circle_mesh.clone()),
        mesh_material: MeshMaterial2d(materials.add(shockwave_color)),
        transform: Transform::from_translation(position.extend(-(position.x / 1000.0 + position.y)))
            .with_scale(Vec3::splat(radius)),
        timed_effect: TimedEffect::new(Duration::from_secs_f32(1.0)),
        bubble_shockwave: BubbleShockwave::new(radius, 500.0),
        collider: Collider {
            radius,
            ..Default::default()
        },
    });

    audio_limiter.play_if_allowed(
        &mut commands,
        audio_cache.bubble_explosion.clone(),
        PlaybackSettings {
            speed: util::random_f32(random.next_u64(), 0.9, 1.1),
            mode: PlaybackMode::Despawn,
            volume: Volume::new(0.5),
            ..default()
        },
    );
}

fn spawn_black_hole(
    mut commands: &mut Commands,
    mesh_cache: &MeshCache,
    audio_cache: &AudioCache,
    audio_limiter: &mut AudioLimiter,
    materials: &mut Assets<ColorMaterial>,
    random: &mut GlobalEntropy<WyRand>,
    position: Vec2,
    max_radius: f32,
    color: Color,
) {
    let mut black_hole_color = color.clone();
    black_hole_color.set_alpha(0.5);
    commands.spawn(BubbleBlackHoleBundle {
        mesh: Mesh2d(mesh_cache.circle_mesh.clone()),
        mesh_material: MeshMaterial2d(materials.add(black_hole_color)),
        transform: Transform::from_translation(position.extend(-(position.x / 1000.0 + position.y)))
            .with_scale(Vec3::ZERO),
        timed_effect: TimedEffect::new(Duration::from_secs_f32(3.0)),
        bubble_black_hole: BubbleBlackHole::new(max_radius, 1000.0, 100.0),
        collider: Collider {
            radius: 0.0,
            ..Default::default()
        },
    });

    audio_limiter.play_if_allowed(
        &mut commands,
        audio_cache.bubble_black_hole.clone(),
        PlaybackSettings {
            speed: util::random_f32(random.next_u64(), 0.9, 1.1),
            mode: PlaybackMode::Despawn,
            ..default()
        },
    );
}

fn spawn_scatter_shot(
    commands: &mut Commands,
    position: Vec2,
    radius: f32,
    variation: f32,
    color: Color,
) {
    let mut scatter_shot_color = color.clone();
    scatter_shot_color.set_alpha(0.5);

    commands.spawn(BubbleScatterShotSpawnerBundle {
        action_timer: ActionTimer::new(Duration::from_secs_f32(0.5), 15, TimerMode::Once),
        spawner: BubbleScatterShotSpawner::new(
            radius,
            variation,
            Duration::from_secs_f32(0.2),
            BubbleShockwave::new(0.0, 50.0),
            color,
        ),
        transform: Transform::from_translation(position.extend(0.0)),
    });
}

fn spawn_beam(
    mut commands: &mut Commands,
    mesh_cache: &MeshCache,
    audio_cache: &AudioCache,
    audio_limiter: &mut AudioLimiter,
    materials: &mut Assets<ColorMaterial>,
    random: &mut GlobalEntropy<WyRand>,
    position: Vec2,
    width: f32,
    color: Color,
) {
    let mut beam_color = color.clone();
    beam_color.set_alpha(0.5);

    commands.spawn(BubbleBeamBundle {
        mesh: Mesh2d(mesh_cache.long_rectangle_mesh.clone()),
        mesh_material: MeshMaterial2d(materials.add(beam_color)),
        timed_effect: TimedEffect::new(Duration::from_secs_f32(1.0)),
        beam: BubbleBeam::new(width),
        transform: Transform::from_translation(position.extend(-(position.x / 1000.0 + position.y)))
            .with_scale(Vec3::new(0.0, 1.0, 1.0)),
    });

    audio_limiter.play_if_allowed(
        &mut commands,
        audio_cache.bubble_beam.clone(),
        PlaybackSettings {
            speed: util::random_f32(random.next_u64(), 0.9, 1.1),
            mode: PlaybackMode::Despawn,
            volume: Volume::new(0.8),
            ..default()
        },
    );
}

pub fn expand_shockwaves(
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut shockwave_query: Query<(
        Entity,
        &mut TimedEffect,
        &mut BubbleShockwave,
        &mut Collider,
        &mut Transform,
        &MeshMaterial2d<ColorMaterial>,
    )>,
) {
    for (
        entity,
        mut timed_effect,
        mut shockwave,
        mut collider,
        mut transform,
        material
    ) in &mut shockwave_query {
        if timed_effect.tick(time.delta()) {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        shockwave.set_radius_from_time(timed_effect.progress());
        collider.radius = shockwave.radius;
        transform.scale = Vec3::splat(shockwave.radius);
        materials.get_mut(material).map(|mat| {
            mat.color.set_alpha((1.0 - timed_effect.progress()).powf(0.5) * 0.3);
        });
    }
}

pub fn wobble_black_holes(
    mut commands: Commands,
    time: Res<Time>,
    mut black_hole_query: Query<(
        Entity,
        &mut TimedEffect,
        &mut BubbleBlackHole,
        &mut Transform,
        &mut Collider,
    )>,
) {
    for (
        entity,
        mut timed_effect,
        mut black_hole,
        mut transform,
        mut collider,
    ) in &mut black_hole_query {
        if timed_effect.tick(time.delta()) {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        black_hole.set_radius_from_time(timed_effect.progress());
        collider.radius = black_hole.radius;
        transform.scale = Vec3::splat(black_hole.radius);
    }
}

pub fn spawn_scatter_shot_shockwaves(
    mut commands: Commands,
    time: Res<Time>,
    mesh_cache: Res<MeshCache>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut random: ResMut<GlobalEntropy<WyRand>>,
    mut scatter_shot_query: Query<(
        Entity,
        &mut ActionTimer,
        &mut BubbleScatterShotSpawner,
        &Transform,
    )>,
) {
    for (
        entity,
        mut action_timer,
        spawner,
        spawner_transform,
    ) in &mut scatter_shot_query {
        let Some(to_spawn) = action_timer.tick(time.delta()) else {
            commands.entity(entity).despawn_recursive();
            continue;
        };

        for _ in 0..to_spawn {
            let angle = util::random_f32(random.next_u64(), 0.0, PI * 2.0);
            let direction = Vec2::new(angle.cos(), angle.sin());
            let (min, max) = (spawner.radius - spawner.variation, spawner.radius + spawner.variation);
            let radius = util::random_f32(random.next_u64(), min, max);
            let position = spawner_transform.translation + (direction * radius).extend(0.0);
            let mut color = spawner.shockwave_color.clone();
            color.set_alpha(0.99);
            
            commands.spawn(BubbleShockwaveBundle {
                mesh: Mesh2d(mesh_cache.circle_mesh.clone()),
                mesh_material: MeshMaterial2d(materials.add(color)),
                transform: Transform::from_translation(position),
                timed_effect: spawner.instance_timer.clone(),
                bubble_shockwave: spawner.instance.clone(),
                collider: Collider {
                    radius: 0.0,
                    ..Default::default()
                },
            });
        }
    }
}

pub fn expand_beam(
    mut commands: Commands,
    time: Res<Time>,
    mut beam_query: Query<(
        Entity,
        &mut TimedEffect,
        &mut BubbleBeam,
        &mut Transform,
    )>,
) {
    for (
        entity,
        mut timed_effect,
        mut beam,
        mut transform,
    ) in &mut beam_query {
        if timed_effect.tick(time.delta()) {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        transform.scale = Vec3::new(beam.width, 1.0, 1.0);
        beam.set_width_from_time(timed_effect.progress());
    }
}
