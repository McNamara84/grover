"""Run the native Rust tests and refresh their coverage reports and README badges."""

import json
import os
from pathlib import Path
import subprocess


def write_badge(metric, counts):
    label = f"Rust {metric} (SSR)"
    percentage = f"{counts['percent']:.1f}%" if counts["count"] else "n/a"
    title = f"{label}: {percentage} ({counts['covered']} of {counts['count']})"
    svg = f'''<svg xmlns="http://www.w3.org/2000/svg" width="172" height="20" role="img" aria-label="{title}">
  <title>{title}</title>
  <rect width="120" height="20" fill="#48565b"/>
  <rect x="120" width="52" height="20" fill="#216158"/>
  <g fill="white" text-anchor="middle" font-family="Verdana, sans-serif" font-size="11">
    <text x="60" y="14">{label}</text><text x="146" y="14">{percentage}</text>
  </g>
</svg>
'''
    Path(f"coverage/rust-{metric}.svg").write_bytes(svg.encode("utf-8"))
    print(title)


def main():
    report = Path("coverage/report")
    report.mkdir(parents=True, exist_ok=True)
    summary = report / "summary.json"
    exclude_tests = ["--ignore-filename-regex", r"/tests(\.rs$|/)"]
    subprocess.run(
        ["cargo", "llvm-cov", "--locked", "--workspace", "--features", "ssr",
         "--json", "--summary-only", "--output-path", str(summary), *exclude_tests],
        check=True,
    )
    subprocess.run(
        ["cargo", "llvm-cov", "report", "--html", "--output-dir", str(report), *exclude_tests],
        check=True,
    )
    # LLVM creates directories with mode 0750. Give the report to the owner of the bind mount.
    owner = report.parent.stat()
    if os.name == "posix" and os.geteuid() == 0 and owner.st_uid != 0:
        subprocess.run(["chown", "-R", f"{owner.st_uid}:{owner.st_gid}", report], check=True)

    totals = json.loads(summary.read_text(encoding="utf-8"))["data"][0]["totals"]
    for metric in ("lines", "functions"):
        write_badge(metric, totals[metric])


if __name__ == "__main__":
    main()
