"use client";

import { Class, Spell, SpellFilter } from "@/pkg/dndlib";
import { MantineProvider, MultiSelect, TextInput } from "@mantine/core";
import { useState } from "react";
import SpellContainer from "./components/spell";
import { useWasm } from "./hooks/useWasm";

export default function Home() {

  const wasm = useWasm();
  const [spells, setSpells] = useState<Array<Spell>>([]);

  const [filter, _setFilter] = useState<SpellFilter>({
    search: null,
    level: null,
    action_type: null,
    ritual: null,
    school: null,
    class: null,
    source: null,
    component_verbal: null,
    component_somatic: null,
    component_material: null,
    range: null,
    duration_type: null,
    concentration: null
  })

  const setFilter = (value) => {
    _setFilter(value);
    if (!wasm) return;
    setSpells(wasm.get_spells(filter));
  }

  return (
    <MantineProvider>
      <TextInput label="name" onChange={(event) => {
        setFilter((filter) => ({
          ...filter,
          ...{ "search": event.target.value }
        }))
      }} />
      <MultiSelect<Class>
        label="Classes"
        data={["Bard", "Cleric", "Druid", "Paladin", "Ranger", "Sorcerer", "Warlock", "Wizard", "Artificer"]}
      />

      {
        spells.map(spell => (
          <SpellContainer spell={spell} key={spell.name} />
        ))
      }
    </MantineProvider>
  );
}
