use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    let _app = App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.7, 1.0)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "collider-issue".into(),
                        width: 1280.0,
                        height: 720.0,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(8.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_system(add_collider)
        .run();
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle {
        transform: Transform::default().with_scale(Vec3::new(0.5, 0.5, 1.0)),
        ..Default::default()
    });

    // Spawn the entity WITHOUT a Collider
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("example.png"),
            ..Default::default()
        })
        .insert(TransformBundle::from(
            Transform::default().with_translation(Vec3::new(64.0, 0.0, 0.0)),
        ));
}

pub fn add_collider(
    mut commands: Commands,
    mut collider_query: Query<(Entity, &Transform), With<Sprite>>,
    input: Res<Input<KeyCode>>,
) {
    let (entity, transform) = collider_query.single_mut(); 

    // Add a Collider to the entity when Q is pressed
    if input.just_pressed(KeyCode::Q) {
        println!("Input Q detected, adding collider!");
        commands.entity(entity).insert(Collider::cuboid(8.0, 8.0));
    }

    // The current fix is to just re-add the transform
    if input.just_pressed(KeyCode::W) {
        println!("Input W detected, re-adding transform!");
        commands.entity(entity).insert(Transform::from(transform.clone()));
    }
}