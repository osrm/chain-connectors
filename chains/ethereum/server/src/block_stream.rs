use super::{
    block_provider::BlockProvider,
    event_stream::{EthereumEventStream, NewBlock},
    state::State,
};
use futures_util::StreamExt;
use rosetta_config_ethereum::Event as EthEvent;
use rosetta_core::{stream::Stream, types::BlockIdentifier, BlockOrIdentifier, ClientEvent};
use rosetta_ethereum_backend::{
    ext::types::{rpc::RpcBlock, H256},
    jsonrpsee::core::{client::Subscription, ClientError as RpcError},
    EthereumPubSub,
};
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct BlockStream<P, RPC>
where
    P: BlockProvider + Unpin + Send + Sync + 'static,
    P::Error: std::error::Error + Send + Sync + 'static,
    RPC: for<'s> EthereumPubSub<Error = RpcError, NewHeadsStream<'s> = Subscription<RpcBlock<H256>>>
        + Unpin
        + Send
        + Sync
        + 'static,
    RPC::SubscriptionError: Send + Sync + 'static,
{
    stream: Option<EthereumEventStream<P, RPC>>,
    state: State,
}

impl<P, RPC> BlockStream<P, RPC>
where
    P: BlockProvider + Unpin + Send + Sync + 'static,
    P::Error: std::error::Error + Send + Sync + 'static,
    RPC: for<'s> EthereumPubSub<Error = RpcError, NewHeadsStream<'s> = Subscription<RpcBlock<H256>>>
        + Unpin
        + Send
        + Sync
        + 'static,
    RPC::SubscriptionError: Send + Sync + 'static,
{
    #[must_use]
    pub fn new(provider: P, client: RPC, state: State) -> Self {
        Self { stream: Some(EthereumEventStream::new(client, provider)), state }
    }
}

impl<P, RPC> Stream for BlockStream<P, RPC>
where
    P: BlockProvider + Unpin + Send + Sync + 'static,
    P::Error: std::error::Error + Send + Sync + 'static,
    RPC: for<'s> EthereumPubSub<Error = RpcError, NewHeadsStream<'s> = Subscription<RpcBlock<H256>>>
        + Unpin
        + Send
        + Sync
        + 'static,
    RPC::SubscriptionError: Send + Sync + 'static,
{
    type Item = ClientEvent<BlockIdentifier, EthEvent>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let Some(mut stream) = self.stream.take() else {
            return Poll::Ready(None);
        };

        let mut failures = 0;
        loop {
            match stream.poll_next_unpin(cx) {
                Poll::Ready(Some(new_block)) => {
                    let block_id = {
                        let header = new_block.sealed_block().header();
                        BlockOrIdentifier::Identifier(BlockIdentifier {
                            index: header.number(),
                            hash: header.hash().0,
                        })
                    };
                    let is_finalized = matches!(new_block, NewBlock::Finalized(_));
                    if let Err(err) = self.state.import(new_block.into_sealed_block()) {
                        failures += 1;
                        tracing::warn!("failed to import block {block_id} ({failures}): {err:?}");
                        if failures >= 5 {
                            return Poll::Ready(None);
                        }
                        continue;
                    }

                    let event = if is_finalized {
                        ClientEvent::NewFinalized(block_id)
                    } else {
                        ClientEvent::NewHead(block_id)
                    };
                    self.stream = Some(stream);
                    break Poll::Ready(Some(event));
                },
                Poll::Ready(None) => break Poll::Ready(None),
                Poll::Pending => {
                    self.stream = Some(stream);
                    break Poll::Pending;
                },
            }
        }
    }
}
