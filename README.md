# lix-diff

## Description

This is a nix plugin built upon the [`lix`](https://lix.systems/) package manager. It
is intended that the experimental feature `lix-custom-sub-commands` which
provides access to the `lix` command which allows for custom sub-commands to be
used.

## Usage

The example below demonstrates the usage of the `lix diff` command.

```bash
lix diff /nix/var/nix/profiles/system-95-link/ /run/current-system
```


Without the experimental feature enabled, the command can be called via

```bash
lix-diff /nix/var/nix/profiles/system-95-link/ /run/current-system
```

