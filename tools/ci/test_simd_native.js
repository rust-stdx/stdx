#!/usr/bin/env node
// Workspace-wide SIMD test runner.
//
// Detects the host CPU features, runs the full workspace test suite with the
// strongest SIMD feature set the CPU can execute, and compile-checks the wider
// feature set when the CPU cannot execute it.
//
// Usage:
//   node tools/ci/test_simd_native.js [--dry-run] [--check-only] [--help]
//
//   --dry-run     Print the commands without running them.
//   --check-only  Compile-check the selected feature set instead of testing.
//   --help        Print this help.

import { spawnSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { arch, platform } from 'node:process';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..', '..');

function help() {
  process.stdout.write(
    'Usage: node tools/ci/test_simd_native.js [--dry-run] [--check-only] [--help]\n\n' +
      '  --dry-run     Print the commands without running them.\n' +
      '  --check-only  Compile-check the selected feature set instead of testing.\n' +
      '  --help        Print this help.\n',
  );
}

function parseArgs(argv) {
  const options = {
    dryRun: false,
    checkOnly: false,
    help: false,
  };
  for (const arg of argv) {
    if (arg === '--dry-run') options.dryRun = true;
    else if (arg === '--check-only') options.checkOnly = true;
    else if (arg === '--help' || arg === '-h') options.help = true;
    else throw new Error(`unknown argument: ${arg}`);
  }
  return options;
}

function cpuInfo() {
  try {
    return readFileSync('/proc/cpuinfo', 'utf8');
  } catch {
    return '';
  }
}

function cpuFeatures() {
  const text = cpuInfo();
  const features = new Set();
  for (const line of text.split('\n')) {
    const match = line.match(/^(?:flags|Features)\s*:\s*(.*)$/i);
    if (!match) continue;
    for (const feature of match[1].trim().split(/\s+/)) {
      features.add(feature.toLowerCase());
    }
  }
  return features;
}

function featureFlags(names) {
  if (names.length === 0) return '';
  return `-C target-feature=${names.map((name) => `+${name}`).join(',')}`;
}

function run(command, args, rustflags, dryRun) {
  const env = { ...process.env };
  if (rustflags) {
    env.RUSTFLAGS = env.RUSTFLAGS ? `${env.RUSTFLAGS} ${rustflags}` : rustflags;
  }
  process.stdout.write(`\n$ ${command} ${args.join(' ')}\n`);
  if (rustflags) process.stdout.write(`  RUSTFLAGS=${env.RUSTFLAGS}\n`);
  if (dryRun) return 0;
  const result = spawnSync(command, args, { cwd: root, stdio: 'inherit', env });
  if (result.error) throw result.error;
  return result.status ?? 1;
}

function cargo(args, rustflags, dryRun) {
  return run('cargo', args, rustflags, dryRun);
}

function planX86(features) {
  const baseline = [];
  for (const [cpu, rust] of [
    ['avx', 'avx'],
    ['avx2', 'avx2'],
    ['pclmulqdq', 'pclmulqdq'],
    ['sse4_1', 'sse4.1'],
    ['ssse3', 'ssse3'],
    ['aes', 'aes'],
  ]) {
    if (features.has(cpu)) baseline.push(rust);
  }

  const max = [...baseline];
  for (const [cpu, rust] of [
    ['avx512f', 'avx512f'],
    ['avx512bw', 'avx512bw'],
    ['vaes', 'vaes'],
    ['vpclmulqdq', 'vpclmulqdq'],
    ['sha_ni', 'sha'],
    ['sha512', 'sha512'],
  ]) {
    if (features.has(cpu)) max.push(rust);
  }

  const canRunAvx2 = features.has('avx2');
  const runtime = canRunAvx2 ? max : [];
  const compile = max.length > 0 && runtime.length !== max.length ? max : [];
  return { runtime, compile };
}

function planAarch64(features) {
  const runtime = [];
  for (const [cpu, rust] of [
    ['aes', 'aes'],
    ['sha2', 'sha2'],
    ['sha3', 'sha3'],
    ['crc32', 'crc'],
  ]) {
    if (features.has(cpu)) runtime.push(rust);
  }
  // Compile-check the full optional set even when the host cannot execute it.
  const compile = ['aes', 'sha2', 'sha3', 'crc'];
  return { runtime, compile };
}

function main() {
  const options = parseArgs(process.argv.slice(2));
  if (options.help) {
    help();
    return;
  }

  const features = cpuFeatures();
  let plan;
  if (arch === 'x64' || arch === 'ia32') {
    plan = planX86(features);
  } else if (arch === 'arm64') {
    plan = planAarch64(features);
  } else {
    process.stdout.write(`No SIMD feature plan for ${arch}/${platform}; running baseline workspace tests.\n`);
    plan = { runtime: [], compile: [] };
  }

  process.stdout.write(`Host architecture: ${arch}\n`);
  process.stdout.write(
    `Runtime SIMD features: ${plan.runtime.length > 0 ? plan.runtime.join(', ') : '(none)'}\n`,
  );
  process.stdout.write(
    `Compile-only SIMD features: ${plan.compile.length > 0 ? plan.compile.join(', ') : '(none)'}\n`,
  );

  if (plan.runtime.length === 0 && plan.compile.length === 0) {
    process.stdout.write('No SIMD features detected; skipping.\n');
    return;
  }

  let failed = false;

  if (plan.runtime.length > 0) {
    const args = options.checkOnly
      ? ['check', '--workspace', '--all-features']
      : ['test', '--workspace', '--all-features'];
    const code = cargo(args, featureFlags(plan.runtime), options.dryRun);
    if (code !== 0) failed = true;
  }

  if (plan.compile.length > 0 && plan.compile.join(',') !== plan.runtime.join(',')) {
    const code = cargo(['check', '--workspace', '--all-features'], featureFlags(plan.compile), options.dryRun);
    if (code !== 0) failed = true;
  }

  if (failed) process.exitCode = 1;
}

try {
  main();
} catch (error) {
  process.stderr.write(`error: ${error.message}\n`);
  process.exitCode = 1;
}
