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

    useEffect(() => {
        console.log('timeserie', timeseries)
    }, [timeseries])


    return (
        <Box>
            <ChartHeader>
                <Typography>Gold Price</Typography>
                <Typography>XAU:USD</Typography>
                <Box>
                    <Typography>{timeseries[timeseries.length - 1].USD.toFixed(2)} $ / g</Typography>

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
        </Box>

    );
};

export default Chart;

const ChartHeader = styled(Box)`
    width: 100%;
`