import { Search2Icon } from '@chakra-ui/icons';
import { Grid, Input, InputGroup, InputRightElement } from '@chakra-ui/react';
import React from 'react';

const SearchBar = () => {
    return (
        <Grid>
            <InputGroup
                display={'flex'}
                alignItems={'center'}
                gridColumn={['1/12', '1/12', '2/11', '3/10']}
            >
                <Input
                    size={'lg'}
                    borderRadius={'50px'}
                    px="20px"
                    placeholder="Search for a Principal, a subaccount or accountID"
                    fontSize={'16px'}
                    borderColor={'blackAlpha.500'}
                />
                <InputRightElement
                    pointerEvents="none"
                    display={'flex'}
                    alignItems={'center'}
                    justifyContent={'center'}
                    height={'100%'}
                    px="30px"
                    cursor={'pointer'}
                    zIndex={20}
                >
                    <Search2Icon color="gray.400" cursor={'pointer'} zIndex={20} />
                </InputRightElement>
            </InputGroup>
        </Grid>
    );
};

export default SearchBar;
