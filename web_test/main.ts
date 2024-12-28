import {
  CacherOptions,
  CacherReturnInfo,
  get_compressed_cacher_info,
  get_decompressed_data,
} from "../pkg/cacher_wasm.js";

if (import.meta.main) {
  const cacher_opt = new CacherOptions();

  const info = get_compressed_cacher_info(cacher_opt, "asd");

  const return_info: CacherReturnInfo = get_decompressed_data(info);

  console.log(return_info.data);
}
