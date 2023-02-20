//! Plays animations from a skinned glTF.

use std::f32::consts::PI;
use std::time::Duration;

use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(diagnostic::FrameTimePlugin::default())
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_startup_system(setup)
        .add_system(setup_scene_once_loaded)
        .add_system(keyboard_animation_control)
        .run();
}

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Insert a resource with the current scene information
    commands.insert_resource(Animations(vec![
        asset_server.load("models/animated/Fox.glb#Animation2"),
        asset_server.load("models/animated/Fox.glb#Animation1"),
        asset_server.load("models/animated/Fox.glb#Animation0"),
    ]));

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(100.0, 100.0, 150.0)
            .looking_at(Vec3::new(0.0, 20.0, 0.0), Vec3::Y),
        ..default()
    });

    // Plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(500000.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 200.0,
            maximum_distance: 400.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // Fox
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/animated/Fox.glb#Scene0"),
        ..default()
    });

    println!("Animation controls:");
    println!("  - spacebar: play / pause");
    println!("  - arrow up / down: speed up / slow down animation playback");
    println!("  - arrow left / right: seek backward / forward");
    println!("  - return: change animation");
}

// Once the scene is loaded, start the animation
fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut player: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
) {
    if !*done {
        if let Ok(mut player) = player.get_single_mut() {
            player.play(animations.0[0].clone_weak()).repeat();
            *done = true;
        }
    }
}

fn keyboard_animation_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut animation_player: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    mut current_animation: Local<usize>,
) {
    if let Ok(mut player) = animation_player.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            if player.is_paused() {
                player.resume();
            } else {
                player.pause();
            }
        }

        if keyboard_input.just_pressed(KeyCode::Up) {
            let speed = player.speed();
            player.set_speed(speed * 1.2);
        }

        if keyboard_input.just_pressed(KeyCode::Down) {
            let speed = player.speed();
            player.set_speed(speed * 0.8);
        }

        if keyboard_input.just_pressed(KeyCode::Left) {
            let elapsed = player.elapsed();
            player.set_elapsed(elapsed - 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::Right) {
            let elapsed = player.elapsed();
            player.set_elapsed(elapsed + 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::Return) {
            *current_animation = (*current_animation + 1) % animations.0.len();
            player
                .play_with_transition(
                    animations.0[*current_animation].clone_weak(),
                    Duration::from_millis(250),
                )
                .repeat();
        }
    }
}

mod diagnostic {
    use bevy::prelude::*;
    use std::time::{Duration, Instant};

    #[derive(Default)]
    pub struct FrameTimePlugin {}

    impl Plugin for FrameTimePlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(FrameTimeState::default())
                .add_system(frame_start.in_base_set(CoreSet::First))
                .add_system(frame_end.in_base_set(CoreSet::LastFlush));
        }
    }

    #[derive(Default, Resource)]
    pub struct FrameTimeState {
        frame_start: Option<Instant>,
        last_logged_time: Option<Instant>,
        history: Vec<Duration>,
    }

    /// record current system time for frame start
    fn frame_start(mut state: ResMut<FrameTimeState>) {
        let now = std::time::Instant::now();
        state.frame_start = Some(now);
        // set default value for 1st frame
        if state.last_logged_time.is_none() {
            state.last_logged_time = Some(now);
        }
    }

    /// record frame's end time
    fn frame_end(mut state: ResMut<FrameTimeState>) {
        let now = Instant::now();

        if let Some(frame_start) = state.frame_start.take() {
            state
                .history
                .push((std::time::Instant::now().duration_since(frame_start)).into());
        }

        if let Some(last_time) = state.last_logged_time {
            if now.duration_since(last_time) > Duration::from_secs(5) {
                // do print logic
                state.history.sort_unstable();

                let len = state.history.len();
                if len > 0 {
                    let max = state.history[len - 1];
                    let min = state.history[0];
                    let pct50 = state.history[len / 2];
                    let pct90 = state.history[len * 9 / 10];

                    info!(
                        "min:{min}us p50:{pct50}us p90:{pct90}us max:{max}us",
                        min = min.as_nanos() / 1000,
                        pct50 = pct50.as_nanos() / 1000,
                        pct90 = pct90.as_nanos() / 1000,
                        max = max.as_nanos() / 1000,
                    );
                }

                state.history.clear();
                state.frame_start = None;
                state.last_logged_time = Some(now);
            }
        }
    }
}
