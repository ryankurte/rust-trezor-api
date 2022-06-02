initSidebarItems({"enum":[["Model","The different kind of Trezor device models."]],"fn":[["find_devices","Search for all available devices. Most devices will show up twice both either debugging enables or disabled."],["find_hid_devices","Search for old HID devices. This should only be used for older devices that don’t have the firmware updated to version 1.7.0 yet. Trying to connect to a post-1.7.0 device will fail."],["unique","Try to get a single device.  Optionally specify whether debug should be enabled or not. Can error if there are multiple or no devices available. For more fine-grained device selection, use `find_devices()`. When using USB mode, the device will show up both with debug and without debug, so it’s necessary to specify the debug option in order to find a unique one."]],"mod":[["client",""],["error","Error Handling"]],"struct":[["AvailableDevice","A device found by the `find_devices()` method.  It can be connected to using the `connect()` method."]],"trait":[["TrezorMessage","This trait extends the protobuf Message trait to also have a constant for the message type code.  This getter is implemented in this file for all the messages we use."]]});