import { Grid, GridItem, Heading, Text } from '@chakra-ui/react';
import Link from 'next/link';
import React from 'react';
import GridSystem from '../layout/Grid';

const TextBloc = ({
    title,
    content,
    link,
    subtitle,
    titleSpan,
    textSpan,
    titleOrder,
    colStart,
    colEnd,
    children,
    textOrder,
    childrenSpan,
    variant,
    titleAlign,
}) => {
    return (
        <GridSystem gap={4}>
            <GridItem colSpan={titleSpan} order={titleOrder} colStart={variant ? '' : colStart}>
                <Heading variant="h4" as="h4" textAlign={titleAlign}>
                    {title}
                </Heading>
                {subtitle && (
                    <Text as="h4" textAlign={titleAlign}>
                        {subtitle}
                    </Text>
                )}
                {link && <Link href={link.href}>{link.label}</Link>}
            </GridItem>
            <GridItem
                colSpan={textSpan}
                order={textOrder}
                colEnd={children ? '' : variant ? '' : colEnd}
                colStart={variant ? colStart : ''}
            >
                <Text fontSize="20px" lineHeight={'26px'}>
                    {content}
                </Text>
            </GridItem>
            {children && (
                <GridItem
                    colSpan={childrenSpan}
                    order={2}
                    colEnd={variant ? '' : colEnd}
                    colStart={variant ? colStart : 'colStart'}
                >
                    {children}
                </GridItem>
            )}
        </GridSystem>
    );
};

export default TextBloc;
