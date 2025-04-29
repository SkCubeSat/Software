/** @file cubeObc_errorDef.h
 *
 * @brief Error Definitions for libCubeObc
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

#ifndef CUBEOBC_ERROR_DEF__H
#define CUBEOBC_ERROR_DEF__H

#ifdef __cplusplus
extern "C" {
#endif

/***************************** SYSTEM INCLUDES *******************************/

#include "cubeObc_typeDef.h"

/************************ DEPENDENT MODULE INCLUDES **************************/

/***************************** GLOBAL DEFINES ********************************/

#define CUBEOBC_ERROR_OK						((S32)0)	/**< No error */

#define CUBEOBC_ERROR_NULLPTR					((S32)1)	/**< NULL pointer */
#define CUBEOBC_ERROR_SIZE						((S32)2)	/**< Size incorrect */
#define CUBEOBC_ERROR_SIZEL						((S32)3)	/**< Size too low */
#define CUBEOBC_ERROR_SIZEH						((S32)4)	/**< Size too high */
#define CUBEOBC_ERROR_OVRRUN					((S32)5)	/**< Overrun */
#define CUBEOBC_ERROR_PARAM						((S32)6)	/**< Parameter error (out of range) */
#define CUBEOBC_ERROR_TOUT						((S32)7)	/**< Timeout */
#define CUBEOBC_ERROR_NACK						((S32)8)	/**< TCTLM comms Nack */
#define CUBEOBC_ERROR_BUSY						((S32)9)	/**< Busy */
#define CUBEOBC_ERROR_FRAME						((S32)10)	/**< Frame */
#define CUBEOBC_ERROR_CRC						((S32)11)	/**< CRC */
#define CUBEOBC_ERROR_READ						((S32)12)	/**< Read */
#define CUBEOBC_ERROR_WRITE						((S32)13)	/**< Write */
#define CUBEOBC_ERROR_CAN_ID					((S32)14)	/**< CAN ID type error */
#define CUBEOBC_ERROR_CAN_ERR					((S32)15)	/**< CAN frame error */
#define CUBEOBC_ERROR_UKN_NACK					((S32)16)	/**< Unknown NACK */
#define CUBEOBC_ERROR_NODE_TYPE					((S32)17)	/**< Invalid node type */
#define CUBEOBC_ERROR_FTP						((S32)18)	/**< CubeSpace file upload internal error */
#define CUBEOBC_ERROR_USAGE						((S32)19)	/**< Usage error */
#define CUBEOBC_ERROR_AUTOD						((S32)20)	/**< Auto-Discovery error */
#define CUBEOBC_ERROR_IMG						((S32)21)	/**< Image error */
#define CUBEOBC_ERROR_EXIST						((S32)22)	/**< Does not exist error */
#define CUBEOBC_ERROR_USER_DATA					((S32)23)	/**< User data error */
#define CUBEOBC_ERROR_COMMIT					((S32)24)	/**< Commit error */
#define CUBEOBC_ERROR_TCTLM_PROTOCOL			((S32)25)	/**< TCTLM protocol error */
#define CUBEOBC_ERROR_UNKNOWN					((S32)26)	/**< General unexpected/unknown error */
#define CUBEOBC_ERROR_TLM_SIZE					((S32)27)	/**< Telemetry response size error */
#define CUBEOBC_ERROR_TCTLM_ID					((S32)28)	/**< TCTLM response ID does not match the request */

#define CUBEOBC_ERROR_TCTLM_INVALID_ID			((S32)50)	/**< TCTLM Nack - invalid ID */
#define CUBEOBC_ERROR_TCTLM_INVALID_LENGTH		((S32)51)	/**< TCTLM Nack - invalid length */
#define CUBEOBC_ERROR_TCTLM_INVALID_PARAM		((S32)52)	/**< TCTLM Nack - invalid parameter data */
#define CUBEOBC_ERROR_TCTLM_CRC					((S32)53)	/**< TCTLM Nack - CRC failed */
#define CUBEOBC_ERROR_TCTLM_NOT_IMPLEMENTED		((S32)54)	/**< TCTLM Nack - not implemented */
#define CUBEOBC_ERROR_TCTLM_BUSY				((S32)55)	/**< TCTLM Nack - busy */
#define CUBEOBC_ERROR_TCTLM_SEQUENCE			((S32)56)	/**< TCTLM Nack - sequence */
#define CUBEOBC_ERROR_TCTLM_INTERNAL			((S32)57)	/**< TCTLM Nack - internal */
#define CUBEOBC_ERROR_TCTLM_PASS_TOUT			((S32)58)	/**< TCTLM Nack - pass-through timeout */
#define CUBEOBC_ERROR_TCTLM_PASS_TARGET			((S32)59)	/**< TCTLM Nack - pass-through target */

#define CUBEOBC_ERROR_CSP_RECV_TIMEOUT			((S32)70)	/**< CSP - receive timeout */
#define CUBEOBC_ERROR_CSP_BUFFER_NONE			((S32)71)	/**< CSP - failed to acquire a buffer */

#define CUBEOBC_ERROR_TODO						((S32)65535)	/**< Not implemented / TODO */

/**************************** GLOBAL CONSTANTS *******************************/

/****************************** GLOBAL MACROS ********************************/

/***************************** GLOBAL TYPEDEFS *******************************/

/***************************** GLOBAL VARIABLES ******************************/

/***************************** GLOBAL FUNCTIONS ******************************/

#ifdef __cplusplus
}
#endif
#endif							// CUBEOBC_ERROR_DEF__H
/*** end of file ***/
