/** @file cubeObc_uart.h
 *
 * @brief UART driver for libcubeobc
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

#ifndef CUBEOBC_UART__H
#define CUBEOBC_UART__H

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
 * @brief UART driver initializer
 *
 * @param[in]	device	UART device
 * @param[in]	baud	Baud reate
 *
 * @return CUUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_uart_init(Text *device, U32 baud);

/**
 * @brief Receive UART data
 *
 * @param[out]	data		Buffer for received data
 * @param[in]	size		Size of data to receive
 * @param[out]	sizeRead	Number of bytes actually read
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_uart_rx(U8 *data, U32 size, U32 *sizeRead);

/**
 * @brief Transmit UART data
 *
 * @param[in]	data	Buffer of data to transmit
 * @param[in]	size	Size of data to transmit
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_uart_tx(CONST U8 *data, U32 size);

#ifdef __cplusplus
}
#endif
#endif /* CUBEOBC_UART__H */
