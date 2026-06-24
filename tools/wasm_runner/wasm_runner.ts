#!/usr/bin/env node
import { execFileSync, spawn } from 'node:child_process';
import { argv, exit } from 'node:process';

// usage: ./wasm_runner.ts my_wasm_module.wasm

function wastimeAvailable() {
  try {
    execFileSync('which', ['wastime'], { stdio: 'ignore' });
    return true;
  } catch {
    return false;
  }
}

async function runWithWastime(wasmPath: string) {
  return new Promise(() => {
    const child = spawn('wastime', ['run', wasmPath, ...argv.slice(3)], {
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

if (wastimeAvailable()) {
  console.log('WASM runtime: Wasmtime');
  await runWithWastime(wasmPath);
} else {
  console.log('WASM runtime: Node.js');
  await runWithNode(wasmPath);
}
