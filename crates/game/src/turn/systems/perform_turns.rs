use crate::prelude::*;

// #[derive(Resource)]
// pub struct CachedTurnSystemState(
//     pub  SystemState<(
//         ResMut<'static, MapManager>,
//         ResMut<'static, TurnManager>,
//         ResMut<'static, ActionQueue>,
//         Query<'static, 'static, &'static Player>,       // player query
//         Query<'static, 'static, &'static Movement>,     // movement query
//         Query<'static, 'static, &'static mut Position>, // position query
//         Query<'static, 'static, (&'static mut AIComponent, &'static Name)>, // ai query
//         Query<'static, 'static, &'static mut Health>,   // health query
//     )>,
// );

// impl FromWorld for CachedTurnSystemState {
//     fn from_world(world: &mut World) -> Self { Self(SystemState::new(world)) }
// }

pub fn perform_turns(world: &mut World) {
    world.resource_scope(|world, mut turn_manager: Mut<TurnManager>| {
        loop {
            if let Some(entity) = turn_manager.start_entity_turn() {
                let is_player = world.get::<Player>(entity).is_some();
                let mut ai_q = world.query::<(&mut AIComponent, &Name)>();
                let mut action_queue = world.resource_mut::<ActionQueue>();

                let mut action = if is_player {
                    if let Some(a) = action_queue.get_action() {
                        a
                    } else {
                        turn_manager.end_entity_turn(entity, 0);
                        return;
                    }
                } else if let Ok((ai_component, name)) = ai_q.get_mut(world, entity) {
                    info!("Starting turn for {}", name);

                    if let Some(a) = ai_component.preferred_action {
                        info!("{} is performing {:?}", name, a);
                        a
                    } else {
                        info!("{} has no preferred action!", name);
                        turn_manager.end_entity_turn(entity, 0);
                        return;
                    }
                } else {
                    info!("AI does not have an AI Component.");
                    // don't add the entity back to the queue...
                    // just go to the next one and try to recover
                    return;
                };

                loop {
                    match perform_action2(entity, action, world) {
                        Ok(time_spent) => {
                            turn_manager.end_entity_turn(entity, time_spent);
                            break;
                        },
                        Err(a) => action = a,
                    }
                }

                if is_player {
                    for (mut ai_component, _name) in ai_q.iter_mut(world) {
                        ai_component.preferred_action = None;
                    }
                    return;
                }
            } else {
                error!("No entities waiting for a turn!");
                return;
            }
        }
    });
}
