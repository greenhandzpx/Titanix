#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <sys/socket.h>
#include <sys/types.h>
#include "server.h"

int socket_fd;
struct tftp_connect connect_list[MAX_CONNS];


int udp_out(in_addr_t raw_client_ip, in_port_t raw_client_port, uint8_t *buf, int len)
{
	struct sockaddr_in client_addr;
	client_addr.sin_family = AF_INET;
	client_addr.sin_port = raw_client_port;
	client_addr.sin_addr.s_addr = raw_client_ip;
	int ret = sendto(socket_fd, buf, len, 0, (struct sockaddr *)&client_addr, sizeof(client_addr));
	fflush(stdout);
	return ret;
}

int tftp_send_error(in_addr_t raw_client_ip, in_port_t raw_client_port, int error_code, char *error_msg)
{
	uint8_t buf[BUF_LEN];
	uint8_t *p = buf;
	int msg_len = strlen(error_msg);
	if (BUF_LEN - sizeof(unsigned short) * 2 < msg_len + 1) return -1;
	*(unsigned short *)p = htons(TFTP_OP_ERROR);
	p += sizeof(unsigned short);
	*(unsigned short *)p = htons(error_code);
	p += sizeof(unsigned short);
	memcpy(p, error_msg, msg_len + 1);
	udp_out(raw_client_ip, raw_client_port, buf, sizeof(unsigned short) * 2 + msg_len + 1);
	return 0;
}

int tftp_send_data(in_addr_t raw_client_ip, in_port_t raw_client_port, FILE *fp, int blkid, int blksz)
{
	if (BUF_LEN - sizeof(unsigned short) * 2 < blksz) return -1;
	uint8_t buf[BUF_LEN];
	fseek(fp, (blkid - 1) * blksz, SEEK_SET);
	uint8_t *p = buf;
	*(unsigned short *)p = htons(TFTP_OP_DATA);
	p += sizeof(unsigned short);
	*(unsigned short *)p = htons(blkid & 0xFFFF);
	p += sizeof(unsigned short);
	int readn = fread(p, 1, blksz, fp);
	udp_out(raw_client_ip, raw_client_port, buf, sizeof(unsigned short) * 2 + readn);
	if (readn < blksz)
	{
		return 1;
	}
	return 0;
}

void work_rrq(in_addr_t raw_client_ip, in_port_t raw_client_port, uint8_t *buf, int len)
{
	printf("receive rrq, %s port %d, %s\n", inet_ntoa((struct in_addr){raw_client_ip}), ntohs(raw_client_port), buf);
	int fid;
	for (fid = 0; fid < NFILE; fid++)
	{
		if (strcmp((char *)buf, file_name[fid]) == 0) break;
	}
	if (fid == NFILE)
	{
		tftp_send_error(raw_client_ip, raw_client_port, 1, "File not found");
		return;
	}
	FILE *fp = fopen(file_path[fid], "rb");
	if (fp == NULL)
	{
		tftp_send_error(raw_client_ip, raw_client_port, 1, "File not found");
		return;
	}
	struct tftp_connect *conn;
	int found = 0;
	for (int i = 0; i < MAX_CONNS; i++)
	{
		conn = &connect_list[i];
		if (conn->available == 1 && conn->raw_client_ip == raw_client_ip && conn->raw_client_port == raw_client_port)
		{
			if (conn->acked_blk > 0)
				tftp_send_error(raw_client_ip, raw_client_port, 0, "Duplicated TFTP request");
			else
			{
				found = 1;
				break;
			}
		}
	}
	if (found == 0) for (int i = 0; i < MAX_CONNS; i++)
	{
		conn = &connect_list[i];
		if (conn->available == 0)
		{
			conn->raw_client_ip = raw_client_ip;
			conn->raw_client_port = raw_client_port;
			conn->available = 1;
			conn->acked_blk = 0;
			conn->lst_blk_id = -1;
			conn->fp = fp;
			found = 1;
			break;
		}
	}
	if (found == 0) tftp_send_error(raw_client_ip, raw_client_port, 3, "Reach max connection");
	else
	{
		tftp_send_data(conn->raw_client_ip, conn->raw_client_port, conn->fp, 1, BLK_SIZE);
	}
}

void work_wrq(in_addr_t raw_client_ip, in_port_t raw_client_port, uint8_t *buf, int len)
{
	tftp_send_error(raw_client_ip, raw_client_port, 2, "Read-only TFTP server");
}

void work_data(in_addr_t raw_client_ip, in_port_t raw_client_port, uint8_t *buf, int len)
{
	tftp_send_error(raw_client_ip, raw_client_port, 2, "Read-only TFTP server");
}

void work_ack(in_addr_t raw_client_ip, in_port_t raw_client_port, uint8_t *buf, int len)
{
	int acked_blk = ntohs(*(uint16_t *)buf);
	for (int i = 0; i < MAX_CONNS; i++)
	{
		struct tftp_connect *conn = &connect_list[i];
		if (conn->available == 1 && conn->raw_client_ip == raw_client_ip && conn->raw_client_port == raw_client_port)
		{
			if (acked_blk == 0 && ((conn->acked_blk + 1) & 0xFFFF) == 0)
			{
				acked_blk = conn->acked_blk + 1;
			}
			else
			{
				acked_blk += (conn->acked_blk & 0xFFFF0000);
			}
			if (acked_blk == conn->lst_blk_id || acked_blk != conn->acked_blk + 1)
			{
				printf("receive ack %d, %s port %d, last ack %d\n", acked_blk, inet_ntoa((struct in_addr){raw_client_ip}), ntohs(raw_client_port), conn->acked_blk);
			}
			if (acked_blk == conn->lst_blk_id)
			{
				conn->available = 0;
				fclose(conn->fp);
				return;
			}
			if (conn->acked_blk < acked_blk) conn->acked_blk = acked_blk;
			int ret = tftp_send_data(conn->raw_client_ip, conn->raw_client_port, conn->fp, conn->acked_blk + 1, BLK_SIZE);
			if (ret == 1)
			{
				conn->lst_blk_id = acked_blk + 1;
				
			}
			return;
		}
	}
	tftp_send_error(raw_client_ip, raw_client_port, 5, "Transfer not found");
}

void work(in_addr_t raw_client_ip, in_port_t raw_client_port, uint8_t *buf, int len)
{
	if (len < 2) return;
	int op = ntohs(*(unsigned short *)buf);
	switch (op)
	{
		case TFTP_OP_RRQ:
			work_rrq(raw_client_ip, raw_client_port, buf + 2, len - 2);
			break;
		case TFTP_OP_WRQ:
			work_wrq(raw_client_ip, raw_client_port, buf + 2, len - 2);
			break;
		case TFTP_OP_DATA:
			work_data(raw_client_ip, raw_client_port, buf + 2, len - 2);
			break;
		case TFTP_OP_ACK:
			work_ack(raw_client_ip, raw_client_port, buf + 2, len - 2);
			break;
	}

}

int main(int argc, char *argv[])
{
	socket_fd = socket(AF_INET, SOCK_DGRAM, 0);
	if (socket_fd == -1)
	{
		perror("create socket fail!\n");
		return 1;
	}
	int optval = 1;
	setsockopt(socket_fd, SOL_SOCKET, SO_REUSEADDR, &optval, sizeof(&optval));

	struct sockaddr_in server_addr;
	server_addr.sin_family = AF_INET;
	server_addr.sin_port = htons(SERVER_PORT);
	server_addr.sin_addr.s_addr = inet_addr(SERVER_IP);

	int ret = bind(socket_fd, (struct sockaddr *)&server_addr, sizeof(server_addr));
	if (ret == -1)
	{
		perror("bind fail!");
		return 1;
	}

	struct sockaddr_in recv_addr;
	socklen_t recv_len = sizeof(recv_addr);
	uint8_t buf[BUF_LEN];
	while (1)
	{
		memset(buf, 0, BUF_LEN);
		ret = recvfrom(socket_fd, buf, BUF_LEN, 0, (struct sockaddr *)&recv_addr, &recv_len);
		work(recv_addr.sin_addr.s_addr, recv_addr.sin_port, buf, ret);
	}
	// close(socket_fd);
	return 0;
}

