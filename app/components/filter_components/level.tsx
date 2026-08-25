import { RangeSlider } from "@mantine/core";
import { LevelFilter } from "@/pkg/dndlib";
import FilterContainer from "../ui/filter_container";

interface FilterLevelProps {
  level: LevelFilter;
  setLevel: (levelFilter: LevelFilter) => void;
};

export default function FilterLevel({ level, setLevel }: FilterLevelProps) {
  const setLevelFromSlider = (values: [number, number]) => {
    setLevel({
      min: values[0],
      max: values[1]
    });
  }
  return <FilterContainer name="Niveau">
    <RangeSlider
      value={[level.min, level.max]}
      onChange={setLevelFromSlider}
      min={0}
      max={9}
      minRange={0}
      label={null}
      marks={[
        { value: 0, label: "0" },
        { value: 1, label: "1" },
        { value: 2, label: "2" },
        { value: 3, label: "3" },
        { value: 4, label: "4" },
        { value: 5, label: "5" },
        { value: 6, label: "6" },
        { value: 7, label: "7" },
        { value: 8, label: "8" },
        { value: 9, label: "9" },
      ]}
      restrictToMarks
      mb={24}
    />
  </FilterContainer>
}
