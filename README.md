# whale_watcher_server
1. Setup: must download Docker, install postgres on it as well as psqli.
2. Run: chmod +x scripts/init_db.sh
3. Run: ./scripts/init_db.sh (any subsequent calls should be (SKIP_DOCKER=true ./scripts/init_db.sh)
4. Run: cargo run
5. The terminal output will show you the address and port the server is running on.
6. In a separate terminal, run: curl -v https://127.0.0.1:8000 or curl -v https://127.0.0.1:8000/{some_route}

Download and install Postgres (once you install docker): https://hub.docker.com/_/postgres
Download Psql: https://blog.timescale.com/blog/how-to-install-psql-on-mac-ubuntu-debian-windows/
