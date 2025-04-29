/** @file cubeObc_time.c
 *
 * @brief Time based functions for posix arch
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

/***************************** SYSTEM INCLUDES *******************************/

#include <limits.h>
#include <unistd.h>
#include <time.h>
#include <errno.h>

/***************************** MODULE INCLUDES *******************************/

#include <cubeObc/arch/cubeObc_time.h>

/***************************** MODULE DEFINES ********************************/

/****************************** MODULE MACROS ********************************/

/***************************** MODULE TYPEDEFS *******************************/

/***************************** MODULE VARIABLES ******************************/

/***************************** MODULE FUNCTIONS ******************************/

/***************************** GLOBAL FUNCTIONS ******************************/

U32 cubeObc_time_getMs(void)
{
	struct timespec ts;

	if (clock_gettime(CLOCK_MONOTONIC, &ts) == 0)
	{
		return (U32)((ts.tv_sec * 1000) + (ts.tv_nsec / 1000000));
	}

	return 0;
}

void cubeObc_time_delay(U32 ms)
{
	struct timespec req, rem;

	req.tv_sec = (ms / 1000U);
	req.tv_nsec = ((ms % 1000U) * 1000000U);

	while ((nanosleep(&req, &rem) < 0) && (errno == EINTR))
	{
		req = rem;
	}
}

/*** end of file ***/
