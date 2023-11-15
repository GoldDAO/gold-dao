import React from 'react';
import moment from 'moment';
import { Box, Tooltip } from '@chakra-ui/react';

const TooltipContent = ({ timestamp }) => {
	const now = new Date();
	const then = new Date(timestamp);
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
	const timestampSeconds = timestamp / 10 ** 6;
	return (
		<Tooltip sx={{ fontSize: '1em' }} label={<TooltipContent timestamp={timestampSeconds} />}>
			<Box style={{ fontSize: '1em', margin: 0 }}>
				{moment(timestampSeconds).format('DD/MM/YYYY / hh:mm:ss +TZ')}
			</Box>
		</Tooltip>
	);
};

export default Timestamp;
