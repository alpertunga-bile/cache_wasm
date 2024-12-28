import { Cacher, CacherOptions, CacherReturnInfo } from "../pkg/cacher_wasm.js";

if (import.meta.main) {
  const cacher_opt = new CacherOptions();

  const cacher = new Cacher(cacher_opt);

  const info = cacher.get_compressed_info("asd");

  const return_info: CacherReturnInfo = cacher.get_data(info);

  console.log(return_info);
}
