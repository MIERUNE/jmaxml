# jmaxml-wasm

## Example

```javascript
import init, { parseXml } from "jmaxml-wasm";
await init({});

// ESM (Cloudflare Workers, etc.)
// 
// import { initSync, parseXml } from "jmaxml-wasm";
// import jmaxmlWasm from "jmaxml-wasm/jmaxml_wasm_bg.wasm";
// initSync({ module: jmaxmlWasm });

const xml = await fetch("20240814050750_0_VPRN50_010000.xml");
const result = parseXml(await xml.text());
console.log(result);
```

## Build

```bash
./build.sh
```
