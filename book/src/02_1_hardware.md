# Checking the hardware

Connect the Espressif Rust Board to your computer. Verify a tiny red control LED lights up.

The device should also expose its UART serial port over USB:

**Windows**: a USB Serial Device (COM port) in the Device Manager under the Ports section

**Linux**: a USB device under `lsusb`. The device will have a VID (vendor ID) of `0x10c4` and a PID (product ID) of `0xea60` -- the `0x` prefix will be omitted in the output of `lsusb`:

``` console
$ lsusb | grep UART
Bus 001 Device 011: ID 10c4:ea60 Silicon Laboratories, Inc. CP2102N USB to UART Bridge Controller  Serial: a4c4193ceaa0eb119085d1acdf749906
```

The device will also show up in the `/dev` directory as a `tty.usbmodem` device:

``` console
$ ls /dev/tty.usbmodem*
/dev/tty.usbmodem0
```

(If you are using a ESP32-C3-DevKitC-02 the command is `$ ls /dev/ttyUSB*` )

**macOS**:

The device will show up as part of the USB tree in `system_profiler`:

```console

$ system_profiler SPUSBDataType | grep -A 11 "USB JTAG"

USB JTAG/serial debug unit:

  Product ID: 0x1001
  Vendor ID: 0x303a
  (...)
```
