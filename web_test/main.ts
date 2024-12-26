import { greet } from "../pkg/cacher_wasm.js";

if (import.meta.main) {
  console.log(greet());
}
