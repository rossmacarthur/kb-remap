#compdef kb-remap

autoload -U is-at-least

_kb-remap() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" : \
'*-s+[Swap two keys. Equivalent to two \`map\` options]:SRC:DST:_default' \
'*--swap=[Swap two keys. Equivalent to two \`map\` options]:SRC:DST:_default' \
'*-m+[A map of source key to destination key]:SRC:DST:_default' \
'*--map=[A map of source key to destination key]:SRC:DST:_default' \
'--name=[Select the first keyboard with this name]:NAME:_default' \
'--vendor-id=[Select the first keyboard with this vendor ID]:VENDOR-ID:_default' \
'--product-id=[Select the first keyboard with this product ID]:PRODUCT-ID:_default' \
'(--list --reset --dump -s --swap -m --map --name --vendor-id --product-id)--completions=[Generate completions for the specified shell]:SHELL:(bash elvish fish powershell zsh)' \
'(--reset --dump -s --swap -m --map)--list[List the available keyboards]' \
'(--list -s --swap -m --map)--reset[Reset the keyboard mapping]' \
'--dump[Dump the raw hidutil command that would be executed]' \
'-h[Print help]' \
'--help[Print help]' \
'-V[Print version]' \
'--version[Print version]' \
&& ret=0
}

(( $+functions[_kb-remap_commands] )) ||
_kb-remap_commands() {
    local commands; commands=()
    _describe -t commands 'kb-remap commands' commands "$@"
}

if [ "$funcstack[1]" = "_kb-remap" ]; then
    _kb-remap "$@"
else
    compdef _kb-remap kb-remap
fi
