/** @file cubeObc_canIfc.h
 *
 * @brief CAN bus interface header file (libCubeObc <-> OBC hardware)
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

#ifndef CUBEOBC_CAN_IFC__H
#define CUBEOBC_CAN_IFC__H

#ifdef __cplusplus
extern "C" {
#endif

/***************************** SYSTEM INCLUDES *******************************/

#include <cubeObc/cubeObc_typeDef.h>

/************************ DEPENDENT MODULE INCLUDES **************************/

/***************************** GLOBAL DEFINES ********************************/

/**************************** GLOBAL CONSTANTS *******************************/

/****************************** GLOBAL MACROS ********************************/

/***************************** GLOBAL TYPEDEFS *******************************/

/***************************** GLOBAL VARIABLES ******************************/

/***************************** GLOBAL FUNCTIONS ******************************/

/**
 * @brief Flush the CAN receive buffer
 *
 * @note This is a WEAK definition and must be implemented by the OBC
 */
void cubeObc_canIfc_rxFlush(void);

/**
 * @brief Receive CAN packet
 *
 * @note This is a WEAK definition and must be implemented by the OBC.
 *
 * @note This function must be non-blocking. A timeout on data reception is implemented in cubeObc_tctlmCommsMasterSvc_sendReceive().
 *
 * @param[out]	packet		Received CAN packet.
 *
 * @return CUBEOBC_ERROR_READ if failure to read CAN packet (hardware or no data).
 *
 * @return CUBEOBC_ERROR_OK if no error.
 */
ErrorCode cubeObc_canIfc_rx(CanPacket *packet);

/**
 * @brief Transmit CAN packet
 *
 * @note This is a WEAK definition and must be implemented by the OBC.
 *
 * @note This function may be blocking. It is up to OBC implementation for arbitration/timeout handling for transmission.
 *
 * @param[out]	packet		CAN packet to transmit.
 *
 * @return CUBEOBC_ERROR_WRITE if failure to transmit the packet.
 *
 * @return CUBEOBC_ERROR_OK if no error.
 */
ErrorCode cubeObc_canIfc_tx(CONST CanPacket *packet);

#ifdef __cplusplus
}
#endif
#endif /* CUBEOBC_CAN_IFC__H */
