#ifndef _TFTP_SERVER_H_
#define _TFTP_SERVER_H_

#include <netinet/in.h>
#include <stdio.h>

#define SERVER_IP "0.0.0.0"
#define SERVER_PORT 69
#define BUF_LEN 2048
#define FILENAME_MAXLEN 256
#define MAX_CONNS 16
#define BLK_SIZE 512

#define TFTP_OP_RRQ 1
#define TFTP_OP_WRQ 2
#define TFTP_OP_DATA 3
#define TFTP_OP_ACK 4
#define TFTP_OP_ERROR 5

#define NFILE 2

const char *file_name[NFILE] = {"kernel.bin", "sdcard.img"};
const char *file_path[NFILE] = {"../kernel/target/riscv64gc-unknown-none-elf/release/kernel.bin", "../fs-img-src/sdcard.img"};
struct tftp_connect
{
	in_addr_t raw_client_ip;
	in_port_t raw_client_port;
	int acked_blk;
	int available;
    int lst_blk_id;
    FILE *fp;
};

#endif