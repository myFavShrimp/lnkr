# lnkr 🪝

`lnkr` is a powerful tool that allows you to symlink files and directories based on a simple configuration file. Say goodbye to manually creating symbolic links and let `lnkr` handle it for you!

## Installation
To install lnkr, use the following command:

```bash
cargo install lnkr
```

## Configuration

lnkr uses a configuration file to define the symlinks. Here's an example configuration from a `lnkr.yaml`:

```yaml
links:
  - os: [linux]
    destination: ~/.config
    items:
      - name: helix
        path: ./helix
        force: true
```

In the above example, we have a symlink configuration for Linux. The destination specifies where the symlink should be created, and the items array contains the individual symlinks to be created. Each item has a `name` and a `path`. The optional `force` parameter can be set to true to overwrite existing files or directories.

lnkr will only create symlinks whose link groups' `os` key matches the current operating system. For example, if you have a link group with os `[linux, macos]` and your current operating system is Linux, lnkr will create the symlinks specified within that group.

You can specify the target operating system by using one or more of the following values in the os field:
- linux
- macos
- ios
- freebsd
- dragonfly
- netbsd
- openbsd
- solaris
- android
- windows

Symlinks will be created by the following logic:

```
<destination>/<name> -> <path>
```

## Usage

To symlink your files and directories using lnkr, follow these steps:

1. Open your terminal and navigate to the directory containing your `lnkr.yaml`.

2. Run the following command:

```bash
  lnkr
```

That's it! lnkr will symlink your files and directories, saving you valuable time and effort.
