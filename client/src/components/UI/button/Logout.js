import { Button, IconButton } from '@mui/material';
import React from 'react';
import LogoutIcon from '@mui/icons-material/Logout';
import { useAtom } from 'jotai';
import { userAtom } from '../../../states/user';
import styled from 'styled-components';

const LogoutButton = () => {
    const [user, setUser] = useAtom(userAtom)
    return (
        <LogOut onClick={() => user.disconnect()}><LogoutIcon /><div>Logout</div></LogOut >
    );
};

export default LogoutButton;

const LogOut = styled(Button)`
    display: flex;
    align-items: center;
    justify-content: flex-end;
    &.MuiButtonBase-root{
        display: flex;
        align-items: center;
        justify-content: flex-end;
    }
    svg{
        color: #000;
        fill: #000;
        width: 20px;
        margin-right: 10px;
    }
    div{
        color: #000;
        font-size: 16px;
        text-transform: lowercase;
    }
`