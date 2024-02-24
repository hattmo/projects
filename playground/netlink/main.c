#define _GNU_SOURCE
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/socket.h>
#include <linux/netlink.h>
#include <linux/rtnetlink.h>
#include <linux/netfilter.h>
#include <linux/xfrm.h>
#include <linux/netfilter/x_tables.h>
#include <linux/netfilter_ipv4/ip_tables.h>

#define BUF_SIZE 4096

int main() {
    struct sockaddr_nl sa;
    int sock = socket(AF_NETLINK, SOCK_RAW, NETLINK_NETFILTER);
    if (sock == -1) {
        perror("socket");
        exit(EXIT_FAILURE);
    }

    memset(&sa, 0, sizeof(sa));
    sa.nl_family = AF_NETLINK;
    sa.nl_pid = getpid();
    sa.nl_groups = 0;  // Unicast

    if (bind(sock, (struct sockaddr*)&sa, sizeof(sa)) == -1) {
        perror("bind");
        close(sock);
        exit(EXIT_FAILURE);
    }

    // Prepare a netlink message to add a rule with Verdict Queue target
    struct nlmsghdr *nlh;
    char buf[BUF_SIZE];

    memset(buf, 0, BUF_SIZE);
    nlh = (struct nlmsghdr*)buf;

    nlh->nlmsg_len = NLMSG_LENGTH(sizeof(struct xt_entry_target));
    nlh->nlmsg_type = XFRM_MSG_NEWSA;
    nlh->nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK;
    nlh->nlmsg_seq = 0;
    nlh->nlmsg_pid = getpid();

    // Create the rule structure
    struct xt_entry_target* target = (struct xt_entry_target*)(NLMSG_DATA(nlh));
    target->u.kernel.target_size = sizeof(struct ipt_entry_target);
    target->u.user.table = NF_INET_FILTER;
    target->u.kernel.hook = NF_INET_PRE_ROUTING;
    target->u.kernel.target = XT_STANDARD_TARGET;

    // Customize the rule parameters for Verdict Queue
    struct ipt_entry* entry = &(target->u.user);
    entry->target_offset = sizeof(struct ipt_entry);
    entry->next_offset = entry->target_offset + sizeof(struct ipt_entry_target);
    entry->ip.proto = IPPROTO_TCP;  // Protocol to filter (e.g., TCP)
    entry->ip.flags = IPT_F_FRAG;   // Fragmentation flag
    entry->target.verdict = NF_QUEUE; // Verdict Queue action
    entry->target.queuenum = 42;      // Queue number (adjust as needed)

    // Send the netlink message
    if (send(sock, nlh, nlh->nlmsg_len, 0) == -1) {
        perror("send");
        close(sock);
        exit(EXIT_FAILURE);
    }

    // Receive the response
    ssize_t len = recv(sock, buf, BUF_SIZE, 0);
    if (len == -1) {
        perror("recv");
        close(sock);
        exit(EXIT_FAILURE);
    }

    // Process the response as needed

    close(sock);
    return 0;
}
