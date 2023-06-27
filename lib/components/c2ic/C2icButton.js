import { ConnectButton, ConnectDialog, useConnect, useDialog } from '@connect2ic/react';
import { Box, Button } from '@mui/material';
import { useAtom } from 'jotai';
import React, { useEffect } from 'react';
import { setGetUserAtom } from '../../states/user';
import { HttpAgent } from "@dfinity/agent";
import { createActor } from '../../../src/agents/declarations/NFTs/GOLD_1g_NFT';
import { NftGAgentAtom, defaultAgent } from '../../states/agents/GLDNFT';
import { NFT_1000_CANISTER_ID, NFT_100_CANISTER_ID, NFT_10_CANISTER_ID, NFT_1_CANISTER_ID } from '../../../src/constant';
import { appStatusAtom } from '../../states/appStatus';
import GetBalanceOrigynNFTs from '../commands/getBalanceOrigynNFTs';
import { idlFactory } from "./../../../src/agents/declarations/NFTs/GOLD_1g_NFT";
import { plugAgentAtom } from '../../states/plugAgent';
export { idlFactory };
import { useCanister } from '@connect2ic/react';

const C2icButton = () => {
    const [, setUser] = useAtom(setGetUserAtom)
    const [, setAppStatus] = useAtom(appStatusAtom)
    const { close, isOpen } = useDialog()
    const [, setNFTsAgents] = useAtom(NftGAgentAtom)
    const [, setPlugActor] = useAtom(plugAgentAtom)

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
                console.log('actor', actor)
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
        setAppStatus(status.idle)
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
            setPlugActor(
                window.ic.plug.createActor({
                    canisterId: NFT_1_CANISTER_ID,
                    interfaceFactory: idlFactory
                }))
            setNFTsAgents(NFTsAgents)
        } else {
            setNFTsAgents(defaultAgent)
        }
    }, [activeProvider])

    return (
        <Box>
            <>
                <ConnectButton
                    style={{
                        backgroundColor: "#D3B872",
                        borderRadius: '10px',
                        fontSize: '20px',
                        textTransform: 'uppercase'
                    }}
                />
                <ConnectDialog />
                {isConnected &&
                    <GetBalanceOrigynNFTs />
                }
            </>
        </Box>
    );
};

export default C2icButton;