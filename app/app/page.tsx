"use client";

import { Spell, SpellFilter } from "@/pkg/dndlib";
import { MantineProvider } from "@mantine/core";
import { useEffect, useState } from "react";
import SpellShort from "./components/spell_short";
import { useWasm } from "./hooks/useWasm";
import Filter from "./components/filter";

export default function Home() {

  const wasm = useWasm();

  const [filter, setFilter] = useState<SpellFilter>({
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
  const [spells, setSpells] = useState<Array<Spell>>([]);

  useEffect(() => {
    if (!wasm) return;
    setSpells(wasm.get_spells(filter));
    console.log(filter)
  }, [wasm, filter]);

  return (
    <MantineProvider>
      <Filter filter={filter} setFilter={setFilter} />
      {
        spells.map(spell => (
          <SpellShort spell={spell} key={spell.name} />
        ))
      }
    </MantineProvider>
  );
}
