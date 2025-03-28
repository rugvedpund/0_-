CONFIG_PATH=$HOME/.config/zxc

echo "Checking Certificates"

# Check if ceritificates and private key already exist
if [ -f $CONFIG_PATH/zxca.crt ] && [ -f $CONFIG_PATH/private.key ]
then
        echo "Certificates already exists"
        exit 0
else
        echo "Generating new pair"
        openssl genrsa -out $CONFIG_PATH/private.key 2048
        openssl req -x509 -new -nodes -key $CONFIG_PATH/private.key -sha256 -days 1024 -out $CONFIG_PATH/zxca.crt -extensions v3_req -config ./mkscripts/CA.cnf
fi
