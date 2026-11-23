import { readFileSync, readdirSync, existsSync, writeFileSync } from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const docDir = join(__dirname, "..", "..", "target", "doc");
const staticDir = join(docDir, "static.files");

const cratesJsPath = join(docDir, "crates.js");
if (!existsSync(cratesJsPath)) {
  console.error("target/doc/crates.js not found. Run 'cargo doc --no-deps' first.");
  process.exit(1);
}

const cratesJs = readFileSync(cratesJsPath, "utf-8");
const match = cratesJs.match(/window\.ALL_CRATES\s*=\s*(\[[\s\S]*?\]);/);
if (!match) {
  throw new Error("Could not parse crates.js");
}

// Rustdoc <= 1.101 emits `["crate", ...]`; newer nightlies emit objects with
// the crate name under `c` (and an optional sidebar heading under `h`).
const crateNames = JSON.parse(match[1])
  .map((entry) => {
    if (typeof entry === "string") return entry;
    if (entry && typeof entry.c === "string") return entry.c;
    return null;
  })
  .filter((name) => name !== null);

const staticFiles = readdirSync(staticDir);

function findFile(prefix, suffix) {
  const f = staticFiles.find(f => f.startsWith(prefix) && f.endsWith(suffix));
  if (!f) throw new Error(`Static file not found: ${prefix}*${suffix}`);
  return f;
}

function findFont(name) {
  const f = staticFiles.find(f => f.startsWith(name));
  if (!f) throw new Error(`Font not found: ${name}*`);
  return f;
}

// Extract rustdoc version info from existing index (if available) for the meta tag
let rustdocVersion = "unknown";
let channel = "unknown";
const existingIndex = join(docDir, "index.html");
if (existsSync(existingIndex)) {
  const html = readFileSync(existingIndex, "utf-8");
  rustdocVersion = html.match(/data-rustdoc-version="([^"]*)"/)?.[1] ?? rustdocVersion;
  channel = html.match(/data-channel="([^"]*)"/)?.[1] ?? channel;
}

const normCss = findFile("normalize-", ".css");
const rdocCss = findFile("rustdoc-", ".css");
const mainJs = findFile("main-", ".js");
const storageJs = findFile("storage-", ".js");
const noscriptCss = findFile("noscript-", ".css");
const faviconPng = findFile("favicon-32x32-", ".png");
const faviconSvg = findFile("favicon-", ".svg");
const searchJs = findFile("search-", ".js");
const stringdexJs = findFile("stringdex-", ".js");
const settingsJs = findFile("settings-", ".js");

const fonts = [
  "SourceSerif4-Regular-",
  "FiraSans-Italic-",
  "FiraSans-Regular-",
  "FiraSans-MediumItalic-",
  "FiraSans-Medium-",
  "SourceCodePro-Regular-",
  "SourceCodePro-Semibold-",
].map(findFont);

function readCrateMeta(name) {
  const indexPath = join(docDir, name, "index.html");
  if (!existsSync(indexPath)) return null;

  const html = readFileSync(indexPath, "utf-8");
  const version = html.match(/<span class="version">([^<]+)<\/span>/)?.[1] ?? "";
  const description = html.match(/<meta name="description" content="([^"]*)">/)?.[1] ?? "";
  return { name, version, description };
}

const crates = crateNames
  .map(readCrateMeta)
  .filter(Boolean)
  .sort((a, b) => a.name.localeCompare(b.name));

const rows = crates.map(c => `      <dt id="crate-${c.name}"><a class="mod" href="./${c.name}/index.html">${c.name.replace(/_/g, "_<wbr>")}</a></dt><dd>${c.version ? `<span class="version">${c.version}</span> — ` : ""}${escapeHtml(c.description)}</dd>`).join("\n");

function escapeHtml(s) {
  return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");
}

const indexHtml = `<!DOCTYPE html><html lang="en"><head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<meta name="generator" content="rustdoc">
<title>stdx — Workspace documentation</title>
<script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","${fonts.join(",")}".split(",").map(f=>\`<link rel="preload" as="font" type="font/woff2"href="./static.files/$\{f}">\`).join(""))</script>
<link rel="stylesheet" href="./static.files/${normCss}">
<link rel="stylesheet" href="./static.files/${rdocCss}">
<meta name="rustdoc-vars" data-root-path="./" data-static-root-path="./static.files/" data-current-crate="stdx_rs" data-themes="" data-resource-suffix="" data-rustdoc-version="${rustdocVersion}" data-channel="${channel}" data-search-js="${searchJs}" data-stringdex-js="${stringdexJs}" data-settings-js="${settingsJs}">
<script src="./static.files/${storageJs}"></script>
<script defer src="./crates.js"></script>
<script defer src="./static.files/${mainJs}"></script>
<noscript><link rel="stylesheet" href="./static.files/${noscriptCss}"></noscript>
<link rel="alternate icon" type="image/png" href="./static.files/${faviconPng}">
<link rel="icon" type="image/svg+xml" href="./static.files/${faviconSvg}">
</head><body class="rustdoc mod crate">
<a class="skip-main-content" href="#main-content">Skip to main content</a>
<rustdoc-topbar><h2><a href="#">Workspace stdx</a></h2></rustdoc-topbar>
<nav class="sidebar">
<div class="sidebar-crate"><h2><a href="./index.html">stdx</a></h2></div>
<div class="sidebar-elems">
<section id="rustdoc-toc"><h3><a href="#">Crates</a></h3><ul class="block">${crates.map(c => `<li><a href="#crate-${c.name}" title="${escapeHtml(c.description)}">${c.name.replace(/_/g, "_<wbr>")}</a></li>`).join("")}</ul></section>
</div>
</nav>
<div class="sidebar-resizer" title="Drag to resize sidebar"></div>
<main><div class="width-limiter">
<section id="main-content" class="content" tabindex="-1">
<div class="main-heading"><h1>stdx <span>workspace</span></h1></div>
<div class="docblock"><p>This workspace contains ${crates.length} crates.</p></div>
<h2 id="crates" class="section-header">All Crates<a href="#crates" class="anchor">§</a></h2>
<dl class="item-table" id="crate-list">
${rows}
</dl>
</section></div></main>
</body></html>`;

writeFileSync(join(docDir, "index.html"), indexHtml);
console.log(`Generated index.html with ${crates.length} crates`);
