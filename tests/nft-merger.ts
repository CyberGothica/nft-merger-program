import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { NftMerger } from '../target/types/nft_merger';

describe('nft-merger', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.NftMerger as Program<NftMerger>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
