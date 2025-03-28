FT_PLUGIN_PATH=$HOME/.vim/ftplugin
FT_CONFIG_FILES="his.vim req.vim wreq.vim"

if [ ! -d $FT_PLUGIN_PATH ]
then
        echo "Creating directory..........$FT_PLUGIN_PATH"
        mkdir -p $FT_PLUGIN_PATH
else
        echo "Directory.................$FT_PLUGIN_PATH exists"
fi

# Check if config files exist
for f in $FT_CONFIG_FILES
do
        if [ ! -f $FT_PLUGIN_PATH/$f ]
        then
                echo "Missing Config file........$f"
                cp config/example/ftplugin/$f $FT_PLUGIN_PATH
                echo "Copied default config file........$f"
        else
                echo "Config File exists........$f"
        fi
done
