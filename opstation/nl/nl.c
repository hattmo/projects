#include "netlink/addr.h"
#include "netlink/errno.h"
#include "netlink/object.h"
#include "netlink/types.h"
#include <arpa/inet.h>
#include <linux/netlink.h>
#include <net/if.h>
#include <netinet/in.h>
#include <netlink/cache.h>
#include <netlink/errno.h>
#include <netlink/handlers.h>
#include <netlink/netlink.h>
#include <netlink/route/addr.h>
#include <netlink/route/link.h>
#include <netlink/route/link/vxlan.h>
#include <netlink/socket.h>
#include <stdio.h>
#include <string.h>
#include <sys/socket.h>

int main(int argc, char **argv) {
  struct nl_cache *cache;
  struct nl_sock *sock = nl_socket_alloc();
  if (!sock) {
    printf("error alloc socket\n");
    return 1;
  }
  nl_connect(sock, NETLINK_ROUTE);
  rtnl_link_alloc_cache(sock, AF_UNSPEC, &cache);

  struct rtnl_link *link = rtnl_link_vxlan_alloc();
  struct nl_dump_params params = {.dp_fd = stdout, .dp_type = NL_DUMP_STATS};
  // nl_cache_dump(cache, &params);
  struct nl_addr *local;
  nl_addr_parse("172.25.212.122", AF_INET, &local);
  rtnl_link_vxlan_set_local(link, local);
  nl_addr_put(local);

  struct nl_addr *remote;
  nl_addr_parse("172.25.212.125", AF_INET, &remote);
  rtnl_link_vxlan_set_group(link, remote);
  nl_addr_put(remote);

  rtnl_link_vxlan_set_id(link, 1234);
  int ifindex = rtnl_link_name2i(cache, "eth0");
  rtnl_link_set_link(link, ifindex);
  rtnl_link_set_name(link, "vxlan0");
  nl_object_dump((struct nl_object *)link, &params);
  int ret = rtnl_link_add(sock, link, NLM_F_CREATE);
  // check ret and clean up
  if (ret < 0) {
    nl_perror(ret, "add");
  }
  return 0;

  int i = 1;
  for (int i = 0; i < 10; i++) {
    struct rtnl_link *link = rtnl_link_get(cache, i);
    if (!link) {
      continue;
    }
    char *name = rtnl_link_get_name(link);
    if (!name) {
      rtnl_link_put(link);
      continue;
    }
    if (!strcmp(name, "wg0")) {
      printf("wg0 found\n");
      struct nl_addr *addr;
      nl_addr_parse("192.168.0.5/24", AF_INET, &addr);
      struct nl_addr *broad;
      nl_addr_parse("192.168.0.255", AF_INET, &broad);

      struct rtnl_addr *local = rtnl_addr_alloc();
      rtnl_addr_set_broadcast(local, broad);
      rtnl_addr_set_local(local, addr);
      rtnl_addr_set_link(local, link);
      int ret = rtnl_addr_add(sock, local, 0);
      if (ret) {
        printf("failed to set ip\n");
        return 1;
      }
      struct rtnl_link *change = rtnl_link_alloc();
      rtnl_link_set_flags(change, IFF_UP);
      ret = rtnl_link_change(sock, link, change, 0);
      if (ret) {
        printf("failed to set up\n");
        return 1;
      }
      printf("success\n");

      rtnl_link_put(change);
      nl_addr_put(addr);
      nl_addr_put(broad);
      rtnl_addr_put(local);
    };
    rtnl_link_put(link);
    i++;
  }
  nl_cache_free(cache);
  nl_socket_free(sock);
  return 0;
}
