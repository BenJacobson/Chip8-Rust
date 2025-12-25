/* tslint:disable */
/* eslint-disable */

export class WasmChip8 {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  get_display(): WasmDisplay;
  tick_timers(): void;
  load_program(program: Uint8Array): void;
  run_instructions(num_instructions: number): boolean;
  static new(): WasmChip8;
  set_keys(keys: number): void;
}

export class WasmDisplay {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  get_height(): number;
  get_pixel(x: number, y: number): boolean;
  get_width(): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_wasmchip8_free: (a: number, b: number) => void;
  readonly __wbg_wasmdisplay_free: (a: number, b: number) => void;
  readonly wasmchip8_get_display: (a: number) => number;
  readonly wasmchip8_load_program: (a: number, b: number, c: number) => void;
  readonly wasmchip8_new: () => number;
  readonly wasmchip8_run_instructions: (a: number, b: number) => number;
  readonly wasmchip8_set_keys: (a: number, b: number) => void;
  readonly wasmchip8_tick_timers: (a: number) => void;
  readonly wasmdisplay_get_height: (a: number) => number;
  readonly wasmdisplay_get_pixel: (a: number, b: number, c: number) => number;
  readonly wasmdisplay_get_width: (a: number) => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
