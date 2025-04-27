use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    components::{
        Finger,
        Winner
    }, 
    resources::WinnerDisplayTimer,
    states::GameState
};

pub fn select_winner(
    mut commands: Commands,
    finger_query: Query<(Entity, &Finger)>,
) {
    let finger_count = finger_query.iter().count();

    if finger_count >= 2 {
        let fingers: Vec<(Entity, &Finger)> = finger_query.iter().collect();

        let mut rng = rand::rng();
        let winner_index = rng.random_range(0..finger_count);
        let (winner_entity, winner_finger) = fingers[winner_index];

        commands.entity(winner_entity).insert(Winner);

        println!("Winner selected: {:?}", winner_finger.key);
    } else {
        println!("Not enough fingers to select a winner!");
    }
}

pub fn setup_winner_timer(mut commands: Commands) {
    commands.insert_resource(
        WinnerDisplayTimer(
            Timer::from_seconds(5.0, TimerMode::Once)
        )
    );
}

pub fn check_winner_timer(
    time: Res<Time>,
    mut timer: ResMut<WinnerDisplayTimer>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        next_game_state.set(GameState::Waiting);
    }
}


pub fn highlight_winner(
    mut query: Query<(&Finger, &mut Transform), With<Winner>>,
    time: Res<Time>,

) {
    for (finger, mut transform) in query.iter_mut() {
        let scale = 1.0 + (time.elapsed_secs().sin() * 0.2 + 0.2);
        transform.scale = Vec3::new(scale, scale, 1.0);

        finger.color.with_luminance(1.0);
    }
}

pub fn cleanup_non_winners(
    mut commands: Commands,
    mut query: Query<(Entity, &Finger), Without<Winner>>
) {
    for (entity, _) in query.iter_mut() {
        commands.entity(entity).despawn();
    }
}


pub fn cleanup_winner(
    mut commands: Commands,
    winner_query: Query<Entity, With<Winner>>,
) {
    for entity in winner_query.iter() {
        commands.entity(entity).remove::<Winner>();
    }

    commands.remove_resource::<WinnerDisplayTimer>();
}
