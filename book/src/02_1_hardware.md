# Checking the Hardware

Connect the Espressif Rust Board to your computer. Verify, a tiny red control LED lights up.

The device should also expose its UART serial port over USB:

**Windows**: a USB Serial Device (COM port) in the Device Manager under the Ports section.

**Linux**: a USB device under `lsusb`.
The device will have a VID (Vendor ID) of `303a` and a PID (Product ID) of `1001` -- the `0x` prefix will be omitted in the output of `lsusb`:

``` console
$ lsusb | grep USB
Bus 006 Device 035: ID 303a:1001 Espressif USB JTAG/serial debug unit
```

Another way to see the device is to see which permissions and port is associated with the device is to check the `/by-id` folder:
``` console
$ ls -l /dev/serial/by-id
lrwxrwxrwx 1 root root .... usb-Espressif_USB_JTAG_serial_debug_unit_60:55:F9:C0:27:18-if00 -> ../../ttyACM0

```
> If you are using a ESP32-C3-DevKitC-02 the command is `$ ls /dev/ttyUSB*`

**macOS**: The device will show up as part of the USB tree in `system_profiler`:

```console
$ system_profiler SPUSBDataType | grep -A 11 "USB JTAG"

USB JTAG/serial debug unit:

  Product ID: 0x1001
  Vendor ID: 0x303a
  (...)
```

The device will also show up in the `/dev` directory as a `tty.usbmodem` device:

``` console
$ ls /dev/tty.usbmodem*
/dev/tty.usbmodem0
```
