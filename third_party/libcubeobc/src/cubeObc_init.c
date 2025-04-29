/** @file cubeObc_init.c
 *
 * @brief libCubeObc Initialization
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

/***************************** SYSTEM INCLUDES *******************************/

#include <cubeObc/cubeObc.h>

/***************************** MODULE INCLUDES *******************************/

#include <cubeObc/cubeObc_tctlmCommsMasterSvc.h>

/***************************** MODULE DEFINES ********************************/

/****************************** MODULE MACROS ********************************/

/***************************** MODULE TYPEDEFS *******************************/

/***************************** MODULE VARIABLES ******************************/

/**
 * @brief CubeObcLib configuration
 */
PRIVATE CubeObc_Config cubeObcConfig;

/***************************** MODULE FUNCTIONS ******************************/

/***************************** GLOBAL FUNCTIONS ******************************/

ErrorCode cubeObc_init(CubeObc_Config *config)
{
	ZERO_VAR(cubeObcConfig);

	MEMCPY((U8 *)&cubeObcConfig, (U8 *)config, sizeof(cubeObcConfig));

	cubeObc_tctlmCommsMasterSvc_init(config->hostAddress);

	return CUBEOBC_ERROR_OK;
}

void cubeObc_getVersion(CubeObc__Version *version)
{
	version->vMajor = (U8)VERSION_MAJOR;
	version->vMinor = (U8)VERSION_MINOR;
	version->vPatch = (U16)VERSION_PATCH;
}

void cubeObc_getSystemVersion(CubeObc__Version *version)
{
	version->vMajor = (U8)SYSTEM_VERSION_MAJOR;
	version->vMinor = (U8)SYSTEM_VERSION_MINOR;
	version->vPatch = (U16)SYSTEM_VERSION_PATCH;
}

U8 cubeObc_getHostAddress(void)
{
	return cubeObcConfig.hostAddress;
}

/*** end of file ***/
