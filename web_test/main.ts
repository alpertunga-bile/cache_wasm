import { Cacher, CacherOptions } from "../pkg/cacher_wasm.js";

if (import.meta.main) {
  const cacher_opt = new CacherOptions();

  const cacher = new Cacher(cacher_opt);

  console.log(cacher.save_file("asd", "asd"));
}
