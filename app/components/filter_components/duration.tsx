import { DurationFilter } from "@/pkg/dndlib";
import NullableBool from "../ui/nullable_bool";
import FilterContainer from "../ui/filter_container";

interface FilterDurationProps {
  durationFilter: DurationFilter;
  setDurationFilter: (durationFilter: DurationFilter) => void;
  onDelete: () => void;
}

export default function FilterDuration({ durationFilter, setDurationFilter, onDelete }: FilterDurationProps) {
  const setConcentration = (concentration: boolean | null) => {
    setDurationFilter({ ...durationFilter, ...{ concentration: concentration } });
  };

  return <FilterContainer name="Durée" onClose={onDelete}>
    <NullableBool
      textForTrue="Concentration"
      textForNull="Les deux"
      textForFalse="Sans concentration"
      value={durationFilter.concentration}
      setValue={setConcentration}
    />
  </FilterContainer>;
}
