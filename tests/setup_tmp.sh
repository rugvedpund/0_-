#!/usr/bin/env bash

rm -rf /tmp/zxc_test
mkdir -p /tmp/zxc_test/history

for i in $(seq 1 10)
do
        # Create folder
        mkdir -p /tmp/zxc_test/history/$i

        # Create request
        echo "original_"$i > /tmp/zxc_test/history/$i/$i.req

        # Create Addon file
        mkdir /tmp/zxc_test/history/$i/addons

        # Create repeater dir
        for j in $(seq 1 $i)
        do
                mkdir /tmp/zxc_test/history/$i/r-$j
                echo "repeater_"$j > /tmp/zxc_test/history/$i/r-$j/rep.req

                mkdir -p /tmp/zxc_test/history/$i/websocket/r-ws-$j
                echo "websocket_"$j > /tmp/zxc_test/history/$i/websocket/$i.wreq
                echo "repeater_websocket_"$j > /tmp/zxc_test/history/$i/websocket/r-ws-$j/$i.wreq

                # create file q- in addons dir
                echo "sqlmap_"$j > /tmp/zxc_test/history/$i/addons/q-$j.req

                # create file z- in addons dir
                echo "ffuf_"$j > /tmp/zxc_test/history/$i/addons/z-$j.req
        done

done

# Create websocket
mkdir /tmp/zxc_test/history/1/websocket/
echo -n "test" > /tmp/zxc_test/history/1/websocket/1.wreq
