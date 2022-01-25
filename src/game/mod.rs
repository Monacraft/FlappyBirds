
use bevy::prelude::*;
use bevy::app::AppExit;
use super::client::*;

// Window Dim
pub const SIZE_X: f32 = 800.;
pub const SIZE_Y: f32 = 600.;

// Pipe generation
pub const MIN_PIPE_SIZE: f32 = 50.;
pub const GAP_SIZE: f32 = 200.;

// Pipe texture
pub const PIPE_SCALE: f32 = 5.0;
pub const PIPE_TEX_HEIGHT: f32 = 92.0;
pub const PIPE_TEX_WIDTH: f32 = 14.0;

// Pipe texture
pub const BIRD_SCALE: f32 = 3.0;
pub const BIRD_TEX_HEIGHT: f32 = 12.0;
pub const BIRD_TEX_WIDTH: f32 = 16.0;

// Physics
pub const GRAVITY: f32 = -1600.0;
pub const JUMP_SPEED: f32 = 600.0;
pub const FORWARD_SPEED: f32 = 450.0;


pub fn move_birds (
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Bird)>
) {
    
    for (mut transform, mut bird) in query.iter_mut() {
        transform.translation.x += FORWARD_SPEED * time.delta_seconds();
        bird.vertical_velocity += GRAVITY * time.delta_seconds();
        transform.translation.y += bird.vertical_velocity * time.delta_seconds();

        let angle: f64 = ((bird.vertical_velocity / (FORWARD_SPEED)) as f64).atan();

        transform.rotation = Quat::from_rotation_z(angle as f32)
    }
}

pub fn generate_pipes (
    mut score: ResMut<Score>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(&Transform, &Player)>
) {
    let (transform, _) = query.single();
    spawn_pipes(&mut commands, asset_server, transform.translation.x);

    score.0 += 1;
}

pub fn remove_pipes (
    mut commands: Commands, 
    mut query: Query<(Entity, &Transform, &Pipe)>
) {
    for (entity, transform, _) in query.iter_mut() {
        if transform.translation.x < -SIZE_X {
            commands.entity(entity).despawn();
        }
    }
}

pub fn collision (
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
    mut exit: EventWriter<AppExit>,
    birds: Query<(&Transform, &Player)>,
    pipes: Query<(Entity, &Transform, &Pipe, Without<Player>)>
) {
    let (Transform { translation: bird_translation, .. }, _) =  birds.single();
    let bird_radius = BIRD_TEX_HEIGHT * BIRD_SCALE/2.0;
    let pipe_size = PIPE_TEX_HEIGHT * PIPE_SCALE;

    // PIPE COLLISION
    for (entity, pipe_transform, _, _) in pipes.iter() {
        if (pipe_transform.translation.x - bird_translation.x).abs()
            < PIPE_TEX_WIDTH/2.0 * PIPE_SCALE + bird_radius  {

                //println!("Pipe in range: {} > {}", (pipe_transform.translation.x - bird_translation.x).abs(), PIPE_TEX_WIDTH * PIPE_SCALE + bird_radius);
                
                //println!(" - Bottom of pipe: {} > {}", bird_translation.y + bird_radius, pipe_transform.translation.y - pipe_size/2.0);
                //println!(" -    Top of pipe: {} < {}", bird_translation.y - bird_radius, pipe_transform.translation.y + pipe_size/2.0);
                
                if bird_translation.y + bird_radius > pipe_transform.translation.y - pipe_size/2.0
                    && bird_translation.y - bird_radius < pipe_transform.translation.y + pipe_size/2.0 {

                       app_state.set(AppState::Spectating).unwrap(); 
                       commands.entity(entity).despawn();
                       exit.send(AppExit);
                }
        }
    }

    // BOUNDARY COLLISION
    if bird_translation.y + bird_radius > SIZE_Y / 2. 
        || bird_translation.y - bird_radius < -SIZE_Y / 2. {
            
            app_state.set(AppState::Spectating).unwrap(); 
            exit.send(AppExit);
    } 
}

