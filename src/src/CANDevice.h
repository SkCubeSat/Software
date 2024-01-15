#pragma once

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <net/if.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <sys/ioctl.h>
#include <linux/can.h>
#include <linux/can/raw.h>


class CANDevice 
{

    private:
        uint8_t address;
        struct sockaddr_can addr;
        struct can_frame frame;
        struct ifreq ifr;


    public:
        CANDevice(unsigned int can_id);
        int write_data(unsigned int[] data, size_t size);
        

}