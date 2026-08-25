import { MultiSelect } from "@mantine/core";
import { useState } from "react";
import { FilterProps } from "../filter";
import { SpellFilter } from "@/pkg/dndlib";
import { ALL_ACTION_TYPES, ALL_CLASSES, ALL_MAGIC_SCHOOLS, ALL_SOURCES } from "@/app/lib/translation_tables";
import FilterContainer from "../ui/filter_container";

type FilterKey = keyof SpellFilter;

interface FilterOption {
  value: FilterKey,
  label: string
}

const ALL_FILTERS: FilterOption[] = [
  { value: "level", label: "Niveau" },
  { value: "class", label: "Classe" },
  { value: "action", label: "Incantation" },
  { value: "school", label: "École de magie" },
  { value: "components", label: "Composantes" },
  { value: "duration", label: "Durée" },
  // { value: "range", label: "Portée" },
  { value: "source", label: "Source" },
];



export default function FilterSelect({ filter, setFilter }: FilterProps) {
  const [filtersStrings, setFiltersStrings] = useState<FilterKey[]>();

  const updateStrings: (values: FilterKey[]) => void = (values) => {
    const current_filters = new Set(filtersStrings);
    const updated_filters = new Set(values);

    const to_remove = current_filters.difference(updated_filters);
    const to_add = updated_filters.difference(current_filters);

    for (const filter_key of to_remove) {
      if (filter_key === "level")
        setFilter({ ...filter, ...{ level: null } });
      else if (filter_key === "action")
        setFilter({ ...filter, ...{ action: null } });
      else if (filter_key === "class")
        setFilter({ ...filter, ...{ class: null } });
      else if (filter_key === "school")
        setFilter({ ...filter, ...{ school: null } });
      else if (filter_key === "components")
        setFilter({ ...filter, ...{ components: null } });
      else if (filter_key === "duration")
        setFilter({ ...filter, ...{ duration: null } });
      else if (filter_key === "source")
        setFilter({ ...filter, ...{ source: null } });
    }

    for (const filter_key of to_add) {
      if (filter_key === "level")
        setFilter({ ...filter, ...{ level: { min: 0, max: 9 } } });
      else if (filter_key === "action")
        setFilter({ ...filter, ...{ action: { action_types: ALL_ACTION_TYPES, ritual: null } } });
      else if (filter_key === "class")
        setFilter({ ...filter, ...{ class: { classes: ALL_CLASSES } } });
      else if (filter_key === "school")
        setFilter({ ...filter, ...{ school: { schools: ALL_MAGIC_SCHOOLS } } });
      else if (filter_key === "components")
        setFilter({ ...filter, ...{ components: { verbal: null, somatic: null, material: null } } });
      else if (filter_key === "duration")
        setFilter({ ...filter, ...{ duration: { concentration: null } } });
      else if (filter_key === "source")
        setFilter({ ...filter, ...{ source: { sources: ALL_SOURCES } } });
    }

    setFiltersStrings(values);
  }
  return <FilterContainer name="Sélectionnez les filtres">
    <MultiSelect<FilterKey>
      data={ALL_FILTERS}
      value={filtersStrings}
      onChange={updateStrings}
    />
  </FilterContainer>;
}
