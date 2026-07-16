import { Class, SpellFilter } from "@/pkg/dndlib"
import { MultiSelect, RangeSlider, TextInput } from "@mantine/core";
import { useEffect, useState } from "react";

interface FilterProps {
  filter: SpellFilter;
  setFilter: (filter: SpellFilter) => void;
}
export default function Filter({ filter, setFilter }: FilterProps) {
  const allClasses: Class[] = ["Bard", "Cleric", "Druid", "Paladin", "Ranger", "Sorcerer", "Warlock", "Wizard", "Artificer"];
  const [classes, setClasses] = useState<Class[]>(allClasses);

  const [levelRange, setLevelRange] = useState<[number, number]>([0, 9]);

  useEffect(() => {
    setFilter({
      ...filter,
      ...{ "classes": classes },
      ...{ "level": levelRange[0] }
    })
  }, [classes, levelRange]);


  return (
    <>
      <TextInput label="name" onChange={(event) => {
        setFilter({
          ...filter,
          ...{ "search": event.target.value }
        })
      }} />
      <RangeSlider
        value={levelRange}
        onChange={setLevelRange}
        min={0}
        max={9}
        minRange={0}
        marks={[
          { value: 0 },
          { value: 1 },
          { value: 2 },
          { value: 3 },
          { value: 4 },
          { value: 5 },
          { value: 6 },
          { value: 7 },
          { value: 8 },
          { value: 9 },
        ]}
        restrictToMarks
      />
      <MultiSelect<Class>
        label="Classes"
        data={allClasses}
        value={classes}
        onChange={setClasses}
      />
    </>
  );
}
