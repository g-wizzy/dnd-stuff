import { Button, Flex } from "@mantine/core";
import { FilterProps } from "../filter";
import { ALL_ACTION_TYPES, ALL_CLASSES, ALL_MAGIC_SCHOOLS, ALL_SOURCES } from "@/app/lib/translation_tables";
import FilterContainer from "../ui/filter_container";

export default function FilterSelect({ filter, setFilter }: FilterProps) {
  const addLevelFilter = () => setFilter({ ...filter, ...{ level: { min: 0, max: 9 } } });
  const addActionFilter = () => setFilter({ ...filter, ...{ action: { action_types: ALL_ACTION_TYPES, ritual: null } } });
  const addClassFilter = () => setFilter({ ...filter, ...{ class: { classes: ALL_CLASSES } } });
  const addMagicSchoolFilter = () => setFilter({ ...filter, ...{ school: { schools: ALL_MAGIC_SCHOOLS } } });
  const addComponentsFilter = () => setFilter({ ...filter, ...{ components: { verbal: null, somatic: null, material: null } } });
  const addDurationFilter = () => setFilter({ ...filter, ...{ duration: { concentration: null } } });
  const addSourceFilter = () => setFilter({ ...filter, ...{ source: { sources: ALL_SOURCES } } });

  return <FilterContainer name="Ajouter des filtres" onClose={null}>
    <Flex wrap="wrap" gap="md">
      {!filter.level && <Button size="md" radius="xl" onClick={addLevelFilter}>Niveau</Button>}
      {!filter.action && <Button size="md" radius="xl" onClick={addActionFilter}>Incantation</Button>}
      {!filter.class && <Button size="md" radius="xl" onClick={addClassFilter}>Classe</Button>}
      {!filter.school && <Button size="md" radius="xl" onClick={addMagicSchoolFilter}>École</Button>}
      {!filter.components && <Button size="md" radius="xl" onClick={addComponentsFilter}>Composantes</Button>}
      {!filter.duration && <Button size="md" radius="xl" onClick={addDurationFilter}>Durée</Button>}
      {!filter.source && <Button size="md" radius="xl" onClick={addSourceFilter}>Source</Button>}
    </Flex>
  </FilterContainer>;
}
