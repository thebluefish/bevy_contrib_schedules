use bevy::{
    app::stage,
    core::Time,
    ecs::{Schedule, ParallelExecutor, World, Resources, System, Entity},
    utils::HashMap,
};

/// Determines how the schedule should run
#[derive(Debug)]
pub enum ScheduleType { // The Schedule runs with...
    // ... Every frame
    Always,
    // ... A fixed tick cycle
    Fixed(f64, f64), // (rate, accumulator)
    // TODO: Figure out how to make this more useful?
    // ... A user-provided fn
    // With(Box<dyn FnMut(&mut PackedSchedule, &mut World, &mut Resources) + Send + Sync>),
}

/// The PackedSchedule is responsible for actual execution
/// You probably won't need to touch this directly
pub struct PackedSchedule(pub ScheduleType, pub Schedule, ParallelExecutor);

impl Default for PackedSchedule {
    fn default() -> Self {
        PackedSchedule(
            ScheduleType::Always,
            Default::default(),
            ParallelExecutor::without_tracker_clears(),
        )
    }
}

impl PackedSchedule {
    fn run(&mut self, mut world: &mut World, mut resources: &mut Resources) {
        self.1.initialize(world, resources);

        match &mut self.0 {
            ScheduleType::Always => {
                self.2.run(&mut self.1, &mut world, &mut resources);
            },
            ScheduleType::Fixed(rate, accumulator) => {
                // Accumulate time
                match resources.get::<Time>() {
                    Some(time) => {
                        *accumulator += time.delta_seconds_f64;
                    },
                    None => log::debug!("Time does not exist, Fixed Schedule cannot run!"),
                };

                // Run fixed-interval ticks
                while accumulator >= rate {
                    self.2.run(&mut self.1, &mut world, &mut resources);
                    *accumulator -= *rate;
                }
            },
        };
    }
}

/// Responsible for holding the data in Bevy
/// Use as a Resource or Component
pub struct ScheduleRunner(pub PackedSchedule);

impl Default for ScheduleRunner {
    fn default() -> Self {
        ScheduleRunner(PackedSchedule { 0: ScheduleType::Always , .. Default::default() })
            .add_default_stages()
    }
}

/// Good portion taken from bevy::AppBuilder for convenience
impl ScheduleRunner {
    /// A fixed-rate runner that runs every `rate` seconds
    pub fn from_rate(rate: f64) -> Self {
        ScheduleRunner(PackedSchedule { 0: ScheduleType::Fixed(rate, 0.0) , .. Default::default() })
            .add_default_stages()
    }

    /// A fixed-rate runner that runs `rate` per second
    pub fn from_rate_inv(rate: f64) -> Self {
        Self::from_rate(1.0 / rate)
    }

    // TODO: Figure out how we should support this stuff
    // A runner executed by a user-provided fn
    // pub fn from_fn<F>(f: F) -> Self
    // where F: FnMut(&mut PackedSchedule, &mut World, &mut Resources) + Send + Sync + 'static {
    //     ScheduleRunner(PackedSchedule { 0: ScheduleType::With(Box::new(f)) , .. Default::default() })
    // }

    pub fn add_default_stages(self) -> Self {
        self.add_stage(stage::FIRST)
            .add_stage(stage::PRE_UPDATE)
            .add_stage(stage::UPDATE)
            .add_stage(stage::POST_UPDATE)
            .add_stage(stage::LAST)
    }

    pub fn add_stage(mut self, stage_name: &'static str) -> Self {
        self.0.1.add_stage(stage_name);
        self
    }

    pub fn add_stage_after(mut self, target: &'static str, stage_name: &'static str) -> Self {
        self.0.1.add_stage_after(target, stage_name);
        self
    }

    pub fn add_stage_before(
        mut self,
        target: &'static str,
        stage_name: &'static str,
    ) -> Self {
        self.0.1.add_stage_before(target, stage_name);
        self
    }

    pub fn add_system(self, system: Box<dyn System>) -> Self {
        self.add_system_to_stage(stage::UPDATE, system)
    }

    pub fn add_systems(self, systems: Vec<Box<dyn System>>) -> Self {
        self.add_systems_to_stage(stage::UPDATE, systems)
    }

    pub fn add_system_to_stage(
        mut self,
        stage_name: &'static str,
        system: Box<dyn System>,
    ) -> Self {
        self.0.1.add_system_to_stage(stage_name, system);
        self
    }

    pub fn add_system_to_stage_front(
        mut self,
        stage_name: &'static str,
        system: Box<dyn System>,
    ) -> Self {
        self.0.1.add_system_to_stage_front(stage_name, system);
        self
    }

    pub fn add_systems_to_stage(
        mut self,
        stage_name: &'static str,
        systems: Vec<Box<dyn System>>,
    ) -> Self {
        for system in systems {
            self.0.1.add_system_to_stage(stage_name, system);
        }
        self
    }
}

/// System responsible for executing all schedules
/// You should add it to your AppBuilder or parent schedule manually
pub fn schedule_runner_system(mut world: &mut World, mut resources: &mut Resources) {
    // Run it as a resource
    if resources.contains::<ScheduleRunner>() {
        // rip and tear
        let mut schedule = std::mem::take(&mut resources.get_mut::<ScheduleRunner>().unwrap().0);
        schedule.run(&mut world, &mut resources);
        resources.get_mut::<ScheduleRunner>().unwrap().0 = schedule;
    }

    // Run it as a component
    // We take all components, run them, put them back
    let mut entity_map: HashMap<Entity, PackedSchedule> = world.query_mut::<(Entity, &mut ScheduleRunner)>()
        .iter()
        .map(|(entity, mut runner)| (entity, std::mem::take(&mut runner.0)))
        .collect();
    for (_, schedule) in entity_map.iter_mut() {
        schedule.run(&mut world, &mut resources);
    }
    for (entity, mut runner) in &mut world.query_mut::<(Entity, &mut ScheduleRunner)>().iter() {
        if let Some(schedule) = entity_map.remove(&entity) {
            runner.0 = schedule;
        }
    }
}
