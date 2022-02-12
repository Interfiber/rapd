#!/usr/bin/env bash

if [[ "$1" == "--build-release" ]]; then
  echo "## Building release package ##"
  rm -rf target
  cargo build --release
  echo "## Built binarys to ./target ##"
fi
if [[ "$1" == "--install-release" ]]; then
  echo "## Installing release build ##"
  if [ ! -f ./target/release/rapd ]; then
    echo "## Error: rapd release build not found ##"
    exit 1
  fi
  if [ ! -f ./target/release/rapc ]; then
    echo "## Error: rapc release build not found ##"
    exit 1
  fi
  if [ -z "${RAPD_INSTALL_ROOT}" ]; then
    export RAPD_INSTALL_ROOT="/"
  fi
  if [ ! -d "$RAPD_INSTALL_ROOT/usr" ]; then
    echo "## Creating $RAPD_INSTALL_ROOT/usr ##"
    sudo mkdir -p $RAPD_INSTALL_ROOT/usr
  fi
  if [ ! -d "$RAPD_INSTALL_ROOT/usr/bin" ]; then
    echo "## Creating $RAPD_INSTALL_ROOT/usr/bin ##"
    sudo mkdir -p $RAPD_INSTALL_ROOT/usr/bin
  fi
  echo "## Installing rapd ##"
  sudo mv ./target/release/rapd $RAPD_INSTALL_ROOT/usr/bin
  echo "## Installing rapc ##"
  sudo mv ./target/release/rapc $RAPD_INSTALL_ROOT/usr/bin
  echo "## Installed release build ##"
  exit 0
fi

if [[ "$1" == "" ]]; then
  echo "install.sh - install and build rapd"
  echo "   --install-release    Install release build"
  echo "   --build-release      Build a release build"
  exit 0
fi
