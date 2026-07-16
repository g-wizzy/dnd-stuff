import { Spell } from "@/pkg/dndlib";
import { Modal, Text } from "@mantine/core";
import { Paper } from "@mantine/core";
import { Stack } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import SpellLong from "./spell_long";

export default function SpellShort({ spell }: { spell: Spell }) {
  const [opened, { open, close }] = useDisclosure(false);

  return (
    <>
      <Paper
        withBorder
        radius="sm"
        shadow="md"
        onClick={open}
      >
        <Stack>
          <div>
            <Text fw={700}>{spell.level}</Text> {spell.name}
          </div>
        </Stack>
      </Paper>

      <Modal opened={opened} onClose={close} title={spell.name}>
        <SpellLong spell={spell} />
      </Modal>
    </>
  );
}
