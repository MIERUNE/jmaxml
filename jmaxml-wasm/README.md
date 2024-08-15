# jmaxml-wasm

## Example

```javascript
import init, { parseXml } from "jmaxml-wasm";
await init({});

const xml = await fetch("20240814050750_0_VPRN50_010000.xml");
xml.text().then((text) => {
    const result = parseXml(text);
    console.log(result);
});
```

## Build

```bash
./build.sh
```
