# Gopherweb

Simple yet fast Gopher browser. Currently in alpha phase, without some necessary functions.

## How to install

Either **use release** (recommended) or by using git and Rust toolchain:
```
git clone https://github.com/Nepose/gopherweb
cargo build
./target/debug/gopherweb
```

I'm going to add Makefile soon.

## How to use

After launching binary you'll be moved into inner command prompt. The allowed keywords are:

	visit [URL host]
		Moves you directly to the root of given URL. The URL mustn't contain 
		`gopher://` protocol scheme. Example: `visit gopher.floodgap.com`

	home
		An alias for `visit gopher.floodgap.com`.

	cd [number]
		Changes directory to the one marked by number printed after doing either
		`visit` or `cd`.

	bye
		Ends Gopherweb session.

	help
		Shows contents of this section.

	changelog
		Shows basic changelog.

The commands I'm going to add in next commits are:

	history
		Shows history of your visits.

	show [number]
		Shows content of file marked by number printed after doing either `visit`
		or `cd`.

	update
		Checks for updates of Gopherweb.
