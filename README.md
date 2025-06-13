# lix-diff


![example](assets/2025-05-13-12-30-48.png)

## Description

This is a nix plugin built upon the [`lix`](https://lix.systems/) package manager. It
is intended that the experimental feature `lix-custom-sub-commands` which
provides access to the `lix` command which allows for custom sub-commands to be
used.

## Installation

Get it from nixpkgs:

```bash
nix profile install nixpkgs#lix-diff
```

Or get it from the flake:

```bash
nix profile install github:tgirlcloud/lix-diff
```

## Usage

The example below demonstrates the usage of the `lix diff` command.

```bash
lix diff /nix/var/nix/profiles/system-95-link/ /run/current-system
```


Without the experimental feature enabled, the command can be called via

```bash
lix-diff /nix/var/nix/profiles/system-95-link/ /run/current-system
```

