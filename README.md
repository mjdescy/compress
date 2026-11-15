# Compress

This is a simple learning project that has little practical value. It wraps command-line tools for compression programs so the user does not need to remember all their different command-line parameters.

## Usage

```sh
compress [--delete] [--algorithm] <input> [output]
```

The output file name extension is used to determine the compression program to use.

If the output filename is omitted, the program uses the same basename as the first input file, with an extension representing the compression program. The default compression algorithm is `zstd`.

The `--delete` (`-d`) flag deletes the input file(s) after the archive file has been created.

The `--algorithm` (`-a`) flag is used to specify the compression algorithm. The default method is `zstd`.

## Supported compression algorithms

Initially, compression will be performed by shelling out to external compression programs.

| Format | File Extension | External Compression Program |
| ------ | -------------- | ---------------------------- |
| 7z | .7z | 7zz |
| bzip2 | .bz2 | bzip2 |
| gzip | .gz | gz |
| lz4 | .lz4 | lz4 |
| tar | .tar | tar |
| xz | .xz | xz |
| zstd | .zstd | zstd |
| zip | .zip | zip |
