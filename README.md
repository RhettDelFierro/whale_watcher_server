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

Get Requests to fetch data:
https://whalewatcherserver-th48j.ondigitalocean.app/holders/?network=bsc&contract_address=rereshfdzfdxgfx
