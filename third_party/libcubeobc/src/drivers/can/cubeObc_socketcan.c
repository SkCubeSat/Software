/** @file cubeObc_socketcan.c
 *
 * @brief libCubeObc Driver for socketcan
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */
#ifdef SOCKETCAN

/***************************** SYSTEM INCLUDES *******************************/

#include <sys/socket.h>
#include <libsocketcan.h>
#include <net/if.h>
#include <linux/can/raw.h>
#include <sys/ioctl.h>
#include <unistd.h>
#include <errno.h>
#include <stdio.h>

/***************************** MODULE INCLUDES *******************************/

#include <cubeObc/drivers/cubeObc_can.h>
#include <cubeObc/arch/cubeObc_time.h>

/***************************** MODULE DEFINES ********************************/

/****************************** MODULE MACROS ********************************/

/***************************** MODULE TYPEDEFS *******************************/

/***************************** MODULE VARIABLES ******************************/

PRIVATE S32 canSocket = -1;

/***************************** MODULE FUNCTIONS ******************************/

/***************************** GLOBAL FUNCTIONS ******************************/

ErrorCode cubeObc_can_init(Text *device)
{
	can_do_stop(device);
	can_set_bitrate(device, 1000000u);
	can_set_restart_ms(device, 100u);
	can_do_start(device);

	canSocket = socket(PF_CAN, SOCK_RAW, CAN_RAW);

	struct timeval tv;

	tv.tv_sec = 0;
	tv.tv_usec = 10000;
	setsockopt(canSocket, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof(tv));

	/* Locate interface */
	struct ifreq ifr;

	strncpy(ifr.ifr_name, device, IFNAMSIZ - 1);
	if (ioctl(canSocket, SIOCGIFINDEX, &ifr) < 0)
	{
		return CUBEOBC_ERROR_EXIST;
	}

	struct sockaddr_can addr;

	MEMSET(&addr, 0, sizeof(addr));
	/* Bind the socket to CAN interface */
	addr.can_family = AF_CAN;
	addr.can_ifindex = ifr.ifr_ifindex;
	if (bind(canSocket, (struct sockaddr *)&addr, sizeof(addr)) < 0)
	{
		return CUBEOBC_ERROR_EXIST;
	}

	return CUBEOBC_ERROR_OK;
}

ErrorCode cubeObc_can_rx(CanPacket *packet)
{
	struct can_frame frame;

	int nbytes = read(canSocket, &frame, sizeof(frame));

	if (nbytes < 0)
	{
		return CUBEOBC_ERROR_READ;
	}

	if (nbytes != sizeof(frame))
	{
		return CUBEOBC_ERROR_SIZE;
	}

	/* Drop frames with standard id (CubeSpace uses extended) */
	if (!(frame.can_id & CAN_EFF_FLAG))
	{
		return CUBEOBC_ERROR_CAN_ID;
	}

	/* Drop error and remote frames */
	if (frame.can_id & (CAN_ERR_FLAG | CAN_RTR_FLAG))
	{
		return CUBEOBC_ERROR_CAN_ERR;
	}

	/* Strip flags */
	frame.can_id &= CAN_EFF_MASK;

	packet->canExtId = frame.can_id;
	packet->canSize = frame.can_dlc;
	MEMCPY(&packet->canData, frame.data, frame.can_dlc);

	return CUBEOBC_ERROR_OK;
}

ErrorCode cubeObc_can_tx(CONST CanPacket *packet)
{
	struct can_frame frame = {.can_id = packet->canExtId | CAN_EFF_FLAG,
							  .can_dlc = packet->canSize};

	MEMCPY(frame.data, &packet->canData, packet->canSize);

	uint32_t elapsed_ms = 0;

	while (write(canSocket, &frame, sizeof(frame)) != sizeof(frame))
	{
		if ((errno != ENOBUFS) || (elapsed_ms >= 1000))
		{
			return CUBEOBC_ERROR_WRITE;
		}

		cubeObc_time_delay(5u);

		elapsed_ms += 5;
	}

	return CUBEOBC_ERROR_OK;
}

#endif
/*** end of file ***/
