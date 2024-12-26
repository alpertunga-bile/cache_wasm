import { greet } from "../pkg/cacher_wasm.js";

// Learn more at https://docs.deno.com/runtime/manual/examples/module_metadata#concepts
if (import.meta.main) {
  const temp = greet();
  console.log(temp);
}
