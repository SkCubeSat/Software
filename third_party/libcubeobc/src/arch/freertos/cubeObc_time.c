/** @file cubeObc_time.c
 *
 * @brief Time based functions for freertos arch
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

/***************************** SYSTEM INCLUDES *******************************/

#include <FreeRTOS.h>

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
	return (U32)(xTaskGetTickCount() * (1000 / configTICK_RATE_HZ));
}

void cubeObc_time_delay(U32 ms)
{
	vTaskDelay(ms / portTICK_RATE_MS);
}

/*** end of file ***/
