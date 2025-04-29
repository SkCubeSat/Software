/** @file cubeObc_tctlmCommsMasterSvc.h
 *
 * @brief libCubeObc TCTLM Master Comms Handler Header file
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

#ifndef CUBEOBC_TCTLM_COMMS_MASTER_SVC__H
#define CUBEOBC_TCTLM_COMMS_MASTER_SVC__H

#ifdef __cplusplus
extern "C" {
#endif

/***************************** SYSTEM INCLUDES *******************************/

#include <cubeObc/cubeObc.h>

/************************ DEPENDENT MODULE INCLUDES **************************/

/***************************** GLOBAL DEFINES ********************************/

/**************************** GLOBAL CONSTANTS *******************************/

/****************************** GLOBAL MACROS ********************************/

/***************************** GLOBAL TYPEDEFS *******************************/

/**
 * @brief Endpoint type and address
 */
typedef struct TctlmCommsMasterSvc_EndpointStruct {
	TypeDef_TctlmEndpoint endpoint;		/**< Generic endpoint */
	U8 id;								/**< TCTLM ID */
} TctlmCommsMasterSvc_Endpoint;

/***************************** GLOBAL VARIABLES ******************************/

/***************************** GLOBAL FUNCTIONS ******************************/

/**
 * @brief Master comms service initializer
 *
 * @param[in]	hostAddress	Host address (for CAN interface)
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_tctlmCommsMasterSvc_init(U8 hostAddress);

/**
 * @brief Return a buffer address
 *
 * @param[in] masterEndpoint Endpoint data used as handle
 */
uint8_t *cubeObc_tctlmCommsMasterSvc_buffer(CONST TctlmCommsMasterSvc_Endpoint *masterEndpoint);

/**
 * @brief Return the buffer size
 *
 * @param[in] masterEndpoint Endpoint data used as handle
 */
uint32_t cubeObc_tctlmCommsMasterSvc_bufferSize(CONST TctlmCommsMasterSvc_Endpoint *masterEndpoint);

/**
 * @brief Issue a send-receive request
 *
 * @details This sequence consists of send and receive phase. The buffer must already
 * be populated before making this call
 *
 * @param[in] masterEndpoint Endpoint data used as handle
 * @param[in,out] bufferSizeUsed Used part of the buffer
 *
 * @return ErrorCode CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_tctlmCommsMasterSvc_sendReceive(TctlmCommsMasterSvc_Endpoint *masterEndpoint, uint32_t *bufferSizeUsed);

/*****************************************************************************/

#ifdef __cplusplus
}
#endif
#endif /* CUBEOBC_TCTLM_COMMS_MASTER_SVC__H */
