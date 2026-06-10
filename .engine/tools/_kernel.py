"""Shared SysML v2 pilot-kernel driver.

One place for the kernel lifecycle so the teardown fix lives once (used by the
validators and by whats-next). Drives the OMG pilot Jupyter kernel via
jupyter_client; cell results arrive over ZMQ (iopub), NOT the kernel's stdout.

OS-SPECIFICITY of the teardown (the bug we fixed) — read before "simplifying":
  * stdout/stderr -> DEVNULL  ........ CROSS-PLATFORM. The kernel JVM otherwise
    inherits our stdout pipe, so a reader (or `conda run`) blocks waiting for
    EOF and the shell hangs even after Python exits. DEVNULL also drops the
    library-load noise. Results are unaffected (they come over ZMQ).
  * NOT calling km.shutdown_kernel(now=True) ... it can BLOCK (observed on
    Windows); harmless to avoid everywhere.
  * os.kill(provisioner.pid, _KILL_SIG) ... the SIGNAL is OS-specific: Windows
    has no SIGKILL and os.kill maps any signal to a forceful TerminateProcess,
    so SIGTERM suffices; POSIX SIGTERM is catchable, so we force with SIGKILL.
  * os._exit() ........................ CROSS-PLATFORM. Bypasses jupyter_client's
    lingering non-daemon threads (which also keep the process alive).
  * km.kernel was REMOVED in jupyter_client 8.x; the process lives under
    km.provisioner (this is a library-version fact, not OS-specific).

Requires the `sysml` conda env; the kernelspec calls bare `java`, so callers
must run through `conda run -n sysml` (sandbox disabled).
"""
import os
import signal
import subprocess
from queue import Empty

# POSIX: force-kill (SIGTERM is catchable). Windows: SIGKILL is undefined and
# os.kill maps any signal to TerminateProcess, so SIGTERM is already forceful.
_KILL_SIG = getattr(signal, "SIGKILL", signal.SIGTERM)


def start():
    """Start the sysml kernel with stdout/stderr suppressed. Returns (km, kc)."""
    from jupyter_client.manager import start_new_kernel
    return start_new_kernel(kernel_name="sysml",
                            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)


def run_cell(kc, code, timeout=180):
    """Execute one cell; return (status, text) where text is the joined iopub
    stream/result/error output (the kernel's textual responses, incl. %magics)."""
    msg_id = kc.execute(code, allow_stdin=False)
    status = "unknown"
    while True:
        try:
            r = kc.get_shell_msg(timeout=timeout)
        except Empty:
            status = "timeout"
            break
        if r["parent_header"].get("msg_id") == msg_id:
            status = r["content"].get("status", "unknown")
            break
    outs = []
    while True:
        try:
            m = kc.get_iopub_msg(timeout=timeout)
        except Empty:
            break
        if m["parent_header"].get("msg_id") != msg_id:
            continue
        t = m["header"]["msg_type"]
        c = m["content"]
        if t == "status" and c.get("execution_state") == "idle":
            break
        elif t == "stream":
            outs.append(c.get("text", ""))
        elif t == "error":
            outs.append(c.get("evalue", "") + "\n" + "\n".join(c.get("traceback", [])))
        elif t in ("execute_result", "display_data"):
            outs.append(str(c.get("data", {}).get("text/plain", "")))
    return status, "\n".join(outs)


def teardown_and_exit(km, code=0):
    """Reap the kernel JVM (non-blocking, OS-aware) and hard-exit so the calling
    shell returns cleanly. Does NOT return."""
    import sys
    sys.stdout.flush()
    try:
        os.kill(km.provisioner.pid, _KILL_SIG)
    except Exception:
        pass
    os._exit(code)
