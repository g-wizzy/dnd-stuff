import { Spell } from "@/pkg/dndlib";
import { Stack } from "@mantine/core";

export default function SpellLong({ spell }: { spell: Spell }) {
  return <Stack>
    <h1>{spell.name}</h1>
    <p>
      {spell.school} de niveau {spell.level} (
      {spell.classes.join(", ")}
      )
    </p>
    <p>
      <strong>Temps d'incantation: </strong>
      {spell.action_cost.action_type}
      {
        // TODO: Create rust methods for to-string this shit
      }
    </p>
  </Stack>
}
