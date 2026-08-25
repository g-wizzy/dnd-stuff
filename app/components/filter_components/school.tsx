import { MAGIC_SCHOOLS_FRENCH } from "@/app/lib/translation_tables";
import { MagicSchool, MagicSchoolFilter } from "@/pkg/dndlib";
import { MultiSelect } from "@mantine/core";
import FilterContainer from "../ui/filter_container";

interface FilterMagicSchoolProps {
  magicSchoolFilter: MagicSchoolFilter;
  setMagicSchoolFilter: (magicSchoolFilter: MagicSchoolFilter) => void;
}

export default function FilterMagicSchool({ magicSchoolFilter, setMagicSchoolFilter }: FilterMagicSchoolProps) {
  const setMagicSchoolFilterFromSelect = (magicSchools: MagicSchool[]) => {
    setMagicSchoolFilter({
      ...magicSchoolFilter,
      ...{ schools: magicSchools }
    });
  };

  return <FilterContainer name="École de magie">
    <MultiSelect<MagicSchool>
      data={MAGIC_SCHOOLS_FRENCH}
      value={magicSchoolFilter.schools}
      onChange={setMagicSchoolFilterFromSelect}
      clearable
      searchable
      hidePickedOptions
      clearSectionMode="clear"
    />
  </FilterContainer>;
}
