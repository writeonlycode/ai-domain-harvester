# PhilEntries

A high-performance Rust pipeline for extracting and structuring philosophical
encyclopedia entries from authoritative sources like the Stanford Encyclopedia
of Philosophy (SEP), with support for additional sources (IEP and others).

## Overview

PhilEntries crawls and parses long-form philosophy articles and transforms them
into clean, structured data for downstream use such as search, analysis, or
knowledge systems.

## Features

- Async crawling of philosophy reference sites
- HTML parsing and structured extraction
- Normalized entry format (title, authors, sections, references)
- Designed for multi-source expansion (SEP, IEP, etc.)
- Efficient, concurrent Rust implementation

## Pipeline

1. Fetch index of entries
2. Extract article links
3. Fetch individual articles
4. Parse and normalize content
5. Export structured dataset (JSON/CSV)

## Output Example

```json
{
  "title": "Free Will",
  "authors": ["Author Name"],
  "sections": [
    { "heading": "Introduction", "content": "..." }
  ],
  "sources": ["SEP"]
}
````

## Goal

To build a clean, extensible knowledge ingestion layer for philosophical texts.
