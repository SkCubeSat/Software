/** @file cubeObc_common.c
 *
 * @brief libCubeObc CubeProduct common operation helpers
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

/***************************** SYSTEM INCLUDES *******************************/

/***************************** MODULE INCLUDES *******************************/

#include <cubeObc/cubeObc_common.h>

/***************************** MODULE DEFINES ********************************/

/****************************** MODULE MACROS ********************************/

/***************************** MODULE TYPEDEFS *******************************/

/***************************** MODULE VARIABLES ******************************/

/***************************** MODULE FUNCTIONS ******************************/

/***************************** GLOBAL FUNCTIONS ******************************/

ErrorCode cubeObc_common_pollForBootState(TypeDef_TctlmEndpoint *endpoint,
										  TctlmCommonFramework1_BootState state,
										  U32 backoff, U32 timeout,
										  Boolean notState)
{
	ErrorCode result;
	Boolean done = FALSE;
	U16 backoffTotal = 0u; // How long we have waited in total for initialization to complete

	do
	{
		TctlmCommonFramework1_BootStatus status;

		result = tctlmCommonFramework1_getBootStatus(endpoint, &status);

		if (result == CUBEOBC_ERROR_OK)
		{
			done = (status.state == state);

			if (notState == TRUE)
			{
				done = !done;
			}

			if (done == FALSE)
			{
				if (backoffTotal >= timeout)
				{
					result = CUBEOBC_ERROR_TOUT;
				}
				else
				{
					cubeObc_time_delay(backoff);

					backoffTotal += backoff; // Increment how many milliseconds we have waited
				}
			}
		}
	}
	while ((result == CUBEOBC_ERROR_OK) && (done == FALSE));

	return result;
}

/*** end of file ***/
