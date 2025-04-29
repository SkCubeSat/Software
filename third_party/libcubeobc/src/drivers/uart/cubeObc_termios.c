/** @file cubeObc_termios.c
 *
 * @brief libCubeObc Driver for termios UART
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */
#ifdef TERMIOS

/***************************** SYSTEM INCLUDES *******************************/

#include <termios.h>
#include <fcntl.h>
#include <unistd.h>
#include <errno.h>
#include <stdio.h>

/***************************** MODULE INCLUDES *******************************/

#include <cubeObc/drivers/cubeObc_uart.h>

/***************************** MODULE DEFINES ********************************/

/****************************** MODULE MACROS ********************************/

/***************************** MODULE TYPEDEFS *******************************/

/***************************** MODULE VARIABLES ******************************/

PRIVATE S32 ttyUart = -1;

/***************************** MODULE FUNCTIONS ******************************/

speed_t getSpeed(U32 baud)
{
	switch (baud)
	{
		case 9600u:
			return B9600;
		case 19200u:
			return B19200;
		case 38400u:
			return B38400;
		case 115200u:
			return B115200;

		case 921600u:
		default:
			return B921600;
	}
}

/***************************** GLOBAL FUNCTIONS ******************************/

ErrorCode cubeObc_uart_init(Text *device, U32 baud)
{
	ttyUart = open(device, O_RDWR | O_NOCTTY | O_SYNC);
	if (ttyUart < 0)
	{
		return CUBEOBC_ERROR_EXIST;
	}

	struct termios tty;

	if (tcgetattr(ttyUart, &tty) < 0)
	{
		return CUBEOBC_ERROR_EXIST;
	}

	speed_t speed = getSpeed(baud);

	cfsetospeed(&tty, speed);
	cfsetispeed(&tty, speed);

	cfmakeraw(&tty);

	// Make read non-blocking
	tty.c_cc[VMIN] = 0;
	tty.c_cc[VTIME] = 0;

	if (tcsetattr(ttyUart, TCSANOW, &tty) != 0)
	{
		return CUBEOBC_ERROR_EXIST;
	}

	return CUBEOBC_ERROR_OK;
}

ErrorCode cubeObc_uart_rx(U8 *data, U32 size, U32 *sizeRead)
{
	ErrorCode result = CUBEOBC_ERROR_OK;

	int readSize = read(ttyUart, data, size);

	if (readSize < 0)
	{
		result = CUBEOBC_ERROR_READ;
	}
	else
	{
		*sizeRead = (U32)readSize;

		if ((U32)readSize != size)
		{
			result = CUBEOBC_ERROR_SIZE;
		}
	}

	return result;
}

ErrorCode cubeObc_uart_tx(CONST U8 *data, U32 size)
{
	int written = write(ttyUart, data, size);

	if (written != (int)size)
	{
		return CUBEOBC_ERROR_WRITE;
	}
	tcdrain(ttyUart);    /* delay for output */

	return CUBEOBC_ERROR_OK;
}

#endif
/*** end of file ***/
