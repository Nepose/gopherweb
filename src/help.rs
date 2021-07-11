pub fn help() -> &'static str {
"Usage of inner command prompt:
        home                Move directly to gopher.floodgap.com.
        visit [url]         Visit given domain and position on its root.
        cd [number]         Change directory from current place to the one marked by given number.
        show [nr]           Show a file with given number.
        help                Show this help prompt.
        changelog           Show information about your installed version of Gopherweb and changelog.
        update              Check for updates.
        bye                 Leave Gopherweb.
"
}

pub fn changelog() -> &'static str {
"Your current version of Gopherweb is 0.0.5.
Changelog of Gopherweb:"
}