use bevy::prelude::*;

use crate::GameState;

use super::{enemy::Enemy, Health};

pub struct ScorePlugin;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct Score(pub usize);

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(Update, update_score.run_if(in_state(GameState::Playing)));
    }
}

fn update_score(
    mut score: ResMut<Score>,
    enemy_query: Query<Ref<Health>, (Changed<Health>, With<Enemy>)>,
) {
    **score += enemy_query.iter().filter(|x| !x.is_added()).count();
}
