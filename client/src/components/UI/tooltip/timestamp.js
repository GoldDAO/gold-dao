import React from 'react';
import { Box, Tooltip } from '@mui/material';


const TooltipContent = ({ timestamp }) => {
    const now = new Date();
    const then = new Date(timestamp);
    console.log('then', then)
    const seconds = Math.floor((now - then) / 1000);

    const days = Math.floor(seconds / (24 * 60 * 60));
    const hours = Math.floor(seconds / (60 * 60)) % 24;
    const minutes = Math.floor(seconds / 60) % 60;

    const daysStr = days > 0 ? days.toString().padStart(2, '0') : '0';
    const hoursStr = hours.toString().padStart(2, '0');
    const minutesStr = minutes.toString().padStart(2, '0');

    return (
        <>
            {daysStr} days, {hoursStr} hours, {minutesStr} minutes ago
        </>
    );
};

const Timestamp = ({ timestamp }) => {
    const date = new Date(timestamp / 1000000);
    const day = date.getDate();
    const month = date.getMonth() + 1;
    const year = date.getFullYear();
    const hours = date.getHours();
    const minutes = date.getMinutes();

    const dateFormated = `${day.toString().padStart(2, '0')}/${month
        .toString()
        .padStart(2, '0')}/${year} ${hours.toString().padStart(2, '0')}:${minutes
            .toString()
            .padStart(2, '0')}`;

    return (
        <Tooltip
            sx={{ fontSize: '1em' }}
            title={<TooltipContent timestamp={timestamp / 1000000} />}
        >
            <Box style={{ fontSize: '1em', margin: 0 }}>{dateFormated}</Box>
        </Tooltip>
    );
};

export default Timestamp;
