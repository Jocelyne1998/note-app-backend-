 ## Note app with Rocket  API
Microservices using the Rost language of a notes application using the backend which allows making a CRUD in the database.

# You need to install Rust (follow the linking below)
https://www.rust-lang.org/tools/install

# Make sure you are using the nightly channel.
rustup toolchain install nightly
rustup default nightly

## Add a PostgreSQL Database

# Run and attach tty session
docker-compose up 
(if you have problems with permissions you must use "sudo docker-compose up")

# Run app in the background
docker-compose up -d

# Get psql client connected to the library db
docker exec -ti conection_postgres psql -U postgres -d library

# Run Test
cargo install cargo-watch
cargo watch -x test