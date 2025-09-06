#include <nftables/libnftables.h>

int main(int argc, char **argv) {
	struct nft_ctx* ctx = nft_ctx_new(NFT_CTX_DEFAULT);
	nft_ctx_output_set_flags(ctx, NFT_CTX_OUTPUT_JSON);
	nft_run_cmd_from_buffer(ctx, "list ruleset");
	nft_ctx_free(ctx);
}
