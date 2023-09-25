import { theme } from '@/theme/theme';
import {
    Box,
    Checkbox,
    CircularProgress,
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableRow,
    Typography,
} from '@mui/material';
import styled from 'styled-components';

export const SectionTitle = styled('h3')`
    color: #333;
    padding-bottom: 15px;
    font-size: 2em;
    padding: 80px 0 15px 0;
    margin-bottom: 80px;
    @media (max-width: 1140px) {
        margin-bottom: 40px;
        font-size: 1.8em;
    }
    @media (max-width: 840px) {
        font-size: 1.4em;
        margin-bottom: 20px;
        padding: 40px 0;
        padding-bottom: 10px;
    }
    @media (max-width: 480px) {
        font-size: 1.2em;
        padding: 30px 0;
        margin-bottom: 20px;
        padding-bottom: 10px;
    }
`;

export const CustomCircularProgress = styled(CircularProgress)`
    color: ${theme.colors.gold} !important;
    fill: ${theme.colors.gold} !important;
`;
export const TableContainer = styled(Box)`
    border-radius: 20px;
    grid-column: 1/13;
    border: 1px solid ${theme.colors.gold};
`;

export const MarketCapContainer = styled(Box)`
    font-size: 1em;
    background-color: #f7f7f7;
    width: fit-content;
    padding: 20px;
    border-radius: 10px;
    display: flex;
    @media (max-width: 940px) {
        font-size: 0.8em;
    }
    @media (max-width: 540px) {
        width: 100%;
    }
`;
export const StyledTableRow = styled(TableRow)`
    width: 100%;
    display: flex;
    justify-content: space-between;
`;
export const StyledCheckbox = styled(Checkbox)``;

export const StyledTableHead = styled(TableHead)`
    font-weight: 400;
    width: 100%;
    border-radius: 20px 20px 0 0;
    background-color: ${theme.colors.grey};
    display: block;
`;
export const StyledTableCell = styled(TableCell)`
    font-weight: inherit;
    width: 100%;
    padding: 15px 40px 10px 40px;
`;

export const StyledTable = styled(Table)`
    border-radius: 20px;
`;
export const ItemName = styled(Typography)`
    height: 100%;
    align-items: center;
    display: inline-flex;
    padding-left: 16px;
`;

export const CustomTableBody = styled(TableBody)`
    height: ${(props) => {
        console.log('props.height', props);
        return props.height ? props.height : '300px';
    }};
    flex-grow: 1;
    overflow: scroll;
    display: block;
    width: 100%;
`;
