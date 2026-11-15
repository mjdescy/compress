use clap::ValueEnum;

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum Algorithm {
    #[value(name = "7z")]
    SevenZ,
    Bzip2,
    Gzip,
    Lz4,
    Tar,
    Xz,
    Zstd,
    Zip,
}

impl Algorithm {
    pub fn file_extension(&self) -> String {
        match self {
            Algorithm::SevenZ => "7z",
            Algorithm::Bzip2 => "bz2",
            Algorithm::Gzip => "gz",
            Algorithm::Lz4 => "lz4",
            Algorithm::Tar => "tar",
            Algorithm::Xz => "xz",
            Algorithm::Zstd => "zst",
            Algorithm::Zip => "zip",
        }
        .to_string()
    }

    pub fn multiple_files_requires_tar_before_compression(&self) -> bool {
        match self {
            Algorithm::Tar | Algorithm::Zip | Algorithm::SevenZ => false,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_extension_returns_expected_extension_for_each_algorithm() {
        let cases = [
            (Algorithm::SevenZ, "7z"),
            (Algorithm::Bzip2, "bz2"),
            (Algorithm::Gzip, "gz"),
            (Algorithm::Lz4, "lz4"),
            (Algorithm::Tar, "tar"),
            (Algorithm::Xz, "xz"),
            (Algorithm::Zstd, "zst"),
            (Algorithm::Zip, "zip"),
        ];
        for (algorithm, expected) in cases {
            assert_eq!(algorithm.file_extension(), expected, "{:?}", algorithm);
        }
    }

    #[test]
    fn multiple_files_do_not_require_tar_for_archive_formats() {
        for algorithm in [Algorithm::Tar, Algorithm::Zip, Algorithm::SevenZ] {
            assert!(
                !algorithm.multiple_files_requires_tar_before_compression(),
                "{:?}",
                algorithm
            );
        }
    }

    #[test]
    fn multiple_files_require_tar_for_single_stream_compressors() {
        for algorithm in [
            Algorithm::Bzip2,
            Algorithm::Gzip,
            Algorithm::Lz4,
            Algorithm::Xz,
            Algorithm::Zstd,
        ] {
            assert!(
                algorithm.multiple_files_requires_tar_before_compression(),
                "{:?}",
                algorithm
            );
        }
    }
}
