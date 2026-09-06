"""Check published artifact identities and onboarding links without network access."""
import hashlib
from pathlib import Path, PurePosixPath
import re
from urllib.parse import unquote, urlsplit
import zipfile

ROOT = Path(__file__).resolve().parents[1]
checked = 0

def verify_manifest(data, read):
    global checked
    for line in data.splitlines():
        if not line.strip():
            continue
        digest, name = line.split(None, 1)
        name = name.lstrip("*")
        assert re.fullmatch(r"[0-9a-f]{64}", digest), line
        assert hashlib.sha256(read(name)).hexdigest() == digest, name
        checked += 1

for manifest in sorted(ROOT.glob("research/**/SOURCE-SHA256SUMS")) + sorted(ROOT.glob("research/evidence/**/SHA256SUMS")):
    verify_manifest(manifest.read_text(), lambda name, base=manifest.parent: (base / name).read_bytes())
for sidecar in sorted(ROOT.glob("research/downloads/*.zip.sha256")):
    verify_manifest(sidecar.read_text(), lambda name, base=sidecar.parent: (base / Path(name).name).read_bytes())
for path in sorted(ROOT.glob("research/downloads/*.zip")):
    with zipfile.ZipFile(path) as archive:
        assert archive.testzip() is None, path
        for name in archive.namelist():
            if PurePosixPath(name).name == "SHA256SUMS":
                base = PurePosixPath(name).parent
                verify_manifest(archive.read(name).decode(), lambda target, base=base: archive.read(str(base / target)))

pages = list(ROOT.glob("*.md")) + list((ROOT / "research").glob("*.md")) + list((ROOT / "research/articles").glob("*.md")) + list((ROOT / "research/code").glob("*/README.md")) + list((ROOT / "research/evidence").glob("README.md")) + list((ROOT / "research/downloads").glob("README.md"))
links = 0
for page in pages:
    content = re.sub(r"```.*?```", "", page.read_text(), flags=re.S)
    for target in re.findall(r"!?\[[^\]]*\]\(([^\s)]+)(?:\s+[^)]*)?\)", content):
        url = urlsplit(target.strip("<>"))
        if url.scheme or url.netloc or not url.path:
            continue
        resolved = (page.parent / unquote(url.path)).resolve()
        assert resolved == ROOT or ROOT in resolved.parents, (page, target)
        assert resolved.exists(), (page.relative_to(ROOT), target)
        links += 1
print(f"Verified {checked} artifact hashes and {links} local documentation links.")
