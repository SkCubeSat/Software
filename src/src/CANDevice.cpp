#include "CANDevice.h";

CANDevice::CANDevice (unsigned int can_id)
{
    addr.can_family = AF_CAN;
    addr.can_ifindex = ifr.ifr_ifindex;
    if((socket = socket(PF_CAN, SOCK_RAW, CAN_RAW)) < 0) 
    {
        perror("Error while opening socket");
        return -1;
    }

    ioctl(socket, SIOCGIFINDEX, &ifr);
    if(bind(socket, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        perror("Error in socket bind");
        return -2;
    }

    frame.can_id = can_id;

} 

CANDevice::write_data (unsigned int[] data, size_t size)
{
    frame.data = data;
    write(s, &frame, sizeof(struct can_frame));
    return 0;
}