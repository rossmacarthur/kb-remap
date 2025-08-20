complete -c kb-remap -s s -l swap -d 'Swap two keys. Equivalent to two `map` options' -r
complete -c kb-remap -s m -l map -d 'A map of source key to destination key' -r
complete -c kb-remap -l name -d 'Select the first keyboard with this name' -r
complete -c kb-remap -l vendor-id -d 'Select the first keyboard with this vendor ID' -r
complete -c kb-remap -l product-id -d 'Select the first keyboard with this product ID' -r
complete -c kb-remap -l completions -d 'Generate completions for the specified shell' -r -f -a "bash\t''
elvish\t''
fish\t''
powershell\t''
zsh\t''"
complete -c kb-remap -l list -d 'List the available keyboards'
complete -c kb-remap -l reset -d 'Reset the keyboard mapping'
complete -c kb-remap -l dump -d 'Dump the raw hidutil command that would be executed'
complete -c kb-remap -s h -l help -d 'Print help'
complete -c kb-remap -s V -l version -d 'Print version'
