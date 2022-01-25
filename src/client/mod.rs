use bevy::prelude::*;
use rand;
use rand::Rng;
use super::game::*;
use bevy::core::FixedTimestep;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Waiting,
    Playing,
    Spectating,
}

#[derive(Component, Default)] 
pub struct Pipe;

#[derive(Component, Default)] 
pub struct Bird {
    animation_state: i32,
    pub vertical_velocity: f32,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MainCamera;


pub struct Score(pub i32);

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_state(AppState::Waiting)
            .insert_resource(Score(-2))
            .add_system_set(SystemSet::on_enter(AppState::Waiting)
                .with_system(init))
            .add_system_set(SystemSet::on_update(AppState::Playing)
                .with_system(move_camera)
                .with_system(move_birds)
                .with_system(animate_bird)
                .with_system(collision)
                .with_system(jump_input)
                .with_system(update_score))
            .add_system_set(
                SystemSet::on_update(AppState::Playing)
                    .with_run_criteria(FixedTimestep::step(0.65))
                    .with_system(generate_pipes)
            )
            .add_system_set(SystemSet::on_update(AppState::Spectating)
                .with_system(move_camera)
                .with_system(move_birds)
                .with_system(animate_bird));
            // .insert_resource(WindowDescriptor::default());
    }
}

fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut app_state: ResMut<State<AppState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>
) {
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(SIZE_X, SIZE_Y);
    window.set_title("Flappy Birds".into());
    window.set_resizable(false);

    // Load textures
    let texture_handle = asset_server.load("fuck-james.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(BIRD_TEX_WIDTH, BIRD_TEX_HEIGHT), 3, 1);
    let handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(MainCamera {});
    commands.spawn_bundle(UiCameraBundle::default());

    let text_style = TextStyle {
        font_size: 60.0,
        color: Color::WHITE,
        font: asset_server.load("Candylicious.ttf")
    };

    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    
    commands.spawn_bundle(TextBundle {
        text: Text::with_section("Score: 0", text_style, text_alignment),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });

    spawn_bird(&mut commands, handle, true);

    
    app_state.set(AppState::Playing).unwrap();
}

pub fn update_score (
    score: Res<Score>,
    mut query: Query<&mut Text>
) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("Score: {}", i32::max(0, score.0));
}

pub fn move_camera(
    mut camera: Query<(&mut Transform, &MainCamera)>,
    player: Query<(&Transform, &Player, Without<MainCamera>)>

) {
    let (mut camera_transform, _) = camera.single_mut();
    let (player_transform, _, _) = player.single();
    camera_transform.translation.x = player_transform.translation.x + 150.0;
}

pub fn animate_bird (
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &mut Bird)>,
) {
    
    for (mut timer, mut sprite, mut bird) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            bird.animation_state += 1;
            match bird.animation_state {
                0 | 4 => {
                    sprite.index = 0;
                }
                1 | 3 => {
                    sprite.index = 1;
                }
                2 => {
                    sprite.index = 2;
                }
                _ => unimplemented!()
            }
            if bird.animation_state >= 4 {
                bird.animation_state = 0;
                timer.reset();
                timer.pause();
            }
        }
    }
}

fn spawn_bird (
    commands: &mut Commands,
    bird_handle: Handle<TextureAtlas>,
    player: bool
) {
    let mut timer = Timer::from_seconds(0.075, true);
    timer.pause();

    let mut bird = commands.spawn_bundle(
        SpriteSheetBundle {
            texture_atlas: bird_handle.clone(),
            transform: Transform {
                scale: Vec3::new(BIRD_SCALE, BIRD_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        });

    bird.insert(timer);
    bird.insert(Bird::default());
    if player {
        bird.insert(Player);
    }
}


pub fn spawn_pipes (
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    player_x: f32,
) {

    let mut rng = rand::thread_rng();

    // Bottom Pipe
    let pipe_one = rng.gen_range((MIN_PIPE_SIZE)..(SIZE_Y - MIN_PIPE_SIZE - GAP_SIZE)) - SIZE_Y/2. + (PIPE_TEX_HEIGHT * PIPE_SCALE)/2.;
    commands.spawn_bundle(
        SpriteBundle {
            texture: asset_server.load("pipe.png"),
            transform: Transform {
                translation: Vec3::new(player_x + SIZE_X, pipe_one - (PIPE_TEX_HEIGHT * PIPE_SCALE), 0.),
                scale: Vec3::new(PIPE_SCALE, PIPE_SCALE, 1.0),
                ..Default::default()
            },
            ..Default::default()
        }
    ).insert(Pipe {});

    // Top Pipe
    let pipe_two = pipe_one + GAP_SIZE;
    commands.spawn_bundle(
        SpriteBundle {
            texture: asset_server.load("pipe.png"),
            transform: Transform {
                translation: Vec3::new(player_x + SIZE_X, pipe_two, 0.),
                scale: Vec3::new(PIPE_SCALE, PIPE_SCALE, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                flip_y: true,
                flip_x: true,
                ..Default::default()
            },
            ..Default::default()
        }
    ).insert(Pipe {});
}

fn jump_input(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut Bird, &mut Timer, &Player)>
) {
    let (mut bird, mut timer, _) = query.single_mut();

    if keyboard.just_pressed(KeyCode::Space) {
        bird.vertical_velocity = JUMP_SPEED;
        timer.unpause();
    }
}

