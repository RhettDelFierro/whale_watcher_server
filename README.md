# whale_watcher_server
1. Run: cargo run
2. The terminal output will show you the address and port the server is running on.
3. In a separate terminal, run: curl -v {whatever that address was} or curl -v {whatever that address was}/{some_route}

Ex: Output when you run cargo run: ```Running on http://127.0.0.1:51194/```

Means run: curl -v  http://127.0.0.1:51194/health_check
