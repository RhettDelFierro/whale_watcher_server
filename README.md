# whale_watcher_server
Requests now work live on production:

POST requests should go to this link as Content-Type: x-www-form-urlencoded:

https://whalewatcherserver-th48j.ondigitalocean.app/holders
``` 
network: 'bsc',
token_name: santa coin,
contract_address: 'rereshfdzfdxgfx',
holder_address: 'kugkuykjfkjf',
place: 10,
amount: 12345.0012345
```

Get Requests to fetch data for all holders: (query parameters are "network" and contract_address):

https://whalewatcherserver-th48j.ondigitalocean.app/holders/list?network=bsc&contract_address=rereshfdzfdxgfx


**Scammers:**

Post request to:

https://whalewatcherserver-th48j.ondigitalocean.app/scam/creators
Params:
```
address: address of the token creator
notes: any notes you may think of.
network_of_scammed_token: eth, bsc, etc.
scammed_contract_address: contract address of crappy token.
```

To check if a token creator has scammed in the past:

Send a Get request to:

https://whalewatcherserver-th48j.ondigitalocean.app/scam/creators/list
Query parameters:
```
network: (supported networks so far: ETH, BSC, ADA, AVAX, MATIC, FTM, SOL, LUNA, DOT, MOVR)
scammer_address: the token creator's address who you want to check.

Example: 127.0.0.1:8000/scam/creators/list?network=eth&scammer_address=0x51D6B827246489Dde847D3dab0b9A6d095017C97
```

To edit tables use:
```
sqlx migrate add <your migration>
SKIP_DOCKER=true ./scripts/init_db.sh
```

Any changes to writing sql on the routes will need to be prepared using sqlx for easier deployment:
```
cargo clean
cargo sqlx prepare -- --bin whale_watcher_server
```