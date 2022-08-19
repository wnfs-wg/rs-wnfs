#!/bin/bash
set -e

# PATHS
# Get current working directory
current_dir=`pwd`

# Get the absolute path where current script is running from.
script_path=$(readlink -f  $(which $0))

# Get the canonical directory of script.
if [[ -L $script_path ]]; then
    script_dir=$(dirname $(readlink -f $script_path))
else
    script_dir=$(dirname $script_path)
fi

# RETURN VARIABLE
ret=""

# ARGUMENTS
args="${@:2}" # All arguments except the first

# COLORS
red='\033[0;31m'
green='\033[0;32m'
purple='\033[0;35m'
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
        publish )
            publish
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
#	rs-wnfs build
#
help() {
    echo ""
    echo "Rust WNFS Utility Script"
    echo ""
    echo "USAGE:"
    echo "    rs-wnfs [COMMAND] [...args]"
    echo ""
    echo "COMMAND:"
    echo "   * build    [--fs|--wasm|--all]  - build projects"
    echo "   * test     [--fs|--wasm|--all]  - run tests"
    echo "   * publish  [--fs|--wasm|--all]  - publish packages"
    echo "   * coverage [--fs|--wasm|--all]  - show code coverage"
    echo "   * setup                           - install rs-wnfs script"
    echo "   * help                            - print this help message"
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
#	rs-wnfs build [--fs|--wasm|--all]
#
build() {
	if check_flag --fs; then
        build_fs
    elif check_flag --wasm; then
        build_wasm
    else
        build_fs
        build_wasm
    fi
}

build_fs() {
    display_header "💿 | BUILDING WNFS PROJECT | 💿"
    cargo build --release
}

build_wasm() {
    display_header "💿 | BUILDING WASM-WNFS PROJECT | 💿"
    cd $script_dir/../crates/wasm
    wasm-pack build --target web
	sed -i.bak \
        -e 's/"name": "wasm-wnfs"/"name": "wnfs",\n  "type": "module"/g' \
        -e 's/"module": "wasm_wnfs\.js"/"module": "wasm_wnfs\.js",\n  "main": "wasm_wnfs\.js"/g' \
        pkg/package.json
	rm pkg/package.json.bak
    
    cd $script_dir/..
    pkg_dir="./crates/wasm/pkg"
    wasm-bindgen ./target/wasm32-unknown-unknown/release/wasm_wnfs.wasm \
        --out-dir=$pkg_dir \
        --target=web \
        --weak-refs
    wasm-opt -O "$pkg_dir/wasm_wnfs_bg.wasm" -o "$pkg_dir/wasm-opt.wasm"
    mv "$pkg_dir/wasm-opt.wasm" "$pkg_dir/wasm_wnfs_bg.wasm"
}

# DESCRIPTION:
#   Runs tests.
#
# USAGE:
#	rs-wnfs test [--fs|--wasm|--all]
#
test() {
	if check_flag --fs; then
        test_fs
    elif check_flag --wasm; then
        test_wasm
    else
        test_fs
        test_wasm
    fi
}

test_fs() {
    display_header "🧪 | RUNNING WNFS TESTS | 🧪"
    cargo test -p wnfs --release -- --nocapture
}

test_wasm() {
    display_header "🧪 | RUNNING WASM-WNFS TESTS | 🧪"
    cd $script_dir/../crates/wasm
    yarn playwright test
}

# DESCRIPTION:
#    Shows the code coverage of the project
#
# USAGE:
#	rs-wnfs coverage [--fs|--wasm|--all]
#
coverage() {
    errorln "coverage command not implemented yet"
    exit 1
}

# DESCRIPTION:
#    Publishes the project.
#
# USAGE:
#	rs-wnfs publish [--fs|--wasm|--all]
#
publish() {
    errorln "publish command not implemented yet"
    exit 1
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
#	Checks if the flag is present in the list of arguments
#
check_flag() {
    local found=1
    local key=$1

    # For every argument in the list of arguments
    for arg in $args; do
        # Check if any of the argument matches the key provided
        if [[ $arg = $key ]]; then
            found=0
            break
        fi
    done

    return $found
}

# DESCRIPTION:
#	Sets up the cript by making it excutable and available system wide
#
setup() {
    displayln "Make script executable"
    chmod u+x $script_path

    displayln "Drop a link to it in /usr/local/bin"
    if ln -s $script_path /usr/local/bin/rs-wnfs; then
        successln "Successfully installed"
    else
        local result=$?
        errorln "Failed to install"
        exit $result
    fi
}

# DESCRIPTION:
#	Prints a message.
#
displayln() {
    printf "\n::: $1 :::\n"
}

# DESCRIPTION:
#	Prints an error message.
#
errorln() {
    printf "\n${red}::: $1 :::${none}\n\n"
}

# DESCRIPTION:
#	Prints an success message.
#
successln() {
    printf "\n${green}::: $1 :::${none}\n\n"
}

# DESCRIPTION:
#	Prints a header message.
#
display_header() {
    printf "\n${purple}$1${none}\n\n"
}

main $@
