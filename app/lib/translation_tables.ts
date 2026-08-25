import { ActionType, Class, DurationType, MagicSchool, Source } from "@/pkg/dndlib";

type Labeled<T> = { value: T, label: string };

function unlabel<T>(table: Labeled<T>[]) {
  return table.map(item => item.value);
}

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
export const ALL_DURATION_TYPES = unlabel(DURATION_TYPES_FRENCH);

export const SOURCES_FRENCH: Labeled<Source>[] = [
  { value: Source.PHB, label: "Player's Handbook 2024" },
  { value: Source.Eberron, label: "Eberron: Forge of the Artificer" },
  { value: Source.ForgottenRealms, label: "Forgotten Realms: Heroes of Faerûn" },
];
export function SourceToFrench(source: Source): string {
  return SOURCES_FRENCH.find(labeled => labeled.value === source)!.label;
}
export const ALL_SOURCES = unlabel(SOURCES_FRENCH);
