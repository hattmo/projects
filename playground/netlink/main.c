#include <stdio.h>
#include <sys/socket.h>
#include <linux/netlink.h>
#include <linux/genetlink.h>
#include <linux/netfilter/nfnetlink.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
int main(int argc, char *argv[])
{

    int fd = socket(AF_NETLINK, SOCK_RAW | SOCK_CLOEXEC, NETLINK_GENERIC);

    if (fd < 0)
    {
        perror("socket");
        return 1;
    }

    struct sockaddr_nl src_addr;
    memset(&src_addr, 0, sizeof(src_addr));
    src_addr.nl_family = AF_NETLINK;
    src_addr.nl_pid = getpid(); /* self pid */
    src_addr.nl_groups = 0;     /* not in mcast groups */
    bind(fd, (struct sockaddr *)&src_addr, sizeof(src_addr));

    struct sockaddr_nl dest_addr;
    memset(&dest_addr, 0, sizeof(dest_addr));
    dest_addr.nl_family = AF_NETLINK;
    dest_addr.nl_pid = 0;    /* For Linux Kernel */
    dest_addr.nl_groups = 0; /* unicast */

    struct nlmsghdr nlhdr = {0};
    nlhdr.nlmsg_len = 0x20;
    nlhdr.nlmsg_type = GENL_ID_CTRL;
    nlhdr.nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK;
    nlhdr.nlmsg_seq = 1;
    nlhdr.nlmsg_pid = 0;

    struct genlmsghdr genlhdr = {0};
    genlhdr.cmd = CTRL_CMD_GETFAMILY;
    genlhdr.version = 1;
    genlhdr.reserved = 0;

    char *message = "test1\0\0";

    struct nlattr nla = {0};
    nla.nla_len = NLA_HDRLEN + strlen(message) + 1;
    nla.nla_type = CTRL_ATTR_FAMILY_NAME;

    char *buffer = malloc(32);
    memcpy(buffer, &nlhdr, sizeof(nlhdr));
    memcpy(buffer + sizeof(nlhdr), &genlhdr, sizeof(genlhdr));
    memcpy(buffer + sizeof(nlhdr) + sizeof(genlhdr), &nla, sizeof(nla));
    memcpy(buffer + sizeof(nlhdr) + sizeof(genlhdr) + sizeof(nla), message, 8);
    for (int i = 0; i < 32; i++)
    {
        printf("%x ", buffer[i]);
    }
    printf("\n");
    struct nlmsghdr *foo = (struct nlmsghdr *)buffer;
    foo->nfgen_family = 0x10;
    int res = send(fd, buffer, 32, 0);
    if (res < 0)
    {
        perror("send");
        return 1;
    }
    char *recv_buff = malloc(2048);
    char *cursor = recv_buff;
    int num_recv = recv(fd, recv_buff, 2048, 0);
    printf("num_recv: %d\n", num_recv);
    struct nlmsghdr *recv_nlhdr = (struct nlmsghdr *)cursor;
    printf("\nnlmsg_type: %d\nnlmsg_flags: %d\nnlmsg_len: %d\nnlmsg_seq: %d\nnlmsg_pid: %d\n", recv_nlhdr->nlmsg_type, recv_nlhdr->nlmsg_flags, recv_nlhdr->nlmsg_len, recv_nlhdr->nlmsg_seq, recv_nlhdr->nlmsg_pid);
    cursor += sizeof(struct nlmsghdr);
    struct genlmsghdr *recv_genlhdr = (struct genlmsghdr *)cursor;
    printf("\ncmd: %d\nversion: %d\nreserved: %d\n", recv_genlhdr->cmd, recv_genlhdr->version, recv_genlhdr->reserved);
}