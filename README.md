# Gopherweb

Simple yet fast Gopher browser. Currently in alpha phase, without some necessary functions.

## How to install

Either **use release** (recommended) or by using git and Rust toolchain:
```
git clone https://github.com/Nepose/gopherweb
cargo build
./target/debug/gopherweb (or gopherweb.exe on Windows)
```

After installing create a cache directory. It's supposed to be located:
- on Windows at `C:\Program Files\gopherweb` while
- on Linux at `/var/cache/gopherweb`.

I'm going to add Makefile and automatize creating cache directory soon.

## How to use

After launching binary you'll be moved into inner command prompt. The allowed keywords are:

	visit [URL host]
		Moves you directly to the root directory of given URL. The URL mustn't contain 
		`gopher://` protocol scheme. Example: `visit gopher.floodgap.com`

	home
		An alias for `visit gopher.floodgap.com`.

	cd [number]
		Changes directory to the one marked by number printed after doing either
		`visit` or `cd`. You can also use `cd ..` to move to previous directory.
    
	show [number]
        	Shows a file marked by number. Works in the same way as `cd`, but for obvious reason there is no `show ..`.

	ls
		Displays content of the directory you're currently into (its Gophermap).

	bye
		Ends Gopherweb session.

	help
		Shows contents of this section.

	changelog
		Shows basic changelog.
	
	pwd
		Shows history of your journey through Gopherspace in current session.

	revert [number]
		Moves you back to particular position from `pwd`.

	update
		Looks for availability of updates for your Gopherweb.

	custom [uri] [path] OR custom [path]
		An equivalent of address bar in GUI browsers.
		-> Entering `custom` with one parameter makes Gopherweb do the same as `cd`,
		but to the directory directly specified by you in first parameter.
		-> Using `custom` with two parameters means "move to domain given in first
		parameter and cd to directory given in second directory".

		In fact, `custom` lets you omit default way of browsing Gopherspace around
		directories by direct accessing given path (or path and domain).
