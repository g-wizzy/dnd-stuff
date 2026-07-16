import { useState, useEffect } from "react";

export function useWasm() {
  const [wasm, setWasm] = useState(null);

  useEffect(() => {
    async function load() {
      const module = await import("@/pkg/dndlib.js");
      await module.default();
      setWasm(module);
      module.init();
    }
    load();
  }, []);

  return wasm;
}
