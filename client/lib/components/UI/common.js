import styled from "styled-components";

export const SectionTitle = styled('h3')`
    color: #333;
    font-weight: 300;
    padding-bottom: 15px;
    border-bottom: 1px solid #D3B872;
    font-size: 2em;
    padding: 80px 0 15px 0;
    margin-bottom: 80px;
    @media (max-width: 1140px){
    margin-bottom: 40px;
    font-size: 1.8em;
    }
    @media (max-width: 840px){
    font-size: 1.4em;
    margin-bottom: 20px;
    padding: 40px 0;
    padding-bottom: 10px;
    }
    @media (max-width: 480px){
    font-size: 1.2em;
    padding: 30px 0;
    margin-bottom: 20px;
    padding-bottom: 10px;
    }
`
