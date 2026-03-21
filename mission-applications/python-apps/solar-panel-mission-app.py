#!/usr/bin/env python3

import argparse
from kubos_app import Services, logging_setup 
import sys

#set each GPIO high (Simultaneously)
#Wait for 10 s
#Potentially verify deployment
#Set low

logger = logging_setup("")

#set GPIO 5 and GPIO 6 to high
write("GPIO_68",1)
write("GPIO_69",1)
logger.info("set GPIO 5 & 6 to high")
#wait 10 seconds
time.sleep(10)
#set GPIO 5 and GPIO 6 to low
write("GPIO_68",0)
write("GPIO_69",0)
logger.info("Set GPIO 5 & 6 to low")

