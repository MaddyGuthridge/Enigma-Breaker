use std::iter;

use crate::{Letter, MachineState, ReflectorId, RotorId};

/// Returns an iterator over all possible states of the machine
///
/// # Args
///
/// * `plugs_iter`: iterator over all possible plug combinations
/// * `reflector`: vec of all possible reflectors
/// * `rotor_ids`: vec of all possible sets of rotor IDs
/// * `rotor_positions`: vec of all possible positions of rotor ID sets
pub fn iter_possible_states<'a>(
    mut plugs_iter: Box<dyn Iterator<Item = Vec<(Letter, Letter)>>>,
    reflector: &'a [ReflectorId],
    rotor_ids: &'a [Vec<RotorId>],
    rotor_positions: &'a [Vec<Letter>],
) -> impl Iterator<Item = MachineState> + 'a {
    let mut plugs_curr = plugs_iter.next();

    let mut reflector_iter = reflector.iter();
    let mut reflector_curr = reflector_iter.next();

    let mut rotor_iter = rotor_ids.iter();
    let mut rotor_curr = rotor_iter.next();

    let mut rotor_pos_iter = rotor_positions.iter();
    let mut rotor_pos_curr = rotor_pos_iter.next();

    iter::from_fn(move || {
        // Make everything be not None

        // First the rotor positions
        let rotors_pos = if let Some(pos) = rotor_pos_curr {
            pos
        } else {
            // Reset this iterator and step the next one
            rotor_pos_iter = rotor_positions.iter();
            rotor_pos_curr = rotor_pos_iter.next();

            rotor_curr = rotor_iter.next();

            // If it's still None, give up
            rotor_pos_curr?
        };

        // Then the rotor IDs
        let rotors_id = if let Some(id) = rotor_curr {
            id
        } else {
            // Reset this iterator and step the next one
            rotor_iter = rotor_ids.iter();
            rotor_curr = rotor_iter.next();

            reflector_curr = reflector_iter.next();

            // If it's still None, give up
            rotor_curr?
        };
        let reflector_id = if let Some(id) = reflector_curr {
            id
        } else {
            // Reset this iterator and step the next one
            reflector_iter = reflector.iter();
            reflector_curr = reflector_iter.next();

            plugs_curr = plugs_iter.next();

            // If it's still None, give up
            reflector_curr?
        };

        // Finally, if we've run out of plug combinations, that means there is
        // no more iteration left to do
        let plugs = plugs_curr.as_ref()?;

        // At this point, none of the required properties are `None`, meaning
        // we can safely make a machine state. Wow this makes me wish that
        // generators were stable (https://github.com/rust-lang/rust/issues/43122)
        // since it'd be sooooooooooo much nicer to yield values here

        let state = MachineState::new(
            plugs.clone(),
            rotors_id.clone(),
            rotors_pos.clone(),
            *reflector_id,
        );

        // Now that we've made it, step to the next rotor position
        rotor_pos_curr = rotor_pos_iter.next();

        Some(state)
    })
}
