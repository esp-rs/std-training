# Interrupts

You can find a skeleton code for this exercise in advanced/button-interrupt.
The goal of this exercise is to log a message upon pressing the `BOOT` button on the board. 



## Tasks

Configure the button (GPIO 9) with the following settings:
    - input mode
    - pull up
    - interrupt on positive edge

Create an mpsc channel

QUESTION what is tx

TODO 

## How to call the C functions


- extra: hold button <-> LED on (might be good to not use semaphore xqueue then)