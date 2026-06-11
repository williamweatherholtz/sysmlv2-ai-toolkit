"""Sweep orphaned pilot-kernel JVMs (CR-11). Interrupted tool runs (timeouts,
sandbox kills, pipe hangs) never reach teardown and each leaks a JVM — 75 were
found during the 2026-06-11 critique. Safe: matches only the ISysML kernel class.

Run:  python .engine/tools/kill_stale_kernels.py
"""
import subprocess
import sys

PS = ("Get-CimInstance Win32_Process -Filter \"name='java.exe'\" | "
      "Where-Object { $_.CommandLine -match 'ISysML' } | "
      "ForEach-Object { try { Stop-Process -Id $_.ProcessId -Force -ErrorAction Stop; "
      "Write-Output $_.ProcessId } catch {} }")


def main():
    if sys.platform == "win32":
        r = subprocess.run(["powershell", "-NoProfile", "-Command", PS],
                           capture_output=True, text=True)
        killed = [p for p in r.stdout.split() if p.strip()]
    else:
        r = subprocess.run(["pkill", "-f", "ISysML"], capture_output=True)
        killed = ["(pkill)"] if r.returncode == 0 else []
    print(f"swept {len(killed)} stale kernel JVM(s)")


if __name__ == "__main__":
    main()
