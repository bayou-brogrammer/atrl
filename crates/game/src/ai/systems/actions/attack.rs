use big_brain::actions::ActionState;

use crate::prelude::*;

#[derive(Debug, Default, Component, Clone)]
// could be used for temporary storage for multi turn actions
pub struct AttackActor;

pub fn attack_action(
    mut commands: Commands,
    player_entity: Res<PlayerEntity>,
    mut target_q: Query<&mut TargetVisualizer>,
    mut mobs_q: Query<(&Position, &Name, &mut AIComponent)>,
    mut action_q: Query<(&Actor, &mut ActionState), With<AttackActor>>,
) {
    use ActionState::*;

    let player_position = match mobs_q.get(player_entity.current()) {
        Ok((p, ..)) => *p,
        Err(err) => {
            info!("No player found: {}", err);
            return;
        },
    };

    for (Actor(actor), mut action_state) in action_q.iter_mut() {
        let Ok((ai_position, name, mut ai_component)) =
        mobs_q.get_mut(*actor) else {
            info!("Actor must have required attack components");
            continue
        };

        if ai_component.preferred_action.is_some() {
            // already attacking, quick return;
            continue;
        }

        match *action_state {
            // Success | Failure
            Success | Failure => {
                // Nothing to do here
                info!("{} attack state: {:?}", name, action_state);
                continue;
            },
            ActionState::Cancelled => {
                info!("{} cancelled attack!", name);
                *action_state = Failure;
                ai_component.preferred_action = None;

                if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                    target_visualizer.clear(&mut commands);
                }

                continue;
            },

            // these final two fall through to logic
            ActionState::Init | ActionState::Requested => {
                info!("{} gonna start attacking!", name);
                *action_state = Executing;
                ai_component.preferred_action = Some(ActionType::Attack(player_position));

                if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                    target_visualizer.set_color(Color::RED);
                    target_visualizer.set_style(TargetVisualizerStyle::Target);
                }
            },
            ActionState::Executing => {},
        }

        if in_attack_range(*ai_position, player_position) {
            println!("{} is in attack range!", name);
            // *action_state = ActionState::Success;
            ai_component.preferred_action = Some(ActionType::Attack(player_position));
        } else {
            *action_state = ActionState::Failure;
            ai_component.preferred_action = Some(ActionType::Movement(player_position));
        }
    }
}
