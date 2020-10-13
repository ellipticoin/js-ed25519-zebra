# Ed25519 Zebra

Wrapper around the [ed25519-zebra](https://crates.io/crates/ed25519-zebra) Rust crate.

## Usage
```
import init, { sign, keyPair, verify } from './pkg/ed25519_zebra.js';

async function run() {
    await init();
    const message = Uint8Array.from([1,2,3]);
    const [sk, vk] = keyPair();
    const signature = sign(message, sk);
    console.log(verify(message, signature, vk));
}

run();
```
