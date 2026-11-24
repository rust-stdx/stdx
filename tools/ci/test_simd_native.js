#!/usr/bin/env node
// Workspace-wide SIMD test runner.
//
// Detects the packages that contain SIMD kernels, detects the host CPU
// features, runs those packages with the strongest SIMD feature set the CPU can
// execute, and compile-checks the wider feature set when the CPU cannot execute
// it.
//
// Usage:
//   node tools/ci/test_simd_native.js [--dry-run] [--check-only] [--list]
//       [--target <triple>] [--exclude <crate>]... [--help]
//
//   --dry-run          Print the commands without running them.
//   --check-only       Compile-check the selected feature set instead of testing.
//   --list             Print the detected SIMD package specifications, one per
//                      line, and exit.
//   --target <triple>  Cargo target triple (passed to every cargo invocation).
//   --exclude <crate>  Skip a detected package (repeatable or comma-separated).
//   --help             Print this help.
//
// A package is considered a SIMD package when any `.rs` file under its manifest
// directory contains `#[target_feature(...)]`, `is_x86_feature_detected!` /
// `is_aarch64_feature_detected!`, `core::arch` / `std::arch`, or
// `cfg(target_feature)`. The scan covers tests as well as library sources.

import { spawnSync } from 'node:child_process';
import { readFileSync, readdirSync } from 'node:fs';
import { arch, platform } from 'node:process';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..', '..');

// Matches the ways a crate can opt into CPU-specific code.
const SIMD_RE =
  /#\[target_feature\s*\(|is_(?:x86|aarch64)_feature_detected!\s*\(|(?:core|std)::arch::|cfg\s*\(\s*target_feature/;

function help() {
  process.stdout.write(
    'Usage: node tools/ci/test_simd_native.js [--dry-run] [--check-only] [--list]\n' +
      '       [--target <triple>] [--exclude <crate>]... [--help]\n\n' +
      '  --dry-run          Print the commands without running them.\n' +
      '  --check-only       Compile-check the selected feature set instead of testing.\n' +
      '  --list             Print the detected SIMD packages (cargo package specs), one per line, and exit.\n' +
      '  --target <triple>  Cargo target triple (passed to every cargo invocation).\n' +
      '  --exclude <crate>  Skip a detected package (repeatable or comma-separated).\n' +
      '  --help             Print this help.\n',
  );
}

function parseArgs(argv) {
  const options = {
    dryRun: false,
    checkOnly: false,
    list: false,
    help: false,
    target: null,
    exclude: new Set(),
  };
  const addExcludes = (value) => {
    for (const name of value.split(',').map((name) => name.trim()).filter(Boolean)) {
      options.exclude.add(name);
    }
  };
  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i];
    if (arg === '--dry-run') options.dryRun = true;
    else if (arg === '--check-only') options.checkOnly = true;
    else if (arg === '--list') options.list = true;
    else if (arg === '--help' || arg === '-h') options.help = true;
    else if (arg === '--target') {
      const value = argv[++i];
      if (!value) throw new Error('--target requires a value');
      options.target = value;
    } else if (arg.startsWith('--target=')) {
      options.target = arg.slice('--target='.length);
      if (!options.target) throw new Error('--target requires a value');
    } else if (arg === '--exclude') {
      const value = argv[++i];
      if (!value) throw new Error('--exclude requires a value');
      addExcludes(value);
    } else if (arg.startsWith('--exclude=')) {
      addExcludes(arg.slice('--exclude='.length));
    } else {
      throw new Error(`unknown argument: ${arg}`);
    }
  }
  return options;
}

function workspacePackages() {
  const result = spawnSync('cargo', ['metadata', '--locked', '--no-deps', '--format-version', '1'], {
    cwd: root,
    encoding: 'utf8',
  });
  if (result.error) throw result.error;
  if (result.status !== 0) {
    throw new Error((result.stderr || 'cargo metadata failed').trim());
  }
  return JSON.parse(result.stdout).packages;
}

function rustFiles(dir, out) {
  let entries;
  try {
    entries = readdirSync(dir, { withFileTypes: true });
  } catch {
    return;
  }
  for (const entry of entries) {
    if (entry.name === 'target' || entry.name === '.git') continue;
    const path = join(dir, entry.name);
    if (entry.isDirectory()) {
      rustFiles(path, out);
    } else if (entry.isFile() && entry.name.endsWith('.rs')) {
      out.push(path);
    }
  }
}

function detectSimdPackages(exclude) {
  const packages = [];
  for (const pkg of workspacePackages()) {
    if (exclude.has(pkg.name)) continue;
    const files = [];
    rustFiles(dirname(pkg.manifest_path), files);
    if (files.some((file) => SIMD_RE.test(readFileSync(file, 'utf8')))) {
      // Prefer the package's path URL over its name: a workspace crate can share
      // a name and version with a registry crate (for example `zmij`), which
      // makes `-p zmij` and `-p zmij@1.0.23` ambiguous. A `file://` URL is also
      // safe to pass through a shell, unlike a full package ID containing `#`.
      const spec = pkg.id.startsWith('path+')
        ? pkg.id.slice('path+'.length).split('#')[0]
        : pkg.name;
      packages.push({ name: pkg.name, spec });
    }
  }
  packages.sort((a, b) => a.name.localeCompare(b.name));
  return packages;
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
  // Verify the committed lockfile instead of silently updating it.
  return run('cargo', [args[0], '--locked', ...args.slice(1)], rustflags, dryRun);
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

  const packages = detectSimdPackages(options.exclude);
  if (options.list) {
    for (const pkg of packages) {
      process.stdout.write(`${pkg.spec}\n`);
    }
    return;
  }

  const names = packages.map((pkg) => pkg.name);
  process.stdout.write(`SIMD packages: ${names.length > 0 ? names.join(', ') : '(none)'}\n`);
  if (packages.length === 0) {
    process.stdout.write('No SIMD packages detected; skipping.\n');
    return;
  }

  const features = cpuFeatures();
  let plan;
  if (arch === 'x64' || arch === 'ia32') {
    plan = planX86(features);
  } else if (arch === 'arm64') {
    plan = planAarch64(features);
  } else {
    process.stdout.write(`No SIMD feature plan for ${arch}/${platform}; skipping SIMD runs.\n`);
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

  const targetArgs = options.target ? ['--target', options.target] : [];
  const packageArgs = packages.flatMap((pkg) => ['-p', pkg.spec]);
  let failed = false;

  if (plan.runtime.length > 0) {
    const args = [
      options.checkOnly ? 'check' : 'test',
      ...targetArgs,
      ...packageArgs,
      '--all-features',
    ];
    const code = cargo(args, featureFlags(plan.runtime), options.dryRun);
    if (code !== 0) failed = true;
  }

  if (plan.compile.length > 0 && plan.compile.join(',') !== plan.runtime.join(',')) {
    const args = ['check', ...targetArgs, ...packageArgs, '--all-features'];
    const code = cargo(args, featureFlags(plan.compile), options.dryRun);
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
