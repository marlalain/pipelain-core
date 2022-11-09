use specs::{Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

use crate::components::items::{InBackpack, PickupQueue};
use crate::{Log, Name, Position};

pub struct PickupSystem {}

impl<'a> System<'a> for PickupSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, Log>,
        WriteStorage<'a, PickupQueue>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, InBackpack>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player, mut log, mut wants_pickup, mut positions, names, mut backpack) = data;

        for pickup in wants_pickup.join() {
            positions.remove(pickup.item);

            backpack
                .insert(
                    pickup.item,
                    InBackpack {
                        owner: pickup.collected_by,
                    },
                )
                .expect("unable to add to backpack");

            if pickup.collected_by == *player {
                log.log(format!(
                    "you pick up the {}",
                    names.get(pickup.item).unwrap()
                ))
            }
        }

        wants_pickup.clear();
    }
}
