import { Group, Paper, Stack, Title, UnstyledButton } from "@mantine/core";
import { XIcon } from "@phosphor-icons/react";
import { ReactNode } from "react";

interface FilterContainerProps {
  name: string;
  onClose: (() => void) | null;
  children: ReactNode;
}

export default function FilterContainer({ name, onClose, children }: FilterContainerProps) {
  return <Paper
    withBorder
    p={12}
  >
    <Stack>
      <Group justify="space-between">
        <Title order={4}>
          {name}
        </Title>
        {
          onClose &&
          <UnstyledButton onClick={onClose}>
            <XIcon onClick={onClose} />
          </UnstyledButton>
        }
      </Group>
      {children}
    </Stack>
  </Paper>;
}
