import React, { useEffect, useState } from 'react';
import { getGoldTimeserie } from '../../utils/getGoldPrice';

import { ResponsiveContainer, LineChart, XAxis, YAxis, Tooltip, CartesianGrid, Legend, Line } from 'recharts'
import { Box, Typography } from '@mui/material';
import styled from 'styled-components';


const Chart = () => {
    const [timeseries, setTimeseries] = useState()

    useEffect(() => {
        const fetchData = async () => {
            const timeserie = await getGoldTimeserie()
            setTimeseries(timeserie)
        }
        fetchData()
    }, [])

    return (
        <Box sx={{ padding: '40px 0' }}>
            <ChartHeader>
                <ChartTitle>Gold Price</ChartTitle>
                <Typography sx={{ fontSize: '14px', paddingTop: "15px" }}>XAU:USD</Typography>
                <Box sx={{ display: 'flex', alignItems: 'center' }}>
                    {timeseries &&
                        <Price>{timeseries[timeseries.length - 1].USD.toFixed(2)}</Price>}
                    <Metrics>USD/g</Metrics>
                </Box>
            </ChartHeader>
            <Box style={{ height: '400px', fontSize: "14px" }} >
                <ResponsiveContainer width="100%" height="100%" >
                    <LineChart
                        width={500}
                        height={300}
                        data={timeseries}
                        margin={{
                            top: 5,
                            right: 30,
                            left: 20,
                            bottom: 5,
                        }}
                    >
                        <CartesianGrid horizontal vertical={false} />
                        <XAxis dataKey="date" interval={40} />
                        <YAxis dataKey="USD" domain={[44, 80]} unit={'$'} />
                        <Tooltip />
                        <Line
                            strokeWidth={2}
                            type="monotone"
                            dataKey='USD'
                            stroke="#d3b872"
                            dot={false}
                            activeDot={false} />
                    </LineChart>
                </ResponsiveContainer>
            </Box>
        </Box >

    );
};

export default Chart;

const ChartHeader = styled(Box)`
    width: 100%;
    padding-bottom: 40px;
`

const ChartTitle = styled('h4')`
    font-size: 18px;
    width: 100%;
    padding-bottom: 15px;
    border-bottom: 1px solid #D3B872;
    font-weight: 500;
`
const Price = styled('p')`
    font-size: 32px;
    margin-right: 6px;    
    font-weight: 500;
`

const Metrics = styled('p')`
    margin-right: 24px;
    font-size: 14px;
`