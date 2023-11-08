use std::iter;

use itertools::Itertools;

use crate::{machine::PlugBoard, Letter, MachineState, ReflectorId, RotorId};

/// Returns an iterator over all possible states of the machine
///
/// # Args
///
/// * `plugs_iter`: iterator over all possible plug combinations
///
/// * `reflector`: vec of all possible reflectors
///
/// * `rotor_ids`: vec of rotors - each rotor is a vec of all of its possible
///   IDs
///
/// * `rotor_positions`: vec of starting positions for each rotor - each rotor
///   has a starting position represented by a vec of starting positions
pub fn iter_possible_states<'a>(
    mut plugs_iter: Box<dyn Iterator<Item = Vec<(Letter, Letter)>> + Send>,
    reflector: &'a [ReflectorId],
    rotor_ids: &'a [Vec<RotorId>],
    rotor_positions: &'a [Vec<Letter>],
) -> impl Iterator<Item = MachineState> + 'a {
    let mut plugs_curr = plugs_iter.next();

    let mut reflector_iter = reflector.iter();
    let mut reflector_curr = reflector_iter.next();

    let mut rotor_iter = rotor_ids.iter().multi_cartesian_product();
    let mut rotor_curr = rotor_iter.next();

    let mut rotor_pos_iter = rotor_positions.iter().multi_cartesian_product();
    let mut rotor_pos_curr = rotor_pos_iter.next();

    // iter::from_fn is the closest we can get to generators, at least without
    // changing to Rust Nightly (which sounds scary)
    // https://stackoverflow.com/a/58683171/6335363
    iter::from_fn(move || {
        // Make everything be not None

        // First the rotor positions
        let rotors_pos = if let Some(pos) = &rotor_pos_curr {
            pos
        } else {
            // Reset this iterator and step the next one
            rotor_pos_iter = rotor_positions.iter().multi_cartesian_product();
            rotor_pos_curr = rotor_pos_iter.next();

            rotor_curr = rotor_iter.next();

            // If it's still None, give up
            rotor_pos_curr.as_ref()?
        };

        // Then the rotor IDs
        let rotors_id = if let Some(id) = &rotor_curr {
            id
        } else {
            // Reset this iterator and step the next one
            rotor_iter = rotor_ids.iter().multi_cartesian_product();
            rotor_curr = rotor_iter.next();

            reflector_curr = reflector_iter.next();

            // If it's still None, give up
            rotor_curr.as_ref()?
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

        // Finally, keep iterating through plug combinations until we find one
        // that's valid
        // If we run out (ie plugs_curr is None), there are no more
        // combinations to check
        let plugs = loop {
            let plugs_temp = plugs_curr.as_ref()?;
            if PlugBoard::new(plugs_temp).is_some() {
                break plugs_temp;
            }
            plugs_curr = plugs_iter.next();
        };

        // At this point, none of the required properties are `None`, meaning
        // we can safely make a machine state. Wow this makes me wish that
        // generators were stable (https://github.com/rust-lang/rust/issues/43122)
        // since it'd be sooooooooooo much nicer to yield values here

        let state = MachineState::new(
            plugs.clone(),
            // Since .iter() gives an iterator over references, we need to
            // clone twice since the type is &&T
            // https://stackoverflow.com/a/53116103/6335363
            rotors_id.iter().cloned().cloned().collect_vec(),
            rotors_pos.iter().cloned().cloned().collect_vec(),
            *reflector_id,
        );

        // Now that we've figured out this state, step to the next rotor
        // position
        rotor_pos_curr = rotor_pos_iter.next();

        Some(state)
    })
}
