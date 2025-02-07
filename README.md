# Sleek Manifest File Handler

The goal of this project is to provide a reliable file creation tool for use with nix

Mainly with [hjem](https://github.com/feel-co/hjem), and possibly for creating `/etc` in [NixOS](https://github.com/NixOS/nixpkgs)

### Example manifest

Note: any option set to `null` here is optional

```json
{
  "files": [
    {
      "type": "file",
      "source": "./sources/file",
      "target": "./outputs/file",
      "permissions": null,
      "uid": null,
      "gid": null,
      "clobber": null
    },
    {
      "type": "symlink",
      "source": "./sources/file",
      "target": "./outputs/symlink",
      "permissions": null,
      "uid": null,
      "gid": null,
      "clobber": null
    },
    {
      "type": "recursiveSymlink",
      "source": "./sources/recursiveSymlink",
      "target": "./outputs/recursiveSymlink",
      "clobber": null
    },
    {
      "type": "modify",
      "target": "./outputs/modified",
      "permissions": null,
      "uid": null,
      "gid": null
    },
    {
      "type": "directory",
      "target": "./outputs/directory",
      "permissions": null,
      "uid": null,
      "gid": null,
      "clobber": null
    },
    {
      "type": "delete",
      "target": "./outputs/delete"
    }
  ],
  "clobber_by_default": false,
  "version": 1
}

```

With the `sources` directory containing:
```bash
$ eza --long --no-user --no-time --no-filesize --tree -L 2 sources
drwxr-xr-x sources
.rw-r--r-- ├── file
drwxr-xr-x └── recursiveSymlink
drwxr-xr-x     ├── dir
lrwxrwxrwx     └── symlink -> ../file

```

And the `outputs` directory looking like this before hand:
```bash
$ eza --long --no-user --no-time --no-filesize --tree -L 2 outputs
drwxr-xr-x outputs
.rw-r--r-- ├── delete
.rw-r--r-- └── modified
```

This should output:
```bash
$ eza --long --no-user --no-time --no-filesize --tree -L 2 outputs
drwxr-xr-x outputs
drwxr-xr-x ├── directory
.rw-r--r-- ├── file
.rw-r--r-- ├── modified
drwxr-xr-x ├── recursiveSymlink
drwxr-xr-x │   ├── dir
lrwxrwxrwx │   └── symlink -> /absolute/path/sources/file
lrwxrwxrwx └── symlink -> /absolute/path/sources/file
```
