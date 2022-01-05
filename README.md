

# Squares-dao
### NFT spec
* Number of NFT based on how many were uploaded during instantiation.
* Limit number of NFT's to 10 per wallet.

### Testing on local terra
```
terrad tx wasm store artifacts/squares.wasm --from user1 --chain-id=localterra --gas=auto --fees=100000uluna --broadcast-mode=block
terrad tx wasm instantiate 1 '{"base":{"minter":"terra1dcegyrekltswvyy0xy69ydgxn9x8x32zdtapd8","name":"test123","symbol":"lol"},"tokens":[{"uri":"something1"},{"uri":"something2"}]}' --from user1 --chain-id=localterra --fees=10000uluna --gas=auto --broadcast-mode=block
terrad tx wasm execute terra18vd8fpwxzck93qlwghaj6arh4p7c5n896xzem5 '{"mint":{"token_id":"","owner":"","token_uri":"","extension":{"uri":""}}}' --from user1 --chain-id=localterra --fees=1000000uluna --gas=auto --broadcast-mode=block
terrad query wasm contract-store terra18vd8fpwxzck93qlwghaj6arh4p7c5n896xzem5 '{"all_nft_info":{"token_id":"1"}}'
```

