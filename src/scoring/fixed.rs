use bevy::{
    ecs::component::{ComponentHooks, Mutable, StorageType},
    prelude::*,
};

use crate::{ecs::CommandsExt, event::OnScore, scoring::Score};

/// [`Score`] [`Component`] that always scores a fixed value.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_observed_utility::prelude::*;
///
/// # let mut app = App::new();
/// # app.add_plugins(ObservedUtilityPlugins::RealTime);
/// # let mut world = app.world_mut();
/// # let mut commands = world.commands();
/// # let scorer =
/// commands
///     .spawn((FixedScore::new(0.5), Score::default()))
/// #   .id();
/// # commands.trigger_targets(RunScoring, scorer);
/// # world.flush();
/// # assert_eq!(world.get::<Score>(scorer).unwrap().get(), 0.5);
/// ```
#[derive(Reflect, Clone, Copy, PartialEq, Debug, Default)]
#[reflect(Component, PartialEq, Debug, Default)]
pub struct FixedScore {
    /// The fixed value to score.
    value: Score,
}

impl FixedScore {
    /// Creates a new [`FixedScore`] with the given value.
    #[must_use]
    pub fn new(value: impl Into<Score>) -> Self {
        Self { value: value.into() }
    }

    /// Returns the fixed value to score.
    #[must_use]
    pub fn value(&self) -> Score {
        self.value
    }

    /// Sets the fixed value to score.
    pub fn set_value(&mut self, value: impl Into<Score>) {
        self.value = value.into();
    }

    /// [`Observer`] for [`FixedScore`] [`Score`] entities that scores itself.
    fn observer(trigger: Trigger<OnScore>, mut target: Query<(&mut Score, &FixedScore)>) {
        let Ok((mut actor_score, settings)) = target.get_mut(trigger.target()) else {
            // The entity is not scoring for fixed.
            return;
        };

        *actor_score = settings.value();
    }
}

impl Component for FixedScore {
    type Mutability = Mutable;
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, _ctx| {
            #[derive(Resource, Default)]
            struct FixedScoreObserverSpawned;

            world
                .commands()
                .once::<FixedScoreObserverSpawned>()
                .observe(Self::observer);
        });
    }
}
