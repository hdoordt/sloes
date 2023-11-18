# Sloes

Analyzing proxy

## Design

- Generates root cert on first run
  - Generates ssh certs for each request domain on demand
- Hyper proxy server running locally
  - Decrypts https stuff using system certs
  - re-enctrypts stuff with generated domain cert
  - Stores request + response data in sqlite db
- Native ui that allows for analysis of sqlite data