use std::default;

use bevy::prelude::*;
use rand::prelude::*;
use bevy::sprite::collide_aabb::collide;

struct Player;
struct Name(String);
struct Gold;
struct Dynamite;
struct Money{
    money: u32,
}

fn main() {
    App::build()
    .insert_resource(ClearColor(Color::ALICE_BLUE))
    .insert_resource(WindowDescriptor{
        title: "Gold Collector".to_string(),
        width: 800.,
        height: 600.,
        resizable: false,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup.system())
    .insert_resource(Money{money: 0})
    .add_system(move_gold.system())
    .add_system(move_dynamite.system())
    .add_system(move_player.system())
    .add_system(player_gold_collision.system())
    .add_system(player_dynamite_collision.system())
    .add_system(money_system.system())
    .run();
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>){
    let player_texture: Handle<Texture> = asset_server.load("img/player.png");
    let gold_texture: Handle<Texture> = asset_server.load("img/gold.png");
    let dynamite_texture: Handle<Texture> = asset_server.load("img/dynamite.png");

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle(SpriteBundle{
        material: materials.add(player_texture.into()),

        transform: Transform{
            translation: Vec3::new(0., -280., 0.),
            scale: Vec3::new(0.2f32, 0.2f32, 0.),
            ..Default::default()
        },

        ..Default::default()
    }).insert(Player).insert(Name("player".to_string()));


    for x in (0..10){
    commands.spawn_bundle(SpriteBundle{
        material: materials.add(gold_texture.clone_weak().into()),

        transform: Transform{
            translation: Vec3::new(random_number(-400.,400.), random_number(310.,1000.), 0.),
            scale: Vec3::new(0.2, 0.2, 0.),
            ..Default::default()
        },

        ..Default::default()
    }).insert(Gold).insert(Name("gold".to_string()));

    commands.spawn_bundle(SpriteBundle{
        material: materials.add(dynamite_texture.clone_weak().into()),

        transform: Transform{
            translation: Vec3::new(random_number(-400.,400.), random_number(310.,1000.), 0.),
            scale: Vec3::new(0.3, 0.3, 0.),
            ..Default::default()
        },

        ..Default::default()
    }).insert(Dynamite).insert(Name("dynamite".to_string()));

    }


    commands.spawn_bundle(TextBundle{
        text: Text{
            sections: vec![
                TextSection{
                    value: "Money: ".to_string(),
                    style: TextStyle{
                        font: asset_server.load("font/Roboto-Regular.ttf"),
                        font_size: 30.,
                        color: Color::ORANGE,
                    },
                },

                TextSection{
                    value: "".to_string(),
                    style: TextStyle{
                        font: asset_server.load("font/Roboto-Regular.ttf"),
                        font_size: 30.,
                        color: Color::ORANGE,
                    },
                },
            ],
            ..Default::default()
        },

        style: Style{
            position_type: PositionType::Absolute,
            position: Rect{
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()

    });

}

fn move_gold(time: Res<Time>, mut query: Query<(&Gold, &mut Transform)>){
    for(gold, mut transform) in &mut query.iter_mut(){
        transform.translation.y -= 200. * time.delta_seconds();

        if transform.translation.y < -300.{
            transform.translation.y = random_number(310.,1000.);
            transform.translation.x = random_number(-400.,400.);
        }
    }

}

fn move_dynamite(time: Res<Time>, mut query: Query<(&Dynamite, &mut Transform)>){
    for(dynamite, mut transform) in &mut query.iter_mut(){
        transform.translation.y -= 200. * time.delta_seconds();

        if transform.translation.y < -300.{
            transform.translation.y = random_number(310.,1000.);
            transform.translation.x = random_number(-400.,400.);
        }
    }

}

fn move_player(time: Res<Time>, keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&Player, &mut Transform)>){
    if let Ok((player, mut transform)) = query.single_mut(){
        if keyboard_input.pressed(KeyCode::Left){
            transform.translation.x -= 590. * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::Right){
            transform.translation.x += 590. * time.delta_seconds();

        }

        transform.translation.x = transform.translation.x.min(370.).max(-370.);
    }
}

fn player_gold_collision(mut money: ResMut<Money>, mut set: QuerySet<( Query<&mut Transform, With<Gold>>, Query<&mut Transform, With<Player>>)>){
   let player_size = Vec2::new(50.,50.);
   let gold_size = Vec2::new(20.,20.0);
   let mut player_position = Vec3::new(0.0,0.0,0.0);
   
    for player_data in set.q1_mut().iter_mut(){
        player_position = player_data.translation;
    }

    for mut gold_data in set.q0_mut().iter_mut(){

        if collide(player_position, player_size, gold_data.translation, gold_size).is_some(){
            gold_data.translation.y = random_number(310.,1000.);
            gold_data.translation.x = random_number(-400.,400.);
            money.money += 1;
            println!("{}",money.money);
        }
    }
}

fn player_dynamite_collision(mut money: ResMut<Money>, mut set: QuerySet<( Query<&mut Transform, With<Dynamite>>, Query<&mut Transform, With<Player>>)>){
    let player_size = Vec2::new(50.,50.);
    let dynamite_size = Vec2::new(10.,10.0);
    let mut player_position = Vec3::new(0.0,0.0,0.0);
    
     for player_data in set.q1_mut().iter_mut(){
         player_position = player_data.translation;
     }
 
     for mut dynamite_data in set.q0_mut().iter_mut(){
 
         if collide(player_position, player_size, dynamite_data.translation, dynamite_size).is_some(){
            dynamite_data.translation.y = random_number(310.,1000.);
            dynamite_data.translation.x = random_number(-400.,400.);
             if(money.money > 0){
                 money.money -= 1;
             }
             println!("{}",money.money);
         }
     }
 }

 fn money_system(money: Res<Money>, mut query: Query<&mut Text>){
     let mut text = query.single_mut().unwrap();
     text.sections[1].value = money.money.to_string();
 }
 



fn random_number(min: f32, max: f32)-> f32{
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

