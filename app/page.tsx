"use client";

import { Spell, SpellBook, SpellFilter } from "@/pkg/dndlib";
import { MantineProvider } from "@mantine/core";
import { useEffect, useRef, useState } from "react";
import SpellShort from "./components/spell_short";
import { useWasm } from "./hooks/useWasm";
import Filter from "./components/filter";

export default function Home() {

  const wasm = useWasm();

  const [filter, setFilter] = useState<SpellFilter>({
    search: "",
    level: null,
    school: null,
    action: null,
    duration: null,
    class: null,
    components: null,
    source: null,
  });
  const spellBook = useRef<SpellBook>(null);
  const [spells, setSpells] = useState<Spell[]>([]);
  const [doneLoaded, setDoneLoaded] = useState(false);

  useEffect(
    () => {
      if (!wasm || doneLoaded) return;
      spellBook.current = SpellBook.build();
      setSpells(spellBook.current.filter(filter));
      setDoneLoaded(true);
    },
    [wasm]
  );

  useEffect(
    () => {
      if (wasm && spellBook.current) {
        setSpells(spellBook.current.filter(filter))
      }
    },
    [filter]
  )

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

