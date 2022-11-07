> ⚠️ **please note** that this is mostly just brainstorming, not describing what
> actually exists.

---

## 📖 Fiction app

i want a tool to convert eBooks to audiobooks, packaged as private podcasts, for
my personal convenience.

## features

- Reads eBooks from
  - EPUB files
  - archiveofourown.org
  - fanfiction.net
  - royalroad.com
- Generates:
  - Opus audio using Microsoft's text-to-speech and FFMPEG.
  - EPUB with embedded aligned synchronized audio.
  - MKV/WebM files, with embedded synchronized text, both individually
    per-chapter and as a single file with embedded chapter markers.
  - (Private) postcast RSS feeds indexing the generated media files.

## ⚙️ Incremental engine

fiction's engine is a sort-of incremental-computation content-based-addressing
query-driven caching engine. Built on top of SQLite and Postcard for now, maybe
using Protocol Buffers later. The main thing distinguishing it from alternatives
like Salsa and Turbo is that it sucks. Secondary to that, fiction is focused on
data that can be cached long-term, to disk, while Salsa seems to be focused on
data that's stored in-memory, and Turbo confused me. Aiming for some kind of
Turbo extension in the future might make sense.

### Storage Layers

An implementation of the Storage layer will provide one or both of:

1. Content-addressed storage of byte strings, between 32B and 1 GiB in size,
   each identified by its 32B BLAKE3 hash digest.
2. Storage of Request-Response pairs, each 32B (addressing a stored blob) or
   less (a short inlined value). Each pair should include `inserted_at` and
   `validated_at` fields, indicating the first and most-recent times that a
   given Request-Response pair was inserted. Should be indexed by `Request`
   hashed, followed by the `validated_at` timestamp descending, so that the most
   recent known `Response` can quickly be found for a given `Request`.

Depending on the desired behaviour, a storage implementation may drop items, or
only save the most recent, or similar. They may be read-only and not support
writing.

The built-in storage layer implementations include:

- entirely in-memory
- SQLite (in-memory or on disk)
- Web (read-only via HTTP(S))
- Static (read-only compiled-in with databake)

and a meta layer type that can be used to read/write from multiple different
storage implementations with customizable behaviour.

### Requests and Responses

They're serialized with Postcard now but I'd like to do something that's
serde-compatible while at least being forward-compatible with protocol buffers.

## 👑 Ser Proto serialization

a serde serialization implementation using a subset of protocol buffers wire format. the name is a reference to Cap'n Proto.
