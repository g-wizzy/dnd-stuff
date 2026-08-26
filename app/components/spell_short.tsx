import { Spell } from "@/pkg/dndlib";
import { Group, Modal, Text } from "@mantine/core";
import { Paper } from "@mantine/core";
import { useDisclosure, useMediaQuery } from "@mantine/hooks";
import SpellLong from "./spell_long";

export default function SpellShort({ spell }: { spell: Spell }) {
  const [opened, { open, close }] = useDisclosure(false);
  const isMobile = useMediaQuery("(max-width: 678px)");

  return (
    <>
      <Paper
        withBorder
        radius="md"
        shadow="md"
        m={6}
        px={12}
        onClick={open}
      >
        <Group h={64}>
          <Text fw={700}>{spell.level}</Text> <Text>{spell.name}</Text>
        </Group>
      </Paper>

      <Modal
        opened={opened}
        onClose={close}
        fullScreen={isMobile}
        title={spell.name}
      >
        <SpellLong spell={spell} />
      </Modal>
    </>
  );
}
