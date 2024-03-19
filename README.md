# json_chunker
Chunking large json files into smaller chunks. Written in Rust 🦀

Important: I have not tested this with huge files and it used around 30 GB ram to split an 8 GB file, so make sure to downloadmoreram.com before you run.

Usage:
```
mkdir chunks
git clone https://github.com/chryzsh/json_chunker
cd json_chunker
cargo run -- -i ../big-json-file.json -o ../chunks -c 100
```

I wrote this mainly to replace https://github.com/ustayready/ShredHound when shredding an 8 GB azurehound file because it was slow and kept crashing.

Protip: more chunks increases performance. If you are importing into Bloodhound Community Edition you should probably use 100-200 chunks to ensure you don't get chunks larger than 1 GB, which is the current limit. I shredded an 8 GB file in around 20 minutes.
