PLUGIN_PATH=$HOME/.vim/plugin

# Check if plugin dir exists
if [ ! -d $PLUGIN_PATH ]
then
        echo "Creating directory..........$PLUGIN_PATH"
        mkdir -p $PLUGIN_PATH
else
        echo "Directory.................$PLUGIN_PATH exists"
fi

# Copy zxc.vim
if [ ! -f $PLUGIN_PATH/zxc.vim ]
then
        cp config/example/zxc.vim $PLUGIN_PATH
fi
