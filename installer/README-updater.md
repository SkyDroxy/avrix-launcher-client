# Avrix Updater (Inno Setup)

This folder contains `updater.iss`, a tiny Inno Setup script that updates only the Avrix launcher executable in place.

Key points:

- Uses the same AppId as the main installer to stay consistent.
- Reads the install directory from the registry key written by `setup.iss`:
  - HKCU\Software\Avrix\Avrix Launcher (default value)
- Copies `payload\\build\\AvrixLauncher.exe` into that directory.

## Building locally

1. Install Inno Setup 6 (adds `ISCC.exe`).
2. Put the new `AvrixLauncher.exe` into `installer/payload/build/`.
3. Optionally set NEW_VERSION (used for output name):
   - Windows PowerShell: `$env:NEW_VERSION = '0.2.7'`
4. Compile:
   - `"C:\\Program Files (x86)\\Inno Setup 6\\ISCC.exe" updater.iss`

The output will be `Avrix-Update-<version>.exe` in `installer/Output/`.

## GitHub Actions workflow

The workflow `.github/workflows/build-updater.yml` can:

- Build or download `AvrixLauncher.exe`
- Compile `updater.iss`
- Optionally sign the updater EXE using Tauri signer
- Optionally upload to S3

Inputs when using "Run workflow":

- `version` (required): e.g. `0.2.7`
- `exeUrl` (optional): if provided, the workflow downloads the EXE
- `buildApp` (optional): set to true to build the app with Tauri instead of downloading

### Required secrets

- `TAURI_PRIVATE_KEY` and `TAURI_KEY_PASSWORD` (if you want a `.sig` via `tauri signer`)
- MinIO (S3 compatible) upload:
  - `MINIO_ENDPOINT`, `MINIO_ACCESS_KEY`, `MINIO_SECRET_KEY`

> Note: Local linters might complain about `${{ secrets.* }}` expressions in the workflow; they are valid at runtime on GitHub.

### MinIO paths

The workflow uploads to `s3/avrix-launcher/v<version>/Avrix-Update-<version>.exe` (and `.exe.sig`) on your MinIO server, matching the release layout.

## Behavior

- If the registry key is missing, the updater aborts with an error (it will not prompt for a path).
- The updater is meant to be launched silently by your app once the `.sig` has been verified client-side.

## Tauri signature

We sign the produced updater executable with Tauri's signer. The launcher can verify it with the embedded Tauri public key before executing the updater.
