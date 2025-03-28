msg="Build Vim| https://github.com/vim/vim/blob/master/src/INSTALL"

VIM_FEATURES="+channel +terminal +timers"
for feature in $VIM_FEATURES
do
        if vim --version | grep -q $feature
        then
                echo "vim $feature feature........found"
        else
                echo "vim $feature feature........not found"
                echo $msg
                exit 1
        fi
done

# Check whether unix socket feature present
# Ubuntu uses vim 8.x version
result=$(vim --clean -e --cmd 'echo has("patch-8.2.4684")' +qall 2>&1 1>/dev/null)

if echo $result | grep 0 >/dev/null
then
        echo "vim unix socket feature........not found"
        echo $msg
        exit 1
fi
