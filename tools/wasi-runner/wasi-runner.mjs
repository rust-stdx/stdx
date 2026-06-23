#!/usr/bin/env node
import { WASI } from 'node:wasi';
import { readFile } from 'node:fs/promises';
import { argv, exit } from 'node:process';

const wasmPath = argv[2];

const wasi = new WASI({
  args: [wasmPath, ...argv.slice(3)],
  env: { ...process.env },
  preopens: { '/': process.cwd() },
  version: 'preview1',
});

const wasm = await WebAssembly.compile(await readFile(wasmPath));
const instance = await WebAssembly.instantiate(wasm, {
  wasi_snapshot_preview1: wasi.wasiImport,
});

try {
  await wasi.start(instance);
} catch (err) {
  exit(err.code === 'ERR_WASI_EXIT' ? err.info?.exitCode ?? 0 : 1);
}
