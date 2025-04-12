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
  database:
    image: pgedge/pgedge:pg16-latest
    ports:
      - "5432:5432"
    volumes:
      - "./db.json:/home/pgedge/db.json"
```

## Database Configuration
Database configuration is managed in a db.json file, you can find more information on this on the [pdEdge Repo](https://github.com/pgEdge/pgedge-docker).

## API Documentation
The API can be explored and tested using the Bruno collections provided in the ```/bruno``` directory.
