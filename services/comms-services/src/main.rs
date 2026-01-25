use rust_i2c::*;

fn main() {
    println!("Hello, world!");
}

/*
// CSP and custom Needronix management ports used in sun sensor:
#define CSP_PORT_CMP 0
#define CSP_PORT_PING 1
#define CSP_PORT_REBOOT 4
#define CSP_PORT_RADIO_READY_CHECK 5 // equivalent of csp_get_buf_free
#define CSP_PORT_GET_UPTIME 6
#define CSP_PORT_MORSE 11
#define CSP_PORT_MORSE_TEXT 12
#define CSP_PORT_AX25_BEACON 15
#define NMP_PORT 20
#define CSP_PORT_GET_SYSTEM_STATS 22
#define CSP_PORT_GET_INTF_STATS 23

#define CSP_ERR_NONE 0 /* No error */
#define CSP_ERR_NOMEM -1 /* Not enough memory */
#define CSP_ERR_INVAL -2 /* Invalid argument */
#define CSP_ERR_TIMEDOUT -3 /* Operation timed out */
#define CSP_ERR_USED -4 /* Resource already in use */
#define CSP_ERR_NOTSUP -5 /* Operation not supported */
#define CSP_ERR_BUSY -6 /* Device or resource busy */
#define CSP_ERR_ALREADY -7 /* Connection already in progress */
#define CSP_ERR_RESET -8 /* Connection reset */
#define CSP_ERR_NOBUFS -9 /* No more buffer space available */
#define CSP_ERR_TX -10 /* Transmission failed */
#define CSP_ERR_DRIVER -11 /* Error in driver layer */
#define CSP_ERR_AGAIN -12 /* Resource temporarily unavailable */
#define CSP_ERR_NOT_NMP -13 /* Length too short to be NMP protocol*/
#define CSP_ERR_LEN -16 /* Length related problem (too short or too long) - this is what a specific command finds and evaluates */
#define CSP_ERR_LOCKED -19 /* NMP critical command requires to unlock NMP first */
#define CSP_ERR_PERMISSION -29 /* NMP: unlocked, key is valid, but you are not authorized to execute this command */
#define CSP_ERR_PERMISSION_INTERNAL -31 /* no permission found - this will not happen in real operation, this is to capture programmers mistake at testing */
#define CSP_ERR_KEY -37 /* no valid key provided */
#define CSP_ERR_HMAC -100 /* HMAC failed */
#define CSP_ERR_XTEA -101 /* XTEA failed */
#define CSP_ERR_CRC32 -102 /* CRC32 failed */

*/