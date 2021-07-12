pub fn help() -> &'static str {
"Usage of inner command prompt:
	
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
		Looks for availability of updates for your Gopherweb installation.

	custom [uri] [path] OR custom [path]
		An equivalent of address bar in GUI browsers.
		-> Entering `custom` with one parameter makes Gopherweb do the same as `cd`,
		but to the directory directly specified by you in first parameter.
		-> Using `custom` with two parameters means \"move to domain given in first
		parameter and cd to directory given in second directory\".

		In fact, `custom` lets you omit default way of browsing Gopherspace around
		directories by direct accessing given path (or path and domain).
"
}

pub fn changelog() -> &'static str {
"Changelog of Gopherweb:
    
    Version 0.1.2 - 12 Jul 2021 11:00:38 CEST:
    [+] Now Gopherweb lets you download new version after `update`
        directly from its inner command prompt. You're supposed to
        create directory `C:\\Program Files\\Gopherweb` (on Windows)
        or `/var/cache/gopherweb` in order to use this function.
        Automatizing it is planned.

    Version 0.1.1 - 11 Jul 2021 20:02:13 CEST:
    [+] Fixing update scheme. Currently it only checks for new versions.
        Functionality of downloading an update through Internet Gopher
        will be introduced together with downloading binaries.
    [+] Fixing typo in README.md.
    [-] Removing formatting of sign which worked only on Linux terminals.
        I'll revert it when I learn how to use it on Windows.

    Version 0.1.0 - 11 Jul 2021 19:23:01 CEST:
    [+] Adding commands: pwd, revert, update, custom, show, ls.
    
    Version 0.0.5 - 11 Jul 2021:
    [?] First 'serious', but still in pre-alpha phase commit.
    Available stuff at this moment: visit domains and cd, but not show fikes.
"}

pub fn get_version() -> &'static str {
    "0.1.2"
}

pub fn get_date() -> &'static str {
    "12 Jul 2021 11:00:38 CEST"
}
