import { ConnectButton, ConnectDialog, useConnect, useDialog } from '@connect2ic/react';
import { Box } from '@mui/material';
import { useAtom } from 'jotai';
import React, { useEffect } from 'react';
import { setGetUserAtom } from '../../states/user';
import { HttpAgent } from "@dfinity/agent";
import GetOrigynNFTs from '../commands/getBalanceOrigynNFTs';
import { createActor } from '../../../src/agents/declarations/NFTs/GOLD_1g_NFT';
import { NftGAgentAtom, defaultAgent } from '../../states/agents/goldNft-1';
import { NFT_1000_CANISTER_ID, NFT_100_CANISTER_ID, NFT_10_CANISTER_ID, NFT_1_CANISTER_ID } from '../../../src/constant';

const C2ic = () => {
    const [, setUser] = useAtom(setGetUserAtom)
    const { close, isOpen } = useDialog()
    const [, setNFTsAgents] = useAtom(NftGAgentAtom)

    useEffect(() => {
        console.log('isOpen ', isOpen)
    }, [isOpen])

    const {
        principal,
        connect,
        disconnect,
        status,
        isInitializing,
        isIdle,
        isConnecting,
        isConnected,
        isDisconnecting,
        activeProvider, } = useConnect({
            onConnect: () => {
                console.log('CONNECTED')
                close()
                document.getElementsByTagName("body")[0].style.overflow = 'scroll'
            },
            onDisconnect: () => {
                console.log('DISCONNECTED')
                setUser({
                    principal: undefined,
                    connect: undefined,
                    disconnect: undefined,
                    status: undefined,
                    isInitializing: undefined,
                    isIdle: undefined,
                    isConnecting: false,
                    isConnected: false,
                    isDisconnecting: undefined,
                    activeProvider: undefined
                })
            }
        })

    useEffect(() => {
        if (isConnected) {
            setUser({
                principal,
                connect,
                disconnect,
                status,
                isInitializing,
                isIdle,
                isConnecting,
                isConnected,
                isDisconnecting,
                activeProvider
            })

        } else {
            setNFTsAgents(null)
        }
    }, [isConnected]);

    useEffect(() => {
        console.log('status', status)
    }, [status])

    useEffect(() => {
        if (activeProvider) {
            const agent = new HttpAgent({
                identity: activeProvider.identity,
                host: "https://ic0.app",
            });
            const NFT_1g = createActor(NFT_1_CANISTER_ID, agent)
            const NFT_10g = createActor(NFT_10_CANISTER_ID, agent)
            const NFT_100g = createActor(NFT_100_CANISTER_ID, agent)
            const NFT_1000g = createActor(NFT_1000_CANISTER_ID, agent)
            const NFTsAgents = {
                NFT_1g,
                NFT_10g,
                NFT_100g,
                NFT_1000g
            }
            setNFTsAgents(NFTsAgents)
        } else {
            setNFTsAgents(defaultAgent)
        }
    }, [activeProvider])

    return (
        <Box>
            {!isConnected &&
                <>
                    <ConnectButton
                        style={{
                            backgroundColor: "#D3B872",
                            borderRadius: '10px',
                            fontSize: '20px',
                            textTransform: 'uppercase'
                        }}
                    />
                    <ConnectDialog dark={false} />
                </>
            }
            {isConnected &&
                <GetOrigynNFTs />}
        </Box>
    );
};

export default C2ic;