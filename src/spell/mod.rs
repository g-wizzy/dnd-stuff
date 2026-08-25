mod magic_school;
pub use magic_school::MagicSchool;

mod class;
pub use class::Class;

mod action;
pub use action::{ActionCost, ActionType};

mod source;
pub use source::Source;

mod component;
pub use component::SpellComponents;

mod range;
pub use range::{Distance, Range, RangeType};

mod duration;
pub use duration::{Duration, DurationType, TimeUnit};

mod spell;
pub use spell::Spell;

mod serialized;
pub(crate) use serialized::get_spells_ron;
