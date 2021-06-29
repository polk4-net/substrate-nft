# Substrate !ink NFT simple implementation
***
This is a simple working version of base NFT smart contract written using latest (as of this date) !ink `3.0.0-rc3`.

To run all tests:
``` 
cargo +nightly test
```
To compile to WASM:
``` 
cargo +nightly contract build 
```
Original wasm size: 47.4K, Optimized: 26.3K

Once your contract is compiled you can find artifacts in:
<PROJECT_DIR>/target/ink

- nft.contract (code + metadata)
- nft.wasm (the contract's code)
- metadata.json (the contract's metadata)

If you are facing any issues please follow the setup step of substrate environment at:

https://substrate.dev/substrate-contracts-workshop/#/0/setup

## Testing on using Polk4 canvas network
***
You can upload this contract and interract with it either via your own local Canvas node or using Polk4 Canvas testnet.

- Head to https://paritytech.github.io/canvas-ui/
- In node settings input custom node endpoint: wss://canvas.polk4.net
- Upload `nft.contract` into the node via UI  
- Instantiate a contract and make RPC calls/transactions 🎉

## License
This NFT template is [MIT](LICENSE)
