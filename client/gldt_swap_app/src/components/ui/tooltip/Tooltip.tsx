/* eslint-disable @typescript-eslint/ban-ts-comment */
// @ts-nocheck
import { PropsWithChildren } from "react";
import { Tooltip as ReactTooltip } from "react-tooltip";
import styled from "styled-components";

interface TooltipProps
  extends PropsWithChildren<{
    className?: string;
    id: string;
    place?: string;
    clickable?: boolean;
    openOnClick?: boolean;
  }> {}

const StyledTooltip = styled(ReactTooltip)`
  background-color: rgb(var(--color-surface-3)) !important;
  opacity: 1 !important;
  color: rgb(var(--color-content)) !important;
  z-index: 9;
  border-radius: 10px !important;
`;

const Tooltip = ({
  className,
  id,
  place = "bottom",
  children,
  clickable = false,
  ...restProps
}: TooltipProps) => {
  return (
    <StyledTooltip
      className={`${className}`}
      id={id}
      place={place}
      clickable={clickable}
      delayShow={300}
      {...restProps}
    >
      {children}
    </StyledTooltip>
  );
};

export default Tooltip;
