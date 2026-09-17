#!/usr/bin/env node
// Discover which crates of this workspace can be tested on WebAssembly and run
// their test suites.
//
// Usage:
//   node tools/wasm_test/wasm_test.js [--target <triple>] [-p <crate>]... [--exclude <crate>]... [--list] [--help]
//
//   --target <triple>  Rust target to test (default: wasm32-wasip1)
//   -p, --package <crate>
//                      Only test the given crate (repeatable, comma-separated
//                      values allowed). Like `cargo test -p`.
//   --exclude <crate>  Skip a crate (repeatable, comma-separated values allowed)
//   --list             Only print the crates that were selected, then exit
//   --help             Print this help
//
// The selection is fully dynamic, there is no hardcoded list of crates:
//   1. Crates that (transitively, dev-dependencies included) depend on tokio
//      are skipped, so async-only crates are never tested on wasm.
//   2. Every remaining crate is probed with `cargo test --no-run` for the
//      target. Crates that fail to build (OS-specific APIs, libc, C code,
//      proc-macros, ...) are skipped.
//   3. The crates that build are tested. A failing test makes the whole run
//      fail and exits with a non-zero status; skipped crates do not.
//
// `--exclude` can be used to force-skip a crate that would otherwise build and
// run, for example a crate with known flaky wasm tests.
//
// Cargo output is streamed to stdout/stderr as it happens. The script exits
// with a non-zero status if `cargo metadata` fails, if the target is missing,
// or if any selected crate has a failing test.

import { spawnSync } from 'node:child_process';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..', '..');

function parseArgs(argv) {
  const options = { target: 'wasm32-wasip1', packages: new Set(), exclude: new Set(), list: false, help: false };
  const addPackages = (value) => {
    for (const name of value.split(',').map((name) => name.trim()).filter(Boolean)) {
      options.packages.add(name);
    }
  };
  const addExcludes = (value) => {
    for (const name of value.split(',').map((name) => name.trim()).filter(Boolean)) {
      options.exclude.add(name);
    }
  };
  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i];
    if (arg === '--target') {
      const value = argv[++i];
      if (!value) throw new Error('--target requires a value');
      options.target = value;
    } else if (arg.startsWith('--target=')) {
      options.target = arg.slice('--target='.length);
    } else if (arg === '-p' || arg === '--package') {
      const value = argv[++i];
      if (!value) throw new Error(`${arg} requires a value`);
      addPackages(value);
    } else if (arg.startsWith('--package=')) {
      addPackages(arg.slice('--package='.length));
    } else if (arg.startsWith('-p=')) {
      addPackages(arg.slice('-p='.length));
    } else if (arg === '--exclude') {
      const value = argv[++i];
      if (!value) throw new Error('--exclude requires a value');
      addExcludes(value);
    } else if (arg.startsWith('--exclude=')) {
      addExcludes(arg.slice('--exclude='.length));
    } else if (arg === '--list') {
      options.list = true;
    } else if (arg === '--help' || arg === '-h') {
      options.help = true;
    } else {
      throw new Error(`unknown argument: ${arg}`);
    }
  }
  return options;
}

function help() {
  process.stdout.write(
    `Usage: node tools/wasm_test/wasm_test.js [--target <triple>] [-p <crate>]... [--exclude <crate>]... [--list] [--help]\n\n` +
      `  --target <triple>  Rust target to test (default: wasm32-wasip1)\n` +
      `  -p, --package <crate>\n` +
      `                     Only test the given crate (repeatable, comma-separated\n` +
      `                     values allowed). Like \`cargo test -p\`.\n` +
      `  --exclude <crate>  Skip a crate (repeatable, comma-separated values allowed)\n` +
      `  --list             Only print the selected crates, then exit\n` +
      `  --help             Print this help\n`,
  );
}

// Runs a command with its output connected to this process' stdout/stderr and
// returns its exit code. Throws only if the command could not be spawned.
function run(command, args) {
  const result = spawnSync(command, args, { cwd: root, stdio: 'inherit' });
  if (result.error) throw result.error;
  return result.status ?? 1;
}

// Runs a command capturing its stdout. Throws if the command exits non-zero.
function capture(command, args) {
  const result = spawnSync(command, args, {
    cwd: root,
    encoding: 'utf8',
    maxBuffer: 256 * 1024 * 1024,
  });
  if (result.error) throw result.error;
  if (result.status !== 0) {
    process.stderr.write(result.stderr || '');
    throw new Error(`${command} ${args.join(' ')} exited with code ${result.status}`);
  }
  return result.stdout;
}

// Returns the names of the workspace members and the set of workspace members
// that transitively depend on tokio.
function analyzeWorkspace() {
  const metadata = JSON.parse(capture('cargo', ['metadata', '--format-version', '1']));

  const nameById = new Map(metadata.packages.map((pkg) => [pkg.id, pkg.name]));
  const nodeById = new Map(metadata.resolve.nodes.map((node) => [node.id, node]));
  const kernelIds = new Set(
    metadata.packages.filter((pkg) => pkg.name === 'tokio').map((pkg) => pkg.id),
  );

  const usesTokio = (id) => {
    if (kernelIds.has(id)) return true;
    const seen = new Set();
    const stack = [id];
    while (stack.length > 0) {
      const current = stack.pop();
      if (seen.has(current)) continue;
      seen.add(current);
      if (kernelIds.has(current)) return true;
      for (const dep of nodeById.get(current)?.deps ?? []) {
        stack.push(dep.pkg);
      }
    }
    return false;
  };

  const members = metadata.workspace_members.map((id) => ({
    id,
    name: nameById.get(id) ?? id,
    procMacroOnly: (metadata.packages.find((pkg) => pkg.id === id)?.targets ?? []).every(
      (target) => target.kind.includes('proc-macro'),
    ),
    tokio: usesTokio(id),
  }));

  members.sort((a, b) => a.name.localeCompare(b.name));
  return members;
}

function main() {
  const options = parseArgs(process.argv.slice(2));
  if (options.help) {
    help();
    return;
  }

  const members = analyzeWorkspace();

  const unknownPackages = [...options.packages].filter(
    (name) => !members.some((member) => member.name === name),
  );
  if (unknownPackages.length > 0) {
    throw new Error(`unknown crate(s) passed to --package: ${unknownPackages.join(', ')}`);
  }

  const unknownExcludes = [...options.exclude].filter(
    (name) => !members.some((member) => member.name === name),
  );
  if (unknownExcludes.length > 0) {
    throw new Error(`unknown crate(s) passed to --exclude: ${unknownExcludes.join(', ')}`);
  }

  const selected = (member) => options.packages.size === 0 || options.packages.has(member.name);

  const skippedTokio = members.filter((member) => selected(member) && member.tokio);
  const skippedProcMacro = members.filter(
    (member) => selected(member) && !member.tokio && member.procMacroOnly,
  );
  const skippedExcluded = members.filter(
    (member) => selected(member) && !member.tokio && !member.procMacroOnly && options.exclude.has(member.name),
  );
  const candidates = members.filter(
    (member) =>
      selected(member) && !member.tokio && !member.procMacroOnly && !options.exclude.has(member.name),
  );

  if (options.list) {
    for (const member of candidates) process.stdout.write(`${member.name}\n`);
    return;
  }

  process.stdout.write(`Target: ${options.target}\n`);
  process.stdout.write(`Workspace members: ${members.length}\n`);
  process.stdout.write(
    `Skipping ${skippedTokio.length} tokio-dependent crate(s): ` +
      `${skippedTokio.map((m) => m.name).join(', ') || 'none'}\n`,
  );
  process.stdout.write(
    `Skipping ${skippedProcMacro.length} proc-macro crate(s): ` +
      `${skippedProcMacro.map((m) => m.name).join(', ') || 'none'}\n`,
  );
  if (skippedExcluded.length > 0) {
    process.stdout.write(`Skipping ${skippedExcluded.length} excluded crate(s): ${skippedExcluded.map((m) => m.name).join(', ')}\n`);
  }
  process.stdout.write(`Probing ${candidates.length} crate(s) for ${options.target}...\n\n`);

  const buildable = [];
  const skippedBuild = [];
  for (const member of candidates) {
    process.stdout.write(`====> probe ${member.name}\n`);
    const code = run('cargo', ['test', '--no-run', '--target', options.target, '-p', member.name]);
    if (code === 0) {
      buildable.push(member);
    } else {
      skippedBuild.push(member);
      process.stdout.write(`<==== ${member.name} is not compatible with ${options.target}, skipping\n\n`);
    }
  }

  const failed = [];
  for (const member of buildable) {
    process.stdout.write(`====> test ${member.name}\n`);
    const code = run('cargo', ['test', '--target', options.target, '-p', member.name]);
    if (code !== 0) failed.push(member);
    process.stdout.write('\n');
  }

  process.stdout.write('Summary\n');
  process.stdout.write(`  tested:  ${buildable.length - failed.length} passed, ${failed.length} failed\n`);
  process.stdout.write(`  skipped: ${skippedTokio.length} tokio, ${skippedProcMacro.length} proc-macro, ${skippedExcluded.length} excluded, ${skippedBuild.length} not ${options.target}-compatible\n`);
  if (failed.length > 0) {
    process.stdout.write(`  failed:  ${failed.map((m) => m.name).join(', ')}\n`);
    process.exitCode = 1;
  }
  if (skippedBuild.length > 0) {
    process.stdout.write(`  not ${options.target}-compatible: ${skippedBuild.map((m) => m.name).join(', ')}\n`);
  }
}

try {
  main();
} catch (error) {
  process.stderr.write(`error: ${error.message}\n`);
  process.exitCode = 1;
}
