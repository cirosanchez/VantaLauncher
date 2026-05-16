#!/usr/bin/env bash
set -euo pipefail

SERVICE="dev.cirosanchez.VantaLauncher"
APP_NAME_LOWER="vantalauncher"

log() {
  printf '[reset-auth] %s\n' "$*"
}

remove_file_if_exists() {
  local p="$1"
  if [ -f "$p" ]; then
    rm -f "$p"
    log "Removed: $p"
  else
    log "Not found: $p"
  fi
}

reset_linux() {
  local data_home="${XDG_DATA_HOME:-$HOME/.local/share}"
  local accounts_path="$data_home/$APP_NAME_LOWER/accounts.json"

  log "Linux detected"
  remove_file_if_exists "$accounts_path"

  if command -v secret-tool >/dev/null 2>&1; then
    mapfile -t usernames < <(secret-tool search service "$SERVICE" 2>/dev/null | sed -n 's/^attribute\.username = \(.*\)$/\1/p' | sort -u)

    if [ "${#usernames[@]}" -eq 0 ]; then
      log "No keyring entries found for service '$SERVICE'"
    else
      for user in "${usernames[@]}"; do
        secret-tool clear service "$SERVICE" username "$user" >/dev/null 2>&1 || true
        log "Removed keyring entry: service=$SERVICE username=$user"
      done
    fi
  else
    log "secret-tool not installed; skipped keyring cleanup"
  fi
}

reset_macos() {
  local accounts_path="$HOME/Library/Application Support/$SERVICE/accounts.json"
  local ids=()

  if [ -f "$accounts_path" ]; then
    mapfile -t ids < <(grep -oE '"id"\s*:\s*"[^"]+"' "$accounts_path" | sed -E 's/.*"([^"]+)"$/\1/' || true)
  fi

  log "macOS detected"
  remove_file_if_exists "$accounts_path"

  if command -v security >/dev/null 2>&1; then
    if [ "${#ids[@]}" -gt 0 ]; then
      for id in "${ids[@]}"; do
        security delete-generic-password -s "$SERVICE" -a "microsoft:$id" >/dev/null 2>&1 || true
        log "Attempted keychain delete: service=$SERVICE account=microsoft:$id"
      done
    fi

    # Fallback: try deleting by service once (removes one matching item per run if present)
    security delete-generic-password -s "$SERVICE" >/dev/null 2>&1 || true
    log "Attempted generic keychain delete for service '$SERVICE'"
  else
    log "security CLI not available; skipped keychain cleanup"
  fi
}

reset_windows() {
  local accounts_path="${LOCALAPPDATA:-}/cirosanchez/VantaLauncher/data/accounts.json"

  log "Windows-like environment detected"
  remove_file_if_exists "$accounts_path"

  if command -v powershell.exe >/dev/null 2>&1; then
    powershell.exe -NoProfile -Command "
      \$service = '$SERVICE'
      \$targets = cmdkey /list | Select-String -Pattern 'Target:\s*(.*)' | ForEach-Object { \$_.Matches[0].Groups[1].Value.Trim() } | Where-Object { \$_ -like \"*\$service*\" }
      foreach (\$t in \$targets) { cmdkey /delete:\$t | Out-Null }
    " >/dev/null 2>&1 || true
    log "Attempted Credential Manager cleanup for service '$SERVICE'"
  else
    log "powershell.exe not available; skipped Credential Manager cleanup"
  fi
}

confirm="${1:-}"
if [ "$confirm" != "--yes" ]; then
  echo "This will hard reset launcher auth state (accounts.json + keyring entries)."
  echo "Run with: $0 --yes"
  exit 1
fi

case "$(uname -s)" in
  Linux*) reset_linux ;;
  Darwin*) reset_macos ;;
  CYGWIN*|MINGW*|MSYS*) reset_windows ;;
  *)
    log "Unsupported OS: $(uname -s)"
    exit 1
    ;;
esac

log "Done."
