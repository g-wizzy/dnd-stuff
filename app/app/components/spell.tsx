import { Spell } from "@/pkg/dnd_stuff";
import { Stack } from "@mantine/core";

export default function SpellContainer({ spell }: { spell: Spell }) {
  return <>
    <Stack>
      <p>{spell.name}</p>
      <p>{spell.level}</p>
    </Stack>
  </>;
}
