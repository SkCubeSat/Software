/** @file cubeObc_time.h
 *
 * @brief Time based functions for libCubeObc
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

#ifndef CUBEOBC_TIME__H
#define CUBEOBC_TIME__H

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
 * @brief Get millisecond time
 *
 * @note The value can be arbitrary as long as the value increments once every millisecond
 *
 * @return Current milliseconds
 */
U32 cubeObc_time_getMs(void);

/**
 * @brief Delay for milliseconds
 *
 * @param[in]	ms	Number of milliseconds to delay for
 */
void cubeObc_time_delay(U32 ms);

#ifdef __cplusplus
}
#endif
#endif /* CUBEOBC_TIME__H */
