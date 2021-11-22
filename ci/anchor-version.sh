#
# This file maintains the anchor versions for use by CI.
#
# Obtain the environment variables without any automatic updating:
#   $ source ci/anchor-version.sh
#
# Obtain the environment variables and install update:
#   $ source ci/anchor-version.sh install

# Then to access the solana version:
#   $ echo "$anchor"
#

if [[ -n $ANCHOR_VERSION ]]; then
  anchor_version="$ANCHOR_VERSION"
else
  anchor_version=v0.18.2
fi

export anchor_version="$anchor_version"

if [[ -n $1 ]]; then
  case $1 in
  install)
    npm install -g yarn
    npm i -g @project-serum/anchor-cli
    anchor --version
    ;;
  *)
    echo "anchor-version.sh: Note: ignoring unknown argument: $1" >&2
    ;;
  esac
fi