# Rustic-Validator

A Rust application for synchronizing and managing proxy information between the Webshare API and a PostgreSQL database.

## Description

Rustic-Validator automatically fetches proxy information from the Webshare API, synchronizes the data with a PostgreSQL database, and performs the necessary operations to keep the database up-to-date:
- Adds new proxies found in the API
- Updates existing proxies with the latest information
- Removes proxies that are no longer available in the API

## Features

- Asynchronous operation using Tokio
- PostgreSQL database integration
- RESTful API communication
- Environment-based configuration
- Detailed operation logging

## Prerequisites

- Rust and Cargo installed (https://www.rust-lang.org/tools/install)
- PostgreSQL database
- Webshare API key

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/your-username/rustic-validator.git
   cd rustic-validator
   ```

2. Create a `.env` file in the project root with the following content:
   ```
   APIKEY=your_webshare_api_key
   DATABASE_URL=postgresql://username:password@localhost/database_name
   ```

3. Build the project:
   ```
   cargo build --release
   ```

## Database Setup

Ensure your PostgreSQL database has a `proxy` table with the following structure:

```sql
CREATE TABLE proxy (
    ip VARCHAR(255) PRIMARY KEY,
    port INTEGER NOT NULL,
    username VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL
);
```

## Usage

Run the application using:

```
cargo run --release
```

The application will:
1. Connect to the database
2. Fetch existing proxy records
3. Retrieve proxy data from the Webshare API (three pages)
4. Compare the API data with the database data
5. Delete, update, or insert proxies as necessary
6. Display a summary of changes

## Output Example

```
Found 150 IPs in database
Page 1: Found 100 proxies
Page 2: Found 100 proxies
Page 3: Found 100 proxies
Deleted IP: 203.0.113.1
Inserted IP: 198.51.100.1
Summary: 5 deleted, 145 updated, 50 inserted
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Author

Pedro Afonso
