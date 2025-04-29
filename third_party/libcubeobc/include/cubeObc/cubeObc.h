/** @file cubeObc.h
 *
 * @brief Single header include for libCubeObc
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

#ifndef CUBEOBC__H
#define CUBEOBC__H

#ifdef __cplusplus
extern "C" {
#endif

/***************************** SYSTEM INCLUDES *******************************/

#include <cubeObc/cubeObc_typeDef.h>
#include <cubeObc/cubeObc_errorDef.h>
#include <cubeObc/cubeObc_tctlmDef.h>

/************************ DEPENDENT MODULE INCLUDES **************************/

#include <cubeObc/arch/cubeObc_time.h>
#include <cubeObc/drivers/cubeObc_can.h>
#include <cubeObc/drivers/cubeObc_uart.h>
#include <cubeObc/interfaces/cubeObc_canIfc.h>
#include <cubeObc/interfaces/cubeObc_cspIfc.h>
#include <cubeObc/interfaces/cubeObc_uartIfc.h>
#include <cubeObc/cubeObc_common.h>
#include <cubeObc/cubeObc_bootloader.h>
#include <cubeObc/cubeObc_cubeComputer.h>
#include <cubeObc/cubeObc_bulkDataTransfer.h>

/***************************** GLOBAL DEFINES ********************************/

/**************************** GLOBAL CONSTANTS *******************************/

/****************************** GLOBAL MACROS ********************************/

/***************************** GLOBAL TYPEDEFS *******************************/

/**
 * @brief CubeObcLib Configuration
 */
typedef struct CubeObc_ConfigStruct {
	U8 hostAddress;		/**< Host address (for CAN interface) */
} CubeObc_Config;

/**
 * @brief CubeObcLib Version structure
 */
typedef struct CubeObc__VersionStruct {
	U8 vMajor;			/**< Major version */
	U8 vMinor;			/**< Minor version */
	U16 vPatch;			/**< Patch version */
} CubeObc__Version;

/***************************** GLOBAL VARIABLES ******************************/

/***************************** GLOBAL FUNCTIONS ******************************/

/**
 * @brief CubeObcLib initializer
 *
 * @param[in]	config	CubeObcLib configuration
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_init(CubeObc_Config *config);

/**
 * @brief Get CubeObcLib version
 *
 * @param[out]	version	CubeObcLib version
 */
void cubeObc_getVersion(CubeObc__Version *version);

/**
 * @brief Get system version for this CubeObcLib
 *
 * @note The system version determines the specific implementation as it relates to the API definition
 *
 * @param[out]	version	System version
 */
void cubeObc_getSystemVersion(CubeObc__Version *version);

/**
 * @brief Get host address
 *
 * @return Host address
 */
U8 cubeObc_getHostAddress(void);

#ifdef __cplusplus
}
#endif
#endif /* CUBEOBC__H */
