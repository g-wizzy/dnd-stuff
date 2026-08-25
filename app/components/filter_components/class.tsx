import { CLASSES_FRENCH } from "@/app/lib/translation_tables";
import { Class, ClassFilter } from "@/pkg/dndlib";
import { MultiSelect } from "@mantine/core";
import FilterContainer from "../ui/filter_container";


interface FilterClassProps {
  classFilter: ClassFilter,
  setClassFilter: (classFilter: ClassFilter) => void;
};

export default function FilterClass({ classFilter, setClassFilter }: FilterClassProps) {
  const setClassFilterFromSelect = (classes: Class[]) => {
    setClassFilter({ classes: classes });
  }
  return <FilterContainer name="Classe">
    <MultiSelect<Class>
      data={CLASSES_FRENCH}
      value={classFilter.classes}
      onChange={setClassFilterFromSelect}
      clearable
      searchable
      hidePickedOptions
      clearSectionMode="clear"
    />
  </FilterContainer>
}
