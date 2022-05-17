#!/bin/bash

# PATHS
# Get current working directory
current_dir=`pwd`

# Get the absolute path of where script is running from
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd)"
script_path="$script_dir/wnfs.sh"

# RETURN VARIABLE
ret=""

# ARGUMENTS
args="${@:2}" # All arguments except the first

# COLORS
red='\033[0;31m'
green='\033[0;32m'
none='\033[0m'

# DESCRIPTION:
#	Where execution starts
#
main() {
    case $1 in
        build )
            build
        ;;
        test )
            test
        ;;
        coverage )
            coverage
        ;;
        setup )
            setup
        ;;
        *)
            help
        ;;
    esac

    exit 0
}

# DESCRIPTION:
#	Prints the help info.
#
# USAGE:
#	wnfs build
#
help() {
    echo ""
    echo "Rust WNFS Utility Script"
    echo ""
    echo "USAGE:"
    echo "    wnfs [COMMAND] [...args]"
    echo ""
    echo "COMMAND:"
    echo "   * build           - build project"
    echo "   * test            - run tests"
    echo "   * coverage        - show code coverage"
    echo "   * setup           - install wnfs script"
    echo "   * -h, help        - print this help message"
    echo ""
    echo ""
}

#-------------------------------------------------------------------------------
# Commands
#-------------------------------------------------------------------------------

# DESCRIPTION:
#	Builds the project.
#
# USAGE:
#	wnfs build
#
build() {
    display_header "💿 BUILDING WNFS PROJECT"
    cargo build --release

    display_header "💿 BUILDING WASM-WNFS PROJECT"
    cd crates/wasm && wasm-pack build --target web --release
}

# DESCRIPTION:
#   Runs tests.
#
# USAGE:
#	wnfs test
#
test() {
    display_header "🧪 RUNNING WNFS TESTS"
    cargo test -p wnfs --release -- --nocapture

    display_header "🧪 RUNNING WASM-WNFS TESTS"
    cd crates/wasm && yarn playwright test
}

# DESCRIPTION:
#    Shows the code coverage of the project
#
# USAGE:
#	wnfs coverage
#
coverage() {
    displayln "coverage command not implemented yet"
}

#------------------------------------------------------------------------------
# Helper functions
#------------------------------------------------------------------------------

# DESCRIPTION:
#	Gets the value following a flag
#
get_flag_value() {
    local found=false
    local key=$1
    local count=0

    # For every argument in the list of arguments
    for arg in $args; do
        count=$((count + 1))
        # Check if any of the argument matches the key provided
        if [[ $arg = $key ]]; then
            found=true
            break
        fi
    done

    local args=($args)
    local value=${args[count]}

    # Check if argument specified was found
    if [[ $found = true ]]; then

        # Check if there exists a word after the key
        # And that such word doesn't start with hyphen
        if [[ ! -z $value ]] && [[ $value != "-"* ]]; then
            ret=$value
        else
            ret=""
        fi

    else
        ret=""
    fi
}

# DESCRIPTION:
#	Sets up the cript by making it excutable and available system wide
#
setup() {
    display "Make script executable"
    chmod u+x $script_path

    display "Drop a link to it in /usr/local/bin"
    ln -s $script_path /usr/local/bin/wnfs
}

# DESCRIPTION:
#	A custom print function
#
display() {
    printf "::: $1 :::\n"
}

# DESCRIPTION:
#	A custom print function that starts its output with a newline
#
displayln() {
    printf "\n::: $1 :::\n"
}

# DESCRIPTION:
#	A custom print function for headers.
#
display_header() {
    printf "\n${green}$1${none}\n\n"
}

main $@
