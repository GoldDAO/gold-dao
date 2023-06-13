import { ConnectButton, ConnectDialog, useConnect, useDialog } from '@connect2ic/react';
import { Box } from '@mui/material';
import { useAtom } from 'jotai';
import React, { useEffect } from 'react';
import { setGetUserAtom } from '../../states/user';
import { defaultAgent } from '../../states/agents';
import { HttpAgent } from "@dfinity/agent";
import GetOrigynNFTs from '../commands/getBalanceOrigynNFTs';
import { createActor } from '../../../src/agents/declarations/NFTs/GOLD_1g_NFT';
import { setGetgoldNft1GAgentAtom } from '../../states/agents/goldNft-1';
import { NFT_1000_CANISTER_ID, NFT_100_CANISTER_ID, NFT_10_CANISTER_ID, NFT_1_CANISTER_ID } from '../../../src/constant';
import { setGetgoldNft10GAgentAtom } from '../../states/agents/goldNft-10';
import { setGetgoldNft1000GAgentAtom } from '../../states/agents/goldNft-1000';
import { setGetgoldNft100GAgentAtom } from '../../states/agents/goldNft-100';


const C2ic = () => {
    const [, setUser] = useAtom(setGetUserAtom)
    const { close, isOpen } = useDialog()
    const [, setNFT1Agent] = useAtom(setGetgoldNft1GAgentAtom)
    const [, setNFT10Agent] = useAtom(setGetgoldNft10GAgentAtom)
    const [, setNFT100Agent] = useAtom(setGetgoldNft100GAgentAtom)
    const [, setNFT1000Agent] = useAtom(setGetgoldNft1000GAgentAtom)

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
            setNFT1Agent(null)
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
            const GOLD_NFT_1g_Agent = createActor(NFT_1_CANISTER_ID, agent)
            const GOLD_NFT_10g_Agent = createActor(NFT_10_CANISTER_ID, agent)
            const GOLD_NFT_100g_Agent = createActor(NFT_100_CANISTER_ID, agent)
            const GOLD_NFT_1000g_Agent = createActor(NFT_1000_CANISTER_ID, agent)
            setNFT1Agent(GOLD_NFT_1g_Agent)
            setNFT10Agent(GOLD_NFT_10g_Agent)
            setNFT100Agent(GOLD_NFT_100g_Agent)
            setNFT1000Agent(GOLD_NFT_1000g_Agent)
        } else {
            setNFT1Agent(defaultAgent)
            setNFT10Agent(defaultAgent)
            setNFT100Agent(defaultAgent)
            setNFT1000Agent(defaultAgent)
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