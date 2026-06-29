import { useState, useEffect } from "react";

export function useWasm() {
  const [wasm, setWasm] = useState(null);

  useEffect(() => {
    async function load() {
      const module = await import("@/pkg/dndlib");
      await module.default();
      module.init();
      setWasm(module);
    }
    load();
  }, []);

  return wasm;
}
