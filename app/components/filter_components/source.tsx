import { SOURCES_FRENCH } from "@/app/lib/translation_tables";
import { Source, SourceFilter } from "@/pkg/dndlib";
import { MultiSelect } from "@mantine/core";
import FilterContainer from "../ui/filter_container";

interface FilterSourceProps {
  sourceFilter: SourceFilter;
  setSourceFilter: (sourceFilter: SourceFilter) => void;
  onDelete: () => void;
}

export default function FilterSource({ sourceFilter, setSourceFilter, onDelete }: FilterSourceProps) {
  const setSourceFilterFromSelect = (sources: Source[]) => {
    setSourceFilter({ sources: sources });
  };

  return <FilterContainer name="Source" onClose={onDelete}>
    <MultiSelect<Source>
      label="Sources"
      data={SOURCES_FRENCH}
      value={sourceFilter.sources}
      onChange={setSourceFilterFromSelect}
      clearable
      searchable
      hidePickedOptions
    />
  </FilterContainer>;
}
