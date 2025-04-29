/** @file cubeObc_time.c
 *
 * @brief Time based functions for cmsis-rtos-v2 arch
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

/***************************** SYSTEM INCLUDES *******************************/

#include <cmsis_os2.h>

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
	return (U32)((osKernelGetTickCount() * 1000) / osKernelGetTickFreq());
}

void cubeObc_time_delay(U32 ms)
{
	osDelay((((ms) * osKernelGetTickFreq()) / 1000U));
}

/*** end of file ***/
