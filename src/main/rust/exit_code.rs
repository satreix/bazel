/// Exit codes for Blaze.
///
/// Must be kept in sync with the Java counterpart under
/// com/google/devtools/build/lib/util/ExitCode.java

pub enum ExitCode {
    // Success.
    Success = 0,

    // Command Line Problem, Bad or Illegal flags or command combination, or
    // Bad environment variables. The user must modify their command line.
    BadArgv = 2,

    // The user interrupted the build, most probably with Ctrl-C.
    Interrupted = 8,

    // The client or server lock is held, and --noblock_for_lock was passed, so
    // this command fails fast.
    LockHeldNoblockForLock = 9,

    // Something is wrong with the host Bazel is running on and a re-run of the
    // same command probably will not help.
    LocalEnvironmentalError = 36,

    // Unexpected server termination, due to e.g. external SIGKILL, misplaced
    // System.exit(), or a JVM crash.
    // This exit code should be a last resort.
    InternalError = 37,
}
