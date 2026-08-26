import { ACTION_TYPES_FRENCH } from "@/app/lib/translation_tables";
import { ActionFilter, ActionType } from "@/pkg/dndlib";
import { MultiSelect } from "@mantine/core";
import NullableBool from "../ui/nullable_bool";
import FilterContainer from "../ui/filter_container";

interface FilterActionProps {
  actionFilter: ActionFilter,
  setActionFilter: (actionFilter: ActionFilter) => void;
  onDelete: () => void;
}

export default function FilterAction({ actionFilter, setActionFilter, onDelete }: FilterActionProps) {
  const setActionFilterFromSelect = (actionTypes: ActionType[]) => {
    setActionFilter({
      ...actionFilter,
      ...{ action_types: actionTypes }
    });
  };
  const setRitual: (value: boolean | null) => void = (value) => {
    setActionFilter({
      ...actionFilter,
      ...{ ritual: value }
    })
  }
  return <FilterContainer name="Incantation" onClose={onDelete}>
    <MultiSelect<ActionType>
      data={ACTION_TYPES_FRENCH}
      value={actionFilter.action_types}
      onChange={setActionFilterFromSelect}
      clearable
      searchable
      hidePickedOptions
      clearSectionMode="clear"
    />
    <NullableBool
      textForTrue="Rituel"
      textForFalse="Pas rituel"
      textForNull="Les deux"
      value={actionFilter.ritual}
      setValue={setRitual}
    />
  </FilterContainer>
}
