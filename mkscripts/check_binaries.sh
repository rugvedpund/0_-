BINARIES="getfattr openssl tmux vim"
for binary in $BINARIES
do
        if command -v $binary > /dev/null 2>&1
        then
                echo "$binary........installed"
        else
                echo "Error: $binary is not installed. Please install it before proceeding."
                exit 1
        fi
done
echo "All required binaries are present"
