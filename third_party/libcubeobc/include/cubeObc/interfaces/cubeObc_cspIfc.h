/** @file cubeObc_cspIfc.h
 *
 * @brief CSP interface header file (libCubeObc <-> OBC CSP implementation)
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

#ifndef CUBEOBC_CSP_IFC__H
#define CUBEOBC_CSP_IFC__H

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
 * @brief Receive data over CSP
 *
 * @note This is a WEAK definition and must be implemented by the OBC.
 *
 * @note Use of this function requires that a connection-less socket has been created
 * that is bound to the port being passed to this function. This socket does not exist in libcubeobc.
 * The port used should be the source port used for communication with the CubeProduct (set in the endpoint).
 * The CubeProduct will respond to TCTLM on this source port (destination port from the CubeProducts perspective).
 *
 * @param[in]	port		The port of the socket bound for comms.
 * @param[out]	data		Data received buffer.
 * @param[out]	dataSize	Size of the received data.
 * @param[in]	timeout		Timeout on data reception
 *
 * @return CUBEOBC_ERROR_OK if no error.
 */
ErrorCode cubeObc_cspIfc_recvFrom(U8 port, U8 *data, U16 *dataSize, U32 timeout);

/**
 * @brief Transmit data over CSP
 *
 * @note This is a WEAK definition and must be implemented by the OBC.
 *
 * @note The CubeComputer requires a 1 millisecond delay between CAN packets,
 * This delay cannot be implemented within this library, since CAN packet transmission is handled in the CSP callback "tx_func",
 * the implementation of which is OBC specific.
 *
 * @param[in]	dst			Destination address.
 * @param[in]	dstPort		Destination port.
 * @param[in]	srcPort		Source port.
 * @param[in]	data		Data transmit buffer.
 * @param[in]	dataSize	Size of the data to transmit.
 * @param[in]	timeout		Timeout on data transmission.
 *
 * @return CUBEOBC_ERROR_OK if no error.
 */
ErrorCode cubeObc_cspIfc_sendTo(U8 dst, U8 dstPort, U8 srcPort, U8 *data, U16 dataSize, U32 timeout);

#ifdef __cplusplus
}
#endif
#endif /* CUBEOBC_CSP_IFC__H */
