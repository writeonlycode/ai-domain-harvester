# ai-domain-harvester

Concurrent Rust CLI for scraping and structuring `.ai` domain sales from forum
data.

## Overview

This project extracts historical `.ai` domain sales from forum threads and
converts them into structured datasets. It is designed as a high-performance
data pipeline using async Rust.

## Pipeline

The system operates in two stages:

1. **Thread Discovery**
   - Fetch forum index pages
   - Extract thread identifiers

2. **Thread Processing**
   - Fetch individual threads
   - Extract and normalize domain sales

```
raw HTML → DOM → extracted text → structured data
````

## Features

- Concurrent scraping with `tokio` and `futures`
- HTML parsing with `scraper`
- CSV export

## Usage

```bash
ai-domain-harvester scrape --since 2023-01-01 --output sales.csv
````

## Example Output

```csv
domain,price_usd,date,venue
choc.ai,4500,2024-01-12,Dynadot
alpha.ai,12000,2024-01-12,Sedo
```

## Motivation

This project demonstrates how to build a robust pipeline to extract meaningful,
structured information from noisy sources.

## Tech Stack

* Rust
* tokio
* futures
* reqwest
* scraper
* csv
