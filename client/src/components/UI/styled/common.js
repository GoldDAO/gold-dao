import { theme } from '@/theme/theme';
import { CircularProgress } from '@mui/material';
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
    color: ${theme.colors.gold};
`;
