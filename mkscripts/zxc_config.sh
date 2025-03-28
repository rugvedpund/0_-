CONFIG_PATH=$HOME/.config/zxc
CONFIG_FILES="config.toml alias tmux.conf"

# Check if config dir exists
if [ ! -d $CONFIG_PATH ]
then
        echo "Creating config directory........$CONFIG_PATH"
        mkdir -p $CONFIG_PATH
else
        echo "Config directory exists........$CONFIG_PATH"
fi

# Check if config files exist
for f in $CONFIG_FILES
do
        if [ ! -f $CONFIG_PATH/$f ]
        then
                echo "Missing Config file........$f"
                cp config/$f $CONFIG_PATH
                echo "Copied default config file........$f"
        else
                echo "Config File exists........$f"
        fi
done
