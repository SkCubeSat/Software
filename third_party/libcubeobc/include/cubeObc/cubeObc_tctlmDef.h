/** @file cubeObc_tctlmDef.h
 *
 * @brief TCTLM Related Definitions for libCubeObc
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

#ifndef CUBEOBC_TCTLM_DEF__H
#define CUBEOBC_TCTLM_DEF__H

#ifdef __cplusplus
extern "C" {
#endif

/***************************** SYSTEM INCLUDES *******************************/

#include "cubeObc_typeDef.h"
#include "cubeObc_errorDef.h"

/************************ DEPENDENT MODULE INCLUDES **************************/

/***************************** GLOBAL DEFINES ********************************/

/**************************** GLOBAL CONSTANTS *******************************/

/****************************** GLOBAL MACROS ********************************/

/***************************** GLOBAL TYPEDEFS *******************************/

/**
 * @brief Comms Error (Nack) Definitions
 */
typedef enum Tctlm_ErrorEnum {
	TCTLM__ERROR_OK = 0,				/*!< All Good */
	TCTLM__ERROR_INVALID_ID = 1,		/*!< Invalid TCTLM ID */
	TCTLM__ERROR_INVALID_LENGTH = 2,	/*!< Invalid Parameter Length */
	TCTLM__ERROR_INVALID_PARAM = 3,		/*!< Invalid Parameter Data */
	TCTLM__ERROR_CRC = 4,				/*!< CRC failed */
	TCTLM__ERROR_NOT_IMPLEMENTED = 5,	/*!< Request not supported for this firmware */
	TCTLM__ERROR_BUSY = 6,				/*!< Firmware cannot accept another command right now */
	TCTLM__ERROR_SEQUENCE = 7,			/*!< Command not possible in current firmware state */
	TCTLM__ERROR_INTERNAL = 8,			/*!< Internal Request Failure */
	TCTLM__ERROR_PASS_TIMEOUT = 9,		/*!< Pass through Request Timeout */
	TCTLM__ERROR_PASS_TARGET = 10,		/*!< Pass through target is invalid (pass through is disabled) */
} Tctlm_Error;

/***************************** GLOBAL VARIABLES ******************************/

/***************************** GLOBAL FUNCTIONS ******************************/

#ifdef __cplusplus
}
#endif
#endif							// CUBEOBC_TCTLM_DEF__H
/*** end of file ***/
