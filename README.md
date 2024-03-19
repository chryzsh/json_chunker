# json_chunker
Chunking large json files into smaller chunks

I wrote this mainly to replace https://github.com/ustayready/ShredHound when shredding an 8 GB azurehound file because it was slow and kept crashing.

Protip: more chunks increases performance. If you are importing into Bloodhound Community Edition you should probably use 100-200 chunks to ensure you don't get chunks larger than 1 GB, which is the current limit.


Example:
```
mkdir chunks
git clone https://github.com/chryzsh/json_chunker
cd json_chunker
cargo run -- -i bigfile.json -o chunks -c 100
```
