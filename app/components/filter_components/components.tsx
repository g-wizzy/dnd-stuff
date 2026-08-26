import { ComponentsFilter } from "@/pkg/dndlib";
import NullableBool from "../ui/nullable_bool";
import FilterContainer from "../ui/filter_container";

interface FilterComponentsProps {
  componentsFilter: ComponentsFilter;
  setComponentsFilter: (componentsFilter: ComponentsFilter) => void;
  onDelete: () => void;
}

export default function FilterComponents({ componentsFilter, setComponentsFilter, onDelete }: FilterComponentsProps) {
  const setVerbalComponentFilter = (value: boolean | null) => {
    setComponentsFilter({ ...componentsFilter, ...{ verbal: value } });
  };
  const setSomaticComponentFilter = (value: boolean | null) => {
    setComponentsFilter({ ...componentsFilter, ...{ somatic: value } });
  };
  const setMaterialComponentFilter = (value: boolean | null) => {
    setComponentsFilter({ ...componentsFilter, ...{ material: value } });
  };

  return <FilterContainer name="Composantes" onClose={onDelete}>
    <NullableBool
      textForTrue="Verbal"
      textForNull="Les deux"
      textForFalse="Non-verbal"
      value={componentsFilter.verbal}
      setValue={setVerbalComponentFilter}
    />
    <NullableBool
      textForTrue="Somatique"
      textForNull="Les deux"
      textForFalse="Non-somatique"
      value={componentsFilter.somatic}
      setValue={setSomaticComponentFilter}
    />
    <NullableBool
      textForTrue="Matériel"
      textForNull="Les deux"
      textForFalse="Non-matériel"
      value={componentsFilter.material}
      setValue={setMaterialComponentFilter}
    />
  </FilterContainer>;
}
