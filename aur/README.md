# Publishing All4Laser to AUR

## Prerequisites

1. **AUR account**: Create one at https://aur.archlinux.org/register
2. **SSH key**: Upload your public SSH key to your AUR profile at https://aur.archlinux.org/account

## Step-by-step publishing

### 1. Clone the AUR package repo (first time only)

```bash
git clone ssh://aur@aur.archlinux.org/all4laser-git.git /tmp/all4laser-aur
```

> This will be empty the first time — that's expected.

### 2. Copy package files

```bash
cp aur/PKGBUILD /tmp/all4laser-aur/
cp aur/.SRCINFO /tmp/all4laser-aur/
```

### 3. Commit and push

```bash
cd /tmp/all4laser-aur
git add PKGBUILD .SRCINFO
git commit -m "Initial upload: all4laser-git 0.1.46"
git push
```

### 4. Verify

Visit https://aur.archlinux.org/packages/all4laser-git to confirm the package appears.

## Updating the AUR package

When you release a new version:

1. Update `PKGBUILD` if dependencies changed
2. Regenerate `.SRCINFO`:
   ```bash
   cd aur/
   makepkg --printsrcinfo > .SRCINFO
   ```
3. Copy both files to the AUR repo, commit, and push

> **Note**: For `-git` packages the `pkgver()` function auto-detects the version
> from git tags, so you only need to bump `pkgrel` if the PKGBUILD itself changes
> (not the upstream source).

## Testing locally before publishing

```bash
cd aur/
makepkg -si
```

This will clone the repo, build, run tests, and install the package.

## Users install with

```bash
# Using yay
yay -S all4laser-git

# Using paru
paru -S all4laser-git

# Manual
git clone https://aur.archlinux.org/all4laser-git.git
cd all4laser-git
makepkg -si
```
