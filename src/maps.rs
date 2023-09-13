use substreams::errors::Error;
use substreams::log;
use substreams_antelope::Block;

use crate::abi;
use crate::waxmarketplaces::*;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<AnyEvents, Error> {
    let mut response = vec![];

    for trx in block.all_transaction_traces() {
        // action traces
        for trace in &trx.action_traces {
            let action_trace = trace.action.as_ref().unwrap();
            if action_trace.account != trace.receiver {continue}
            if action_trace.account != "atomicmarket" {continue}
            // Announce Sale event
            if action_trace.name == "announcesale" {
                match abi::Announcesale::try_from(action_trace.json_data.as_str()) {
                    Ok(data) => {
                        if !["nft.hive", "market.nefty", "chainchampss"].contains(&data.maker_marketplace.as_str()) {continue}
                        response.push(AnyEvent{
                            event: Some(any_event::Event::Saleitem(
                                SaleEvent {
                                trx_id: trx.id.clone(),
                                timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),
                                event: action_trace.name.clone(),
                                seller: data.seller,
                                asset_ids: data.asset_ids,
                                listing_price: data.listing_price,
                                settlement_symbol: data.settlement_symbol,
                                maker_marketplace: data.maker_marketplace,
                                }
                            ))
                        });
                    },
                    Err(_) => {
                        // print data to console
                        log::info!("{:?}", action_trace.json_data);
                        panic!("Failed to decode atomicmarket::announcesale");
                    }
                }
            }
            // Announce Auction event
            if action_trace.name == "announceauct" {
                match abi::Announceauct::try_from(action_trace.json_data.as_str()) {
                    Ok(data) => {
                        if !["nft.hive", "market.nefty", "chainchampss"].contains(&data.maker_marketplace.as_str()) {continue}
                        response.push(AnyEvent{
                            event: Some(any_event::Event::Auctionitem(
                                AuctionEvent {
                                trx_id: trx.id.clone(),
                                timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),
                                event: action_trace.name.clone(),
                                seller: data.seller,
                                asset_ids: data.asset_ids,
                                starting_bid: data.starting_bid,
                                duration: data.duration,
                                maker_marketplace: data.maker_marketplace,
                                }
                            ))
                        });
                    },
                    Err(_) => {
                        // print data to console
                        log::info!("{:?}", action_trace.json_data);
                        panic!("Failed to decode atomicmarket::announceauct");
                    }
                }
            }
            // Lognewbuyo event
            if action_trace.name == "lognewbuyo" {
                match abi::Lognewbuyo::try_from(action_trace.json_data.as_str()) {
                    Ok(data) => {
                        if !["nft.hive", "market.nefty", "chainchampss"].contains(&data.maker_marketplace.as_str()) {continue}
                        response.push(AnyEvent{
                            event: Some(any_event::Event::Newbuyoitem(
                                NewBuyo {
                                trx_id: trx.id.clone(),
                                timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),
                                event: action_trace.name.clone(),
                                buyoffer_id: data.buyoffer_id,
                                buyer: data.buyer,
                                recipient: data.recipient,
                                price: data.price,
                                asset_ids: data.asset_ids,
                                memo: data.memo,
                                maker_marketplace: data.maker_marketplace,
                                collection_name: data.collection_name,
                                collection_fee: data.collection_fee,
                                }
                            ))
                        });
                    },
                    Err(_) => {
                        // print data to console
                        log::info!("{:?}", action_trace.json_data);
                        panic!("Failed to decode atomicmarket::lognewbuyo");
                    }
                }
            }
            // Purchasesale event
            if action_trace.name == "purchasesale" {
                match abi::Purchasesale::try_from(action_trace.json_data.as_str()) {
                    Ok(data) => {
                        if !["nft.hive", "market.nefty", "chainchampss"].contains(&data.taker_marketplace.as_str()) {continue}
                        response.push(AnyEvent{
                            event: Some(any_event::Event::Purchasesaleitem(
                                PurchaseSale {
                                trx_id: trx.id.clone(),
                                timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),
                                event: action_trace.name.clone(),
                                buyer: data.buyer,
                                sale_id: data.sale_id,
                                intended_delphi_median: data.intended_delphi_median,
                                taker_marketplace: data.taker_marketplace,
                                }
                            ))
                        });
                    },
                    Err(_) => {
                        // print data to console
                        log::info!("{:?}", action_trace.json_data);
                        panic!("Failed to decode atomicmarket::purchasesale");
                    }
                }
            }
        }
    }
    // Handle any event
    Ok(AnyEvents { items: response })
}