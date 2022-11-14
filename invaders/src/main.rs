use bevy::prelude::*;


#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn startup_system(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(
            GreetTimer(
                Timer::from_seconds(2.0, TimerMode::Repeating)
            )
        )
        .add_startup_system(startup_system)
        .add_system(greet_people);
    }
}


#[derive(Component)]
struct AnimateScale;


fn startup_system_scale(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let font = asset_server.load("fonts\\AGENCYB.TTF");
    let text_style = TextStyle {
        font_size: 60.0,
        color: Color::WHITE,
        font,
    };
    let text_alignment = TextAlignment::CENTER;
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Seviyorum", text_style.clone()).with_alignment(text_alignment),
            ..default()
        },
        AnimateScale,
    ));
}

pub struct ScalePlugin;

impl Plugin for ScalePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(animate_scale)
        .add_startup_system(startup_system_scale);
    }
}

fn animate_scale(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateScale>)>,
) {
    // Consider changing font-size instead of scaling the transform. Scaling a Text2D will scale the
    // rendered quad, resulting in a pixellated look.
    for mut transform in &mut query {
        transform.translation = Vec3::new(0.0, 0.0, 0.0);
        transform.scale = Vec3::splat((time.elapsed_seconds().sin() + 1.1) * 2.0);
    }
}


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(ScalePlugin)
    .run();
}
