/** @file cubeObc_uartIfc.h
 *
 * @brief UART interface header file (libCubeObc <-> OBC hardware)
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

#ifndef CUBEOBC_UART_IFC__H
#define CUBEOBC_UART_IFC__H

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
 * @brief Flush the UART receive buffer
 *
 * @note This is a WEAK definition and must be implemented by the OBC
 */
void cubeObc_uartIfc_rxFlush(void);

/**
 * @brief Receive data from UART
 *
 * @note This is a WEAK definition and must be implemented by the OBC.
 *
 * @note This function must be non-blocking. A timeout on data reception is implemented in cubeObc_tctlmCommsMasterSvc_sendReceive().
 *
 * @param[out]	data		Buffer for received data.
 * @param[in]	size		Number of bytes to read.
 * @param[out]	sizeRead	Number of bytes that were actually read.
 *
 * @return CUBEOBC_ERROR_READ if failure to read from the UART.
 *
 * @return CUBEOBC_ERROR_OK if no error, even if size != sizeRead.
 */
ErrorCode cubeObc_uartIfc_rx(U8 *data, U32 size, U32 *sizeRead);

/**
 * @brief Transmit data on UART
 *
 * @note This is a WEAK definition and must be implemented by the OBC.
 *
 * @note This function may be blocking. It is up to OBC implementation for timeout handling for transmission.
 *
 * @param[out]	data		Buffer containing data to transmit.
 * @param[in]	size		Number of bytes to transmit.
 *
 * @return CUBEOBC_ERROR_WRITE if failure to transmit the specified number of bytes.
 *
 * @return CUBEOBC_ERROR_OK if no error.
 */
ErrorCode cubeObc_uartIfc_tx(CONST U8 *data, U32 size);

#ifdef __cplusplus
}
#endif
#endif /* CUBEOBC_UART_IFC__H */
