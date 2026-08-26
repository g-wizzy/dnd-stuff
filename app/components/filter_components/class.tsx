import { CLASSES_FRENCH } from "@/app/lib/translation_tables";
import { Class, ClassFilter } from "@/pkg/dndlib";
import { MultiSelect } from "@mantine/core";
import FilterContainer from "../ui/filter_container";


interface FilterClassProps {
  classFilter: ClassFilter,
  setClassFilter: (classFilter: ClassFilter) => void;
  onDelete: () => void;
};

export default function FilterClass({ classFilter, setClassFilter, onDelete }: FilterClassProps) {
  const setClassFilterFromSelect = (classes: Class[]) => {
    setClassFilter({ classes: classes });
  }
  return <FilterContainer name="Classe" onClose={onDelete}>
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
