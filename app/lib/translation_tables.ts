import { ActionType, Class, Distance, Duration, DurationType, MagicSchool, Range, RangeType, Source, TimedDuration, TimeUnit } from "@/pkg/dndlib";

type Labeled<T> = { value: T, label: string };

function unlabel<T>(table: Labeled<T>[]) {
  return table.map(item => item.value);
}

// CLASSES 
export const CLASSES_FRENCH: Labeled<Class>[] = [
  { value: Class.Bard, label: "Barde" },
  { value: Class.Cleric, label: "Clerc" },
  { value: Class.Druid, label: "Druide" },
  { value: Class.Paladin, label: "Paladin" },
  { value: Class.Ranger, label: "Rôdeur" },
  { value: Class.Sorcerer, label: "Ensorceleur" },
  { value: Class.Warlock, label: "Occultiste" },
  { value: Class.Wizard, label: "Magicien" },
  { value: Class.Artificer, label: "Artificier" }
];
export function ClassToFrench(klass: Class): string {
  return CLASSES_FRENCH.find(labeled => labeled.value === klass)!.label;
}
export const ALL_CLASSES = unlabel(CLASSES_FRENCH);

// ACTION TYPES
export const ACTION_TYPES_FRENCH: Labeled<ActionType>[] = [
  { value: ActionType.Action, label: "Action" },
  { value: ActionType.BonusAction, label: "Action bonus" },
  { value: ActionType.Reaction, label: "Réaction" },
  { value: ActionType.Long, label: "Action longue" },
];
export function ActionTypeToFrench(actionType: ActionType): string {
  return ACTION_TYPES_FRENCH.find(labeled => labeled.value === actionType)!.label;
}
export const ALL_ACTION_TYPES = unlabel(ACTION_TYPES_FRENCH);

// MAGIC SCHOOLS
export const MAGIC_SCHOOLS_FRENCH: Labeled<MagicSchool>[] = [
  { value: MagicSchool.Abjuration, label: "Abjuration" },
  { value: MagicSchool.Conjuration, label: "Invocation" },
  { value: MagicSchool.Divination, label: "Divination" },
  { value: MagicSchool.Enchantment, label: "Enchantement" },
  { value: MagicSchool.Evocation, label: "Évocation" },
  { value: MagicSchool.Illusion, label: "Illusion" },
  { value: MagicSchool.Necromancy, label: "Nécromancie" },
  { value: MagicSchool.Transmutation, label: "Transmutation" },
];
export function MagicSchoolToFrench(magicSchool: MagicSchool): string {
  return MAGIC_SCHOOLS_FRENCH.find(labeled => labeled.value === magicSchool)!.label;
}
export const ALL_MAGIC_SCHOOLS = unlabel(MAGIC_SCHOOLS_FRENCH);

// DURATION
export const DURATION_TYPES_FRENCH: Labeled<DurationType>[] = [
  { value: DurationType.Timed, label: "Durée déterminée" },
  { value: DurationType.Special, label: "Spéciale" },
  { value: DurationType.Dispelled, label: "Jusqu'à dissipation" },
  { value: DurationType.DispelledOrTriggered, label: "Jusqu'à dissipation ou déclenchement" },
  { value: DurationType.Instantaneous, label: "Instantanée" }
];
export function DurationTypeToFrench(durationType: DurationType): string {
  return DURATION_TYPES_FRENCH.find(labeled => labeled.value === durationType)!.label;
}
export const TIME_UNIT_FRENCH: Labeled<TimeUnit>[] = [
  { value: TimeUnit.Day, label: "jour" },
  { value: TimeUnit.Hour, label: "heure" },
  { value: TimeUnit.Minute, label: "minute" },
  { value: TimeUnit.Round, label: "round" },
];
export function TimeUnitToFrench(time_unit: TimeUnit): string {
  return TIME_UNIT_FRENCH.find(labeled => labeled.value === time_unit)!.label;
}
export function TimedDurationToFrench(timedDuration: TimedDuration): string {
  const plural = timedDuration.value > 1;
  return `${timedDuration.value} ${TimeUnitToFrench(timedDuration.unit)}${plural ? "s" : ""}`;
}
export function DurationToFrench(duration: Duration): string {
  if (duration.duration_type === DurationType.Timed) {
    return `${duration.concentration ? "Concentration, jusqu'à " : ""}\
      ${TimedDurationToFrench(duration.duration!)}`;
  } else {
    return DurationTypeToFrench(duration.duration_type);
  }
}
export const ALL_DURATION_TYPES = unlabel(DURATION_TYPES_FRENCH);

// RANGES
export const RANGE_TYPES_FRENCH: Labeled<RangeType>[] = [
  { value: RangeType.Self_, label: "Personnelle" },
  { value: RangeType.Touch, label: "Contact" },
  { value: RangeType.Sight, label: "Vue" },
  { value: RangeType.Special, label: "Spéciale" },
  { value: RangeType.Unlimited, label: "Illimitée" },
  { value: RangeType.Distance, label: "Distance" },
];
export function RangeTypeToFrench(rangeType: RangeType): string {
  return RANGE_TYPES_FRENCH.find(labeled => labeled.value === rangeType)!.label;
}
export function DistanceToFrench(distance: Distance): string {
  return `${distance.value * 1.5} ${distance.large_unit ? "km" : "m"}`;
}
export function RangeToFrench(range: Range): string {
  if (range.range_type === RangeType.Distance) {
    return `${DistanceToFrench(range.distance!)}`;
  } else {
    return RangeTypeToFrench(range.range_type);
  }
}
// SOURCES
export const SOURCES_FRENCH: Labeled<Source>[] = [
  { value: Source.PHB, label: "Player's Handbook 2024" },
  { value: Source.Eberron, label: "Eberron: Forge of the Artificer" },
  { value: Source.ForgottenRealms, label: "Forgotten Realms: Heroes of Faerûn" },
];
export function SourceToFrench(source: Source): string {
  return SOURCES_FRENCH.find(labeled => labeled.value === source)!.label;
}
export const ALL_SOURCES = unlabel(SOURCES_FRENCH);
