import { ActionFilter, ClassFilter, ComponentsFilter, DurationFilter, LevelFilter, MagicSchoolFilter, SourceFilter, SpellFilter } from "@/pkg/dndlib"
import { ActionIcon, Affix, Drawer, Stack, TextInput } from "@mantine/core";
import FilterClass from "./filter_components/class";
import FilterLevel from "./filter_components/level";
import FilterAction from "./filter_components/action";
import FiltersSelect from "./filter_components/filter_select";
import { Dispatch, SetStateAction } from "react";
import FilterMagicSchool from "./filter_components/school";
import FilterComponents from "./filter_components/components";
import FilterDuration from "./filter_components/duration";
import FilterSource from "./filter_components/source";
import { useDisclosure } from "@mantine/hooks";
import { FunnelIcon } from "@phosphor-icons/react";


export interface FilterProps {
  filter: SpellFilter;
  setFilter: Dispatch<SetStateAction<SpellFilter>>;
}

export default function Filter({ filter, setFilter }: FilterProps) {

  const setLevelFilter = (levelFilter: LevelFilter) => {
    setFilter({ ...filter, ...{ level: levelFilter } });
  };
  const setClassFilter = (classFilter: ClassFilter) => {
    setFilter({ ...filter, ...{ class: classFilter } });
  };
  const setActionFilter = (actionFilter: ActionFilter) => {
    setFilter({ ...filter, ...{ action: actionFilter } });
  };
  const setMagicSchoolFilter = (magicSchoolFilter: MagicSchoolFilter) => {
    setFilter({ ...filter, ...{ school: magicSchoolFilter } });
  };
  const setComponentsFilter = (componentsFilter: ComponentsFilter) => {
    setFilter({ ...filter, ...{ components: componentsFilter } });
  };
  const setDurationFilter = (durationFilter: DurationFilter) => {
    setFilter({ ...filter, ...{ duration: durationFilter } });
  };
  const setSourceFilter = (sourceFilter: SourceFilter) => {
    setFilter({ ...filter, ...{ source: sourceFilter } });
  };

  const [opened, { open, close }] = useDisclosure(false);

  return (
    <>
      <TextInput placeholder="Tapez le nom d'un sort" onChange={(event) => {
        filter.search = event.target.value;
        setFilter({
          ...filter,
          ...{ "search": event.target.value }
        })
      }} />
      <Affix position={{ bottom: 50, right: 40 }} onClick={open}>
        <ActionIcon size={120} radius={60}>
          <FunnelIcon size={60} />
        </ActionIcon>
      </Affix>
      <Drawer opened={opened} onClose={close} title="Filtres">
        <Stack gap={18}>
          {filter.level && <FilterLevel level={filter.level} setLevel={setLevelFilter} />}
          {filter.class && <FilterClass classFilter={filter.class} setClassFilter={setClassFilter} />}
          {filter.action && <FilterAction actionFilter={filter.action} setActionFilter={setActionFilter} />}
          {filter.school && <FilterMagicSchool magicSchoolFilter={filter.school} setMagicSchoolFilter={setMagicSchoolFilter} />}
          {filter.components && <FilterComponents componentsFilter={filter.components} setComponentsFilter={setComponentsFilter} />}
          {filter.duration && <FilterDuration durationFilter={filter.duration} setDurationFilter={setDurationFilter} />}
          {filter.source && <FilterSource sourceFilter={filter.source} setSourceFilter={setSourceFilter} />}
          <FiltersSelect filter={filter} setFilter={setFilter} />
        </Stack>
      </Drawer>
    </>
  );
}
