### NFT merger
Merger for CyberGothica's nfts.
twitter.com/Cyber_Gothica

Current repo is fork of metaplex program library project: https://github.com/metaplex-foundation/metaplex-program-library

UI client: https://github.com/Yuriy-Ihor/nft-merger-client 

Repo consists of 2 programs:
1. NFT burner: clients sends NFT to burn. Program verifies them and if everything is ok, they are burned.
2. NFT minter: fork of metaplex candy machine v2. A lot of functions were removed (like token mint), but minor functionalities were added: it checks whether NFT burn instruction exists in list of transaction's instructions. If yes, user is able to get new NFT. 

Commands:
anchor build
solana address -k ./target/deploy/nft_merge_minter-keypair.json <- keypair used for nft burner
solana address -k ./target/deploy/nft_merge_burner-keypair.json <- keypair for nft minter
anchor test