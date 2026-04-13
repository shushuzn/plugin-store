#!/usr/bin/env python
"""Plugin Store CLI version checker + auto-updater.
Compatible with Python 2.6+ and Python 3.x.
Called by CLI wrapper on every invocation."""
import os, sys, json, time, subprocess

CACHE_DIR = os.path.join(os.path.expanduser("~"), ".plugin-store", "version-cache")
REGISTRY_URL = "https://raw.githubusercontent.com/okx/plugin-store/main/registry.json"
CHECK_INTERVAL = 3600  # 1 hour

def check(name, current_version):
    cache_file = os.path.join(CACHE_DIR, name)

    # Cache check
    try:
        if os.path.exists(cache_file):
            if time.time() - os.path.getmtime(cache_file) < CHECK_INTERVAL:
                return
    except Exception:
        pass

    # Fetch latest version
    try:
        from urllib.request import urlopen, Request
    except ImportError:
        from urllib2 import urlopen, Request

    try:
        req = Request(REGISTRY_URL, headers={"User-Agent": "plugin-store-updater"})
        resp = urlopen(req, timeout=5)
        data = json.loads(resp.read().decode("utf-8"))

        latest = None
        for p in data.get("plugins", []):
            if p.get("name") == name:
                latest = p.get("version")
                break

        if not latest:
            return

        # Update cache
        try:
            os.makedirs(CACHE_DIR)
        except OSError:
            pass
        try:
            with open(cache_file, "w") as f:
                f.write(latest)
        except Exception:
            pass

        if latest == current_version:
            return

        # Version outdated — decide action based on terminal mode
        is_interactive = hasattr(sys.stdin, 'isatty') and sys.stdin.isatty()

        if is_interactive:
            # Interactive terminal: ask user
            sys.stderr.write("\n\033[33m" + chr(0x1f504) + " " + name + " v" + current_version + " " + chr(0x2192) + " v" + latest + " available. Update now? [Y/n] \033[0m")
            sys.stderr.flush()
            try:
                answer = sys.stdin.readline().strip().lower()
                if answer == "n":
                    return
            except Exception:
                pass
            sys.stderr.write("\033[33m   Updating in background...\033[0m\n")
            sys.stderr.flush()
        else:
            # Non-interactive (AI agent / pipe / CI): auto-update silently
            sys.stderr.write("\033[33m" + chr(0x26a0) + chr(0xfe0f) + " " + name + " v" + current_version + " " + chr(0x2192) + " v" + latest + " updating in background...\033[0m\n")
            sys.stderr.flush()

        # Background update via npx skills add (non-blocking)
        devnull = open(os.devnull, "w")
        try:
            subprocess.Popen(
                ["npx", "skills", "add", "okx/plugin-store", "--skill", name, "--yes"],
                stdout=devnull,
                stderr=devnull,
                close_fds=True
            )
        except Exception:
            pass

    except Exception:
        pass

if __name__ == "__main__":
    if len(sys.argv) == 3:
        check(sys.argv[1], sys.argv[2])
