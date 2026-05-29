/**
 * Generiert eine 1024x1024-PNG aus der SVG-Quelle fuer das Tauri-Icon-Set.
 *
 * Das thorly-Logo (logo-symbol.svg) hat einen Brand-Emerald-Hintergrund mit
 * weissem thorly-Mark. Wir rendern das auf 1024x1024 mit etwas Padding,
 * damit Tauri-Icon (das daraus icns/ico/PNG-Set erzeugt) sauber arbeitet.
 *
 * Run: bun run scripts/build-icon-source.ts
 */
import sharp from 'sharp';
import { readFileSync, writeFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT = join(__dirname, '..');
const SVG_PATH = join(ROOT, 'src-tauri/icons/icon-source.svg');
const PNG_PATH = join(ROOT, 'src-tauri/icons/icon-source.png');

const svg = readFileSync(SVG_PATH, 'utf-8');

// Sharp rendert SVG in 1024x1024 PNG. density=512 gibt scharfes Anti-Aliasing.
await sharp(Buffer.from(svg), { density: 512 })
  .resize(1024, 1024, {
    fit: 'contain',
    background: { r: 5, g: 150, b: 105, alpha: 1 }, // Brand-Emerald
  })
  .png()
  .toFile(PNG_PATH);

console.log(`✓ Icon-Source erzeugt: ${PNG_PATH}`);
