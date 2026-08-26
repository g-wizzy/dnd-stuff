import { Spell } from "@/pkg/dndlib";
import { Stack } from "@mantine/core";
import { ActionTypeToFrench, ClassToFrench, DurationToFrench, MagicSchoolToFrench, RangeToFrench } from "../lib/translation_tables";

export default function SpellLong({ spell }: { spell: Spell }) {
  return <Stack>
    <h1>{spell.name}</h1>
    <p>
      {MagicSchoolToFrench(spell.school)} de niveau {spell.level} (
      {spell.classes.map(ClassToFrench).join(", ")}
      )
    </p>
    <p>
      <strong>Temps d'incantation: </strong>
      {ActionTypeToFrench(spell.action_cost.action_type)}
      {spell.action_cost.additional_info}
      {spell.action_cost.ritual && <> (Rituel)</>}
    </p>
    <p>
      <strong>Portée: </strong>
      {RangeToFrench(spell.range)}
    </p>
    <p>
      <strong>Composantes: </strong>
      {spell.components.verbal && "V"}
      {spell.components.somatic && "S"}
      {spell.components.material && `M (${spell.components.material.text})`}
    </p>
    <p>
      <strong>Durée: </strong>
      {DurationToFrench(spell.duration)}
    </p>
    <p>
      <strong>Description: </strong>
      <span dangerouslySetInnerHTML={{ __html: spell.description }} />
    </p>
  </Stack>
}
