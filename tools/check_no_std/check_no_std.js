#!/usr/bin/env node
// Discover which crates of this workspace support `no_std` and check them for
// the bare-metal Tier 1 targets `riscv32imac-unknown-none-elf` and
// `thumbv7em-none-eabi`.
//
// Usage:
//   node tools/check_no_std/check_no_std.js [--target <triple>]... [-p <crate>]... \
//        [--exclude <crate>]... [--list | --list-detailed] [--help]
//
//   --target <triple>  Rust target to check (repeatable; default: both
//                      riscv32imac-unknown-none-elf and thumbv7em-none-eabi)
//   -p, --package <crate>
//                      Only check the given crate (repeatable, comma-separated
//                      values allowed). Like `cargo check -p`.
//   --exclude <crate>  Skip a crate (repeatable, comma-separated values allowed)
//   --list             Only print the crates that are no_std-capable, then exit
//   --list-detailed    Like --list, but also print the feature configurations
//                      that compile for each crate
//   --help             Print this help
//
// A crate is a candidate when its library declares `#![no_std]` (directly or
// through `#![cfg_attr(..., no_std)]`). A crate that does not declare `no_std`
// cannot build for a target without `std`, so it is never a candidate.
//
// For every candidate the script derives the set of `std`-free feature
// configurations from `cargo metadata`:
//   * the `default` feature is ignored, and
//   * any feature that transitively enables `std` (its own or a dependency's)
//     is dropped (for example `timezone-system = ["std", ...]`).
//
// Each configuration is checked with
// `cargo check --lib --no-default-features [--features ...] --target <triple>`.
// `--lib` is used on purpose: binaries such as `unsafe_libyaml`'s test-suite
// runners are allowed to use `std` and must not fail the check.
//
// A crate is no_std-capable on a target when at least one configuration
// compiles. The script exits with a non-zero status when a crate that declares
// `no_std` does not compile under any configuration on any requested target.
// Configurations that fail even though the crate is capable (for example a
// `random` feature backed by `getrandom`, which has no bare-metal backend) are
// reported but are not fatal.
//
// Cargo output is captured; failures are shown in the summary.

import { spawnSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..', '..');
const DEFAULT_TARGETS = ['riscv32imac-unknown-none-elf', 'thumbv7em-none-eabi'];

function parseArgs(argv) {
  const options = {
    targets: [],
    packages: new Set(),
    exclude: new Set(),
    list: false,
    listDetailed: false,
    help: false,
  };
  const addSet = (set) => (value) => {
    for (const name of value.split(',').map((name) => name.trim()).filter(Boolean)) {
      set.add(name);
    }
  };
  const addPackages = addSet(options.packages);
  const addExcludes = addSet(options.exclude);
  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i];
    if (arg === '--target') {
      const value = argv[++i];
      if (!value) throw new Error('--target requires a value');
      options.targets.push(value);
    } else if (arg.startsWith('--target=')) {
      options.targets.push(arg.slice('--target='.length));
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
    } else if (arg === '--list-detailed') {
      options.listDetailed = true;
    } else if (arg === '--help' || arg === '-h') {
      options.help = true;
    } else {
      throw new Error(`unknown argument: ${arg}`);
    }
  }
  if (options.targets.length === 0) options.targets = [...DEFAULT_TARGETS];
  if (options.list && options.listDetailed) {
    throw new Error('--list and --list-detailed are mutually exclusive');
  }
  return options;
}

function help() {
  process.stdout.write(
    `Usage: node tools/check_no_std/check_no_std.js [--target <triple>]... [-p <crate>]... [--exclude <crate>]... [--list | --list-detailed] [--help]\n\n` +
      `  --target <triple>  Rust target to check (repeatable; default: both\n` +
      `                     riscv32imac-unknown-none-elf and thumbv7em-none-eabi)\n` +
      `  -p, --package <crate>\n` +
      `                     Only check the given crate (repeatable, comma-separated\n` +
      `                     values allowed). Like \`cargo check -p\`.\n` +
      `  --exclude <crate>  Skip a crate (repeatable, comma-separated values allowed)\n` +
      `  --list             Only print the crates that are no_std-capable, then exit\n` +
      `  --list-detailed    Like --list, but also print the compiling configurations\n` +
      `  --help             Print this help\n`,
  );
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

// Returns true when the crate root declares `#![no_std]`, either directly or
// through `#![cfg_attr(..., no_std)]`.
function declaresNoStd(srcPath) {
  let text;
  try {
    text = readFileSync(srcPath, 'utf8');
  } catch {
    return false;
  }
  return /#!\[[^\]]*\bno_std\b[^\]]*\]/.test(text);
}

// Returns the set of features of `features` (a `cargo metadata` feature map)
// that transitively enable `std`, either their own or a dependency's.
function stdPullingFeatures(features) {
  const pulling = new Set();
  for (const [name, items] of Object.entries(features)) {
    if (name === 'std' || items.some((item) => item === 'std' || item.endsWith('/std'))) {
      pulling.add(name);
    }
  }
  let changed = true;
  while (changed) {
    changed = false;
    for (const [name, items] of Object.entries(features)) {
      if (pulling.has(name)) continue;
      const enablesStd = items.some(
        (item) => !item.startsWith('dep:') && !item.includes('/') && pulling.has(item),
      );
      if (enablesStd) {
        pulling.add(name);
        changed = true;
      }
    }
  }
  return pulling;
}

// Returns the `std`-free feature configurations to check for a crate, as arrays
// of feature names. The empty array is the core-only configuration.
function noStdConfigs(features) {
  const pulling = stdPullingFeatures(features);
  const stdFree = Object.keys(features)
    .filter((name) => name !== 'default' && !pulling.has(name))
    .sort();
  const hasAlloc = stdFree.includes('alloc');

  const configs = [[]];
  if (hasAlloc) configs.push(['alloc']);
  for (const feature of stdFree) {
    if (feature === 'alloc') continue;
    configs.push([feature]);
    if (hasAlloc) configs.push(['alloc', feature]);
  }

  const seen = new Set();
  return configs.filter((config) => {
    const key = config.join(',');
    if (seen.has(key)) return false;
    seen.add(key);
    return true;
  });
}

function configLabel(config) {
  return config.length === 0 ? 'core' : config.join(',');
}

function analyzeWorkspace() {
  const metadata = JSON.parse(capture('cargo', ['metadata', '--format-version', '1']));

  const packageById = new Map(metadata.packages.map((pkg) => [pkg.id, pkg]));
  const nodeById = new Map(metadata.resolve.nodes.map((node) => [node.id, node]));
  const tokioIds = new Set(
    metadata.packages.filter((pkg) => pkg.name === 'tokio').map((pkg) => pkg.id),
  );

  const usesTokio = (id) => {
    if (tokioIds.has(id)) return true;
    const seen = new Set();
    const stack = [id];
    while (stack.length > 0) {
      const current = stack.pop();
      if (seen.has(current)) continue;
      seen.add(current);
      if (tokioIds.has(current)) return true;
      for (const dep of nodeById.get(current)?.deps ?? []) {
        stack.push(dep.pkg);
      }
    }
    return false;
  };

  const members = [];
  const skipped = { tokio: [], procMacro: [], noLib: [] };
  for (const id of metadata.workspace_members) {
    const pkg = packageById.get(id);
    if (!pkg) continue;
    const targets = pkg.targets ?? [];
    const lib = targets.find((target) => target.kind.includes('lib') || target.kind.includes('rlib'));
    if (!lib) {
      skipped.noLib.push(pkg.name);
      continue;
    }
    if (targets.length > 0 && targets.every((target) => target.kind.includes('proc-macro'))) {
      skipped.procMacro.push(pkg.name);
      continue;
    }
    if (usesTokio(id)) {
      skipped.tokio.push(pkg.name);
      continue;
    }
    members.push({
      id,
      name: pkg.name,
      declaredNoStd: declaresNoStd(lib.src_path),
      features: pkg.features ?? {},
    });
  }
  members.sort((a, b) => a.name.localeCompare(b.name));
  for (const names of Object.values(skipped)) names.sort();
  return { members, skipped };
}

// Best-effort list of installed rustup targets. Returns null when rustup is not
// available, in which case the target check is skipped.
function installedTargets() {
  const result = spawnSync('rustup', ['target', 'list', '--installed'], {
    cwd: root,
    encoding: 'utf8',
  });
  if (result.error || result.status !== 0) return null;
  return new Set(result.stdout.split('\n').map((line) => line.trim()).filter(Boolean));
}

// Checks one crate/configuration/target. Returns whether it compiled and the
// captured compiler output (used to explain gate failures).
function checkConfig(target, name, config) {
  const args = ['check', '--quiet', '--lib', '--no-default-features'];
  if (config.length > 0) args.push('--features', config.join(','));
  args.push('--target', target, '-p', name);
  const result = spawnSync('cargo', args, {
    cwd: root,
    encoding: 'utf8',
    maxBuffer: 64 * 1024 * 1024,
  });
  if (result.error) throw result.error;
  return { ok: result.status === 0, output: `${result.stderr || ''}${result.stdout || ''}` };
}

// Extracts the compiler error headers (and their location line) from captured
// cargo output, so gate failures are readable instead of a wall of warnings.
function errorLines(output) {
  const lines = output.split('\n');
  const picked = [];
  for (let i = 0; i < lines.length; i++) {
    if (!/^\s*error(\[|:)/.test(lines[i])) continue;
    picked.push(lines[i].trimEnd());
    const next = lines[i + 1];
    if (next && /-->/.test(next)) picked.push(next.trimEnd());
  }
  return picked.slice(0, 40);
}

function main() {
  const options = parseArgs(process.argv.slice(2));
  if (options.help) {
    help();
    return;
  }

  const { members, skipped } = analyzeWorkspace();

  const known = new Set(members.map((member) => member.name));
  const unknownPackages = [...options.packages].filter((name) => !known.has(name));
  if (unknownPackages.length > 0) {
    throw new Error(`unknown crate(s) passed to --package: ${unknownPackages.join(', ')}`);
  }
  const unknownExcludes = [...options.exclude].filter((name) => !known.has(name));
  if (unknownExcludes.length > 0) {
    throw new Error(`unknown crate(s) passed to --exclude: ${unknownExcludes.join(', ')}`);
  }

  const installed = installedTargets();
  if (installed) {
    const missing = options.targets.filter((target) => !installed.has(target));
    if (missing.length > 0) {
      throw new Error(
        `target(s) not installed: ${missing.join(', ')} (run \`rustup target add ${missing.join(' ')}\`)`,
      );
    }
  }

  const selected = (member) =>
    (options.packages.size === 0 || options.packages.has(member.name)) &&
    !options.exclude.has(member.name);
  const candidates = members.filter((member) => member.declaredNoStd && selected(member));

  // name -> target -> [{ label, ok, output }]
  const results = new Map();
  for (const member of candidates) {
    results.set(member.name, new Map());
    for (const target of options.targets) {
      const configs = noStdConfigs(member.features);
      const rows = [];
      for (const config of configs) {
        const { ok, output } = checkConfig(target, member.name, config);
        rows.push({ label: configLabel(config), ok, output });
      }
      results.get(member.name).set(target, rows);
    }
  }

  const capableOn = (name, target) => results.get(name).get(target).some((row) => row.ok);
  const capableEverywhere = (name) => options.targets.every((target) => capableOn(name, target));

  if (options.list || options.listDetailed) {
    for (const member of candidates) {
      if (!capableEverywhere(member.name)) continue;
      if (options.list) {
        process.stdout.write(`${member.name}\n`);
        continue;
      }
      const first = results.get(member.name).get(options.targets[0]);
      const common = first
        .filter((row) => row.ok)
        .map((row) => row.label)
        .filter((label) =>
          options.targets.every((target) =>
            results.get(member.name).get(target).some((row) => row.ok && row.label === label),
          ),
        );
      process.stdout.write(`${member.name} (${common.join(', ')})\n`);
    }
    return;
  }

  process.stdout.write(`Targets: ${options.targets.join(', ')}\n`);
  process.stdout.write(`Workspace members: ${members.length}\n`);
  process.stdout.write(
    `Skipping ${skipped.tokio.length} tokio-dependent crate(s), ` +
      `${skipped.procMacro.length} proc-macro crate(s), ` +
      `${skipped.noLib.length} crate(s) without a library target\n`,
  );
  process.stdout.write(`Checking ${candidates.length} no_std crate(s)...\n\n`);

  const broken = [];
  for (const target of options.targets) {
    process.stdout.write(`### ${target}\n`);
    for (const member of candidates) {
      const rows = results.get(member.name).get(target);
      const okLabels = rows.filter((row) => row.ok).map((row) => row.label);
      if (okLabels.length > 0) {
        process.stdout.write(`  OK   ${member.name} (${okLabels.join(', ')})\n`);
      } else {
        process.stdout.write(`  FAIL ${member.name}\n`);
        broken.push({ name: member.name, target, rows });
      }
    }
    process.stdout.write('\n');
  }

  const capableCount = candidates.filter((member) => capableEverywhere(member.name)).length;
  process.stdout.write('Summary\n');
  process.stdout.write(`  no_std-capable: ${capableCount}/${candidates.length}\n`);
  if (broken.length > 0) {
    process.stdout.write(`  declared no_std but did not compile:\n`);
    for (const entry of broken) {
      process.stdout.write(`    ${entry.name} (${entry.target})\n`);
    }
    const first = broken[0];
    const errors = errorLines(first.rows[0].output);
    process.stdout.write(`\n${first.name} (${first.target}) errors:\n`);
    process.stdout.write(`${errors.length > 0 ? errors.join('\n') : first.rows[0].output.trimEnd()}\n`);
    process.exitCode = 1;
  }
}

try {
  main();
} catch (error) {
  process.stderr.write(`error: ${error.message}\n`);
  process.exitCode = 1;
}
