import { Box } from '@mui/system';
import React from 'react';

const Marketcap = ({data}) => {
    return (
        <Box sx={{backgroundColor: '#F2F2F2', width: 'fit-content', padding: '39px',borderRadius: "10px",fontSize: '36px', display: 'flex', margin: "100px auto"}} >
            <Box sx={{fontWeight: 600, marginRight: "15px"}}>{data.label}</Box>
            <Box sx={{color: "#D3B872"}}> {data.value}</Box>
        </Box>
    );
};

export default Marketcap;