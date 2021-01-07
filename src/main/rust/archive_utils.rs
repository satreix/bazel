use std::io::prelude::*;

// FIXME remove C++ imports
//
// #include <functional>
// #include <memory>
// #include <string>
// #include <vector>
//
// #include "src/main/cpp/archive_utils.h"
// #include "src/main/cpp/blaze_util_platform.h"
// #include "src/main/cpp/util/errors.h"
// #include "src/main/cpp/util/exit_code.h"
// #include "src/main/cpp/util/file.h"
// #include "src/main/cpp/util/logging.h"
// #include "src/main/cpp/util/path.h"
// #include "src/main/cpp/util/strings.h"
// #include "third_party/ijar/zip.h"
//
// using std::string;
// using std::vector;

struct PartialZipExtractor {
    // struct PartialZipExtractor : public devtools_ijar::ZipExtractorProcessor {
    //   using CallbackType =
    //       std::function<void(const char *name, const char *data, size_t size)>;
    stop_name: String,
    stop_value: String,
    seen_names: Vec<String>,
    //   CallbackType callback_;
    done: bool,
}

impl PartialZipExtractor {
    fn new() -> Self {
        Self {
            stop_name: "".to_string(),
            stop_value: "".to_string(),
            seen_names: vec![],
            done: false,
        }
    }

    /// Scan the zip file "archive_path" until a file named "stop_entry" is seen,
    /// then stop.
    ///
    /// If entry_names is not null, it receives a list of all file members
    /// up to and including "stop_entry".
    /// If a callback is given, it is run with the name and contents of
    /// each such member.
    ///
    /// Returns the contents of the "stop_entry" member.
    fn unzip_until(&self) {
        //   string unzip_until(const string &archive_path, const string &stop_entry,
        //                     vector<string> *entry_names = nullptr,
        //                     CallbackType &&callback = {})

        unimplemented!();
        //     std::unique_ptr<devtools_ijar::ZipExtractor> extractor(
        //         devtools_ijar::ZipExtractor::Create(archive_path.c_str(), this));
        //     if (!extractor) {
        //       BAZEL_DIE(blaze_exit_code::LOCAL_ENVIRONMENTAL_ERROR)
        //           << "Failed to open '" << archive_path
        //           << "' as a zip file: " << blaze_util::GetLastErrorString();
        //     }
        //     stop_name_ = stop_entry;
        //     seen_names_.clear();
        //     callback_ = callback;
        //     done_ = false;
        //     while (!done_ && extractor->ProcessNext()) {
        //       // Scan zip until EOF, an error, or Accept() has seen stop_entry.
        //     }
        //     if (const char *err = extractor->GetError()) {
        //       BAZEL_DIE(blaze_exit_code::LOCAL_ENVIRONMENTAL_ERROR)
        //           << "Error reading zip file '" << archive_path << "': " << err;
        //     }
        //     if (!done_) {
        //       BAZEL_DIE(blaze_exit_code::LOCAL_ENVIRONMENTAL_ERROR)
        //           << "Failed to find member '" << stop_entry << "' in zip file '"
        //           << archive_path << "'";
        //     }
        //     if (entry_names) *entry_names = std::move(seen_names_);
        //     return stop_value_;
    }

    fn accept() {
        //   bool Accept(const char *filename, devtools_ijar::u4 attr) override {

        unimplemented!();
        //     if (devtools_ijar::zipattr_is_dir(attr)) return false;
        //     // Sometimes that fails to detect directories.  Check the name too.
        //     string fn = filename;
        //     if (fn.empty() || fn.back() == '/') return false;
        //     if (stop_name_ == fn) done_ = true;
        //     seen_names_.push_back(std::move(fn));
        //     return done_ || !!callback_;  // true if a callback was supplied
    }

    fn process() {
        //   void Process(const char *filename, devtools_ijar::u4 attr,
        //                const devtools_ijar::u1 *data, size_t size) override {

        unimplemented!();
        //     if (done_) {
        //       stop_value_.assign(reinterpret_cast<const char *>(data), size);
        //     }
        //     if (callback_) {
        //       callback_(filename, reinterpret_cast<const char *>(data), size);
        //     }
    }
}

/// Determines the contents of the archive, storing the names of the contained
/// files into `files` and the install md5 key into `install_md5`.
pub fn determine_archive_contents() {
    // void determine_archive_contents(const std::string &archive_path,
    //                               std::vector<std::string> *files,
    //                               std::string *install_md5);

    unimplemented!();
    // PartialZipExtractor pze;
    // *install_md5 = pze.unzip_until(archive_path, "install_base_key", files);
}

/// Extracts the embedded data files in `archive_path` into `output_dir`.
/// It's expected that `output_dir` already exists and that it's a directory.
/// Fails if `expected_install_md5` doesn't match that contained in the archive,
/// as this could indicate that the contents has unexpectedly changed.
pub fn extract_archive_or_die() {
    // void extract_archive_or_die(const std::string &archive_path,
    //                          const std::string &product_name,
    //                          const std::string &expected_install_md5,
    //                          const std::string &output_dir);

    unimplemented!();
    //   string error;
    //   std::unique_ptr<blaze::embedded_binaries::Dumper> dumper(
    //       blaze::embedded_binaries::Create(&error));
    //   if (dumper == nullptr) {
    //     BAZEL_DIE(blaze_exit_code::LOCAL_ENVIRONMENTAL_ERROR) << error;
    //   }
    //
    //   if (!blaze_util::PathExists(output_dir)) {
    //     BAZEL_DIE(blaze_exit_code::INTERNAL_ERROR)
    //         << "Archive output directory didn't exist: " << output_dir;
    //   }
    //
    //   BAZEL_LOG(USER) << "Extracting " << product_name << " installation...";
    //
    //   PartialZipExtractor pze;
    //   string install_md5 = pze.unzip_until(
    //       archive_path, "install_base_key", nullptr,
    //       [&](const char *name, const char *data, size_t size) {
    //         dumper->Dump(data, size, blaze_util::JoinPath(output_dir, name));
    //       });
    //
    //   if (!dumper->Finish(&error)) {
    //     BAZEL_DIE(blaze_exit_code::LOCAL_ENVIRONMENTAL_ERROR)
    //         << "Failed to extract embedded binaries: " << error;
    //   }
    //
    //   if (install_md5 != expected_install_md5) {
    //     BAZEL_DIE(blaze_exit_code::LOCAL_ENVIRONMENTAL_ERROR)
    //         << "The " << product_name << " binary at " << archive_path
    //         << " was replaced during the client's self-extraction (old md5: "
    //         << expected_install_md5 << " new md5: " << install_md5
    //         << "). If you expected this then you should simply re-run "
    //         << product_name
    //         << " in order to pick up the different version. If you didn't expect "
    //            "this then you should investigate what happened.";
    //   }
}

/// Retrieves the build label (version string) from `archive_path` into
/// `build_label`.
pub fn extract_build_label(archive_path: &str) -> String {
    let fname = std::path::Path::new(archive_path);
    let zipfile = std::fs::File::open(&fname).unwrap();
    let mut archive = zip::ZipArchive::new(zipfile).unwrap();

    let filename = "build-label.txt";
    let mut file = match archive.by_name(filename) {
        Ok(file) => file,
        Err(..) => { panic!("File {} not found", filename); }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

/// Returns the server jar path from the archive contents.
pub fn get_server_jar_path() {
    // std::string get_server_jar_path(const std::vector<std::string> &archive_contents);

    unimplemented!();
    // if (archive_contents.empty()) {
    //   BAZEL_DIE(blaze_exit_code::LOCAL_ENVIRONMENTAL_ERROR)
    //       << "Couldn't find server jar in archive";
    // }
    // return archive_contents[0];
}
