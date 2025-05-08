# Selfless
Selfless is a media server written in Rust, designed to efficently manage and serve your media library.

## Features
- Media organization and management
- Media streaming capabilities
- User management
- RESTful API

## Roadmap
- Android TV interface
- Web interface
- Admin interface
- Transcoding

## Prerequisites
- pgEdge PostgreSQL database

## Getting Started
You can quickly start the server using this docker compose file
```
services:
  server:
    image: ghcr.io/reilley64/sfls/server:master
    environment:
      RUST_LOG: info
      DATABASE_URL: postgresql://sfls:sfls@database:5432/sfls
    ports:
      - "10000:8080"
    depends_on:
      - database

  gateway:
    image: ghcr.io/reilley64/sfls/gateway:master
    environment:
      BASE_URL: http://server:8080
    ports:
      - "10001:3000"
    depends_on:
      - server

  database:
    image: pgedge/pgedge:pg16-latest
    volumes:
      - "./db.json:/home/pgedge/db.json"
```

## Database Configuration
Database configuration is managed in a db.json file, you can find more information on this on the [pdEdge Repo](https://github.com/pgEdge/pgedge-docker).

## API Documentation
The API can be explored and tested using the Bruno collections provided in the ```/bruno``` directory.
