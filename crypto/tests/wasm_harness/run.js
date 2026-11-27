// Minimal wasm32 runtime harness. Build first, e.g.:
//
//   cd crypto/tests/wasm_harness
//   RUSTFLAGS="-C target-feature=+simd128" \
//     CARGO_TARGET_DIR=/tmp/stdx-target-chacha-wasm \
//     cargo build --target wasm32-unknown-unknown --release
//   CARGO_TARGET_DIR=/tmp/stdx-target-chacha-wasm node run.js
//
// `run()` returns a bitmask of failures (0 == PASS); see src/lib.rs.
const fs = require('fs');
const path = require('path');

const targetDir = process.env.CARGO_TARGET_DIR || path.join(__dirname, 'target');
const wasmPath =
  process.env.WASM ||
  path.join(targetDir, 'wasm32-unknown-unknown', 'release', 'wasm_harness.wasm');

const bytes = fs.readFileSync(wasmPath);
WebAssembly.instantiate(bytes, {}).then(({ instance }) => {
  console.log('exports', Object.keys(instance.exports));
  const r = instance.exports.run();
  console.log('run() =', r, r === 0 ? 'PASS' : 'FAIL');
  process.exit(r === 0 ? 0 : 1);
}).catch((e) => {
  console.error('ERR', e);
  process.exit(2);
});
