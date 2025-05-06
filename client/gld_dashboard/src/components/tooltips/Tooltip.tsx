/* eslint-disable @typescript-eslint/ban-ts-comment */
// @ts-nocheck
import { PropsWithChildren } from "react";
import { Tooltip as ReactTooltip } from "react-tooltip";
import styled from "styled-components";

type TooltipProps = PropsWithChildren<{
  className?: string;
  id: string;
  place?: string;
  clickable?: boolean;
  openOnClick?: boolean;
}>;

const StyledTooltip = styled(ReactTooltip)`
  background-color: var(--color-surface-primary) !important;
  opacity: 1 !important;
  color: var(--color-content) !important;
  z-index: 100;
  border-radius: 10px !important;
  border: 1px solid var(--color-border) !important;

  &.react-tooltip__place-top > .react-tooltip-arrow {
    border-bottom: 1px solid var(--color-border) !important;
    border-right: 1px solid var(--color-border) !important;
  }

  &.react-tooltip__place-right > .react-tooltip-arrow {
    border-bottom: 1px solid var(--color-border) !important;
    border-right: 1px solid var(--color-border) !important;
  }

  &.react-tooltip__place-bottom > .react-tooltip-arrow {
    border-bottom: 1px solid var(--color-border) !important;
    border-right: 1px solid var(--color-border) !important;
  }

  &.react-tooltip__place-left > .react-tooltip-arrow {
    border-bottom: 1px solid var(--color-border) !important;
    border-right: 1px solid var(--color-border) !important;
  }
`;

const Tooltip = ({
  className,
  id,
  place = "bottom",
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
    />
  );
};

export default Tooltip;
