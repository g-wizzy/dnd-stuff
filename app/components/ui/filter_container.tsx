import { Group, Paper, Stack, Title } from "@mantine/core";
import { XIcon } from "@phosphor-icons/react";
import { ReactNode } from "react";

interface FilterContainerProps {
  name: string;
  onClose: () => void;
  children: ReactNode;
}

export default function FilterContainer({ name, onClose, children }: FilterContainerProps) {
  return <Paper
    withBorder
    p={12}
  >
    <Stack>
      <Group>
        <Title order={4}>
          {name}
        </Title>
        <XIcon onClick={onClose} />
      </Group>
      {children}
    </Stack>
  </Paper>;
}
