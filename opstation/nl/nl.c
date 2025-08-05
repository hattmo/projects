#include <net/if.h>
#include <netlink/cache.h>
#include <netlink/handlers.h>
#include <netlink/netlink.h>
#include <netlink/route/addr.h>
#include <netlink/route/link.h>
#include <netlink/route/link/vxlan.h>
#include <netlink/socket.h>
#include <stdio.h>
#include <string.h>
#include <sys/socket.h>

int main(int argc, char** argv) {
  struct nl_cache* cache;
  struct nl_sock* sock = nl_socket_alloc();
  if (!sock) {
    printf("error alloc socket\n");
    return 1;
  }
  nl_connect(sock, NETLINK_ROUTE);
  int ret = rtnl_link_alloc_cache(sock, AF_UNSPEC, &cache);
  if (ret < 0) {
    printf("error alloc cache\n");
    return 1;
  }
  int i = 1;
  for (int i = 0; i < 10; i++) {
    struct rtnl_link* link = rtnl_link_get(cache, i);
    if (!link) {
      continue;
    }
    char* name = rtnl_link_get_name(link);
    if (!name) {
      rtnl_link_put(link);
      continue;
    }
    if (!strcmp(name, "wg0")) {
      printf("wg0 found\n");
      struct nl_addr* addr;
      nl_addr_parse("192.168.0.5/24", AF_INET, &addr);
      struct nl_addr* broad;
      nl_addr_parse("192.168.0.255", AF_INET, &broad);
      struct nl_addr* phy;
      nl_addr_parse("00:11:22:33:44:55", AF_LLC, &phy);

      struct rtnl_addr* local = rtnl_addr_alloc();
      rtnl_addr_set_broadcast(local, broad);
      rtnl_addr_set_local(local, addr);
      rtnl_addr_set_link(local, link);
      // int ret = rtnl_addr_add(sock, local, 0);
      // if (ret) {
      //   printf("failed to set ip\n");
      //   return 1;
      // }
      struct rtnl_link* change = rtnl_link_alloc();
      // rtnl_link_set_flags(change, IFF_UP);
      rtnl_link_set_addr(change, phy);
      ret = rtnl_link_change(sock, link, change, 0);
      if (ret) {
        printf("failed to set mac\n");
        return 1;
      }
      printf("success\n");
      rtnl_link_put(change);
      nl_addr_put(addr);
      nl_addr_put(broad);
      nl_addr_put(phy);
      rtnl_addr_put(local);
    };
    rtnl_link_put(link);
    i++;
  }
  nl_cache_free(cache);
  nl_socket_free(sock);
  rtnl_link_vxlan_set_remote();
  return 0;
}
