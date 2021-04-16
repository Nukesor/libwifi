# Example for a Full Parser

The following will use `wlan0` for the interface name.
Please replace this with your respective interface.

1. Set your device to monitor mode. For me this looks something like this:
    ```
    sudo ip link set wlan0 down
    sudo iwconfig wlan0 mode monitor
    sudo ip link set wlan0 up
    ```
    The commands might differ for your distribution/OS.
2. Run `sudo cargo run -- wlan0`
3. You should get some output.


If you don't get any output, you might have to switch the channel of your wifi card, since you can only listen to one channel at a time.

Choose a channel of which you know that it's used by a router in your vicinity.
