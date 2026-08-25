import { useState, useEffect } from "react";

type DndWasmModule = {
  default: () => Promise<unknown>;
  init: () => unknown;
}

export function useWasm() {
  const [wasm, setWasm] = useState<DndWasmModule | null>(null);

  useEffect(() => {
    async function load() {
      const module = await import("@/pkg/dndlib.js") as unknown as DndWasmModule;
      await module.default();
      setWasm(module);
      module.init();
    }
    load();
  }, []);

  return wasm;
}
