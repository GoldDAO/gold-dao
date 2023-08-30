import { Alert } from '@mui/material';
import React, { useState } from 'react';
import {
  IconButton,
  Snackbar,
} from '@mui/material';
import styled from 'styled-components';
import ContentCopyIcon from '@mui/icons-material/ContentCopy';
import SnackBarFeedback from '../feedback/SnackBar';


const CopyToClipboard = ({ text }) => {
  const [open, setOpen] = useState(false);

  const handleClick = () => {
    setOpen(true);
  };

  const handleClose = () => {
    setOpen(false);
  };

  return (
    <>
      <ThemedIconButton
        sx={{ cursor: 'pointer', margin: '0 2px' }}
        aria-label="Copy to clipboard"
        onClick={() => {
          navigator.clipboard.writeText(text);
          handleClick();
        }}
      >
        <ThemedContentCopyIcon />
      </ThemedIconButton>
      <SnackBarFeedback open={open} handleClose={handleClose} text="Address copied to the clipboard" />
    </>
  );
};

export default CopyToClipboard;

const CustomSnackBar = styled(Snackbar)`
  .MuiAlert-icon {
    fill: #fff;
  }
`;

const ThemedContentCopyIcon = styled(ContentCopyIcon)`
`;

const ThemedIconButton = styled(IconButton)`
  .MuiSvgIcon-root {
    width: 20px;
    height: 20px;
  }
`