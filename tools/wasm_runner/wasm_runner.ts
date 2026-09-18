#!/usr/bin/env node
import { execFileSync, spawn, spawnSync } from 'node:child_process';
import { argv, exit } from 'node:process';

// A wrapper script to run WASM modules with either Wasmtime or Node.js
// usage: ./tools/wasm_runner.ts my_wasm_module.wasm

// V8's baseline compiler (Liftoff) produces very slow code for some 64-bit
// integer workloads (e.g. SHA-512), and wasm functions are not reliably
// promoted to the optimizing tier (TurboFan) during a benchmark run. Pinning
// both tiers makes Node.js measure consistently optimized code.
// const NODE_V8_FLAGS = ['--no-liftoff', '--no-wasm-tier-up'];
const NODE_V8_FLAGS = ['--liftoff', '--no-wasm-tier-up', '--no-capture'];

function wasmtimeAvailable() {
  try {
    execFileSync('which', ['wasmtime'], { stdio: 'ignore' });
    return true;
  } catch {
    return false;
  }
}

// Re-exec Node with `NODE_V8_FLAGS` if they are not already active. V8 only
// reads these flags at startup, so they cannot be set from within the current
// process. Does nothing (and does not recurse) once the flags are present.
function ensureNodeOptimizingTier() {
  if (NODE_V8_FLAGS.every((flag) => process.execArgv.includes(flag))) {
    return;
  }

  const child = spawnSync(process.execPath, [...process.execArgv, ...NODE_V8_FLAGS, argv[1], ...argv.slice(2)], {
    stdio: 'inherit',
  });
  exit(child.status ?? 1);
}

async function runWithWastime(wasmPath: string) {
  return new Promise(() => {
    const child = spawn('wasmtime', ['run', wasmPath, ...argv.slice(3)], {
      stdio: 'inherit',
    });
    child.on('exit', (code: number) => exit(code ?? 0));
  });
}

async function runWithNode(wasmPath: string) {
  const { WASI } = await import('node:wasi');
  const { readFile } = await import('node:fs/promises');

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
  } catch (err: any) {
    exit(err.code === 'ERR_WASI_EXIT' ? err.info?.exitCode ?? 0 : 1);
  }
}

const wasmPath = argv[2];

if (wasmtimeAvailable()) {
  console.log('WASM runtime: Wasmtime');
  await runWithWastime(wasmPath);
} else {
  ensureNodeOptimizingTier();
  console.log(`WASM runtime: Node.js ${process.version}`);
  await runWithNode(wasmPath);
}
