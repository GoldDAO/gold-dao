import { useState, Dispatch, SetStateAction } from "react";
import { PaginationState, SortingState } from "@tanstack/react-table";
import { useSearchParams } from "react-router-dom";

export interface TableProps {
  pagination?: PaginationState;
  setPagination?: Dispatch<SetStateAction<PaginationState>>;
  sorting?: SortingState;
  setSorting?: Dispatch<SetStateAction<SortingState>>;
}

interface PaginationStateCursor {
  pageSize: number;
  pageStart: number | undefined;
}

export const usePagination = ({
  pageSize = 10,
  pageIndex = 0,
  identifier = "",
} = {}): [PaginationState, Dispatch<SetStateAction<PaginationState>>] => {
  const [searchParams] = useSearchParams();
  const _pageSize = Number(
    searchParams.get(`page_size${identifier ?? `_${identifier}`}`)
  );
  const _pageIndex = Number(
    searchParams.get(`page_index${identifier ?? `_${identifier}`}`)
  );
  const [pagination, setPagination] = useState<PaginationState>({
    pageSize: _pageSize ? _pageSize : pageSize,
    pageIndex: _pageIndex ? _pageIndex - 1 : pageIndex,
  });
  return [pagination, setPagination];
};

export const usePaginationCursor = (
  { pageSize, pageStart } = { pageSize: 5, pageStart: undefined }
): [PaginationStateCursor, Dispatch<SetStateAction<PaginationStateCursor>>] => {
  const [searchParams] = useSearchParams();
  const _pageSize = Number(searchParams.get(`page_size`));
  const _pageStart = Number(searchParams.get(`page_start`));
  const [pagination, setPagination] = useState<PaginationStateCursor>({
    pageSize: _pageSize ? _pageSize : pageSize,
    pageStart: _pageStart ? _pageStart : pageStart,
  });
  return [pagination, setPagination];
};

export const useSorting = ({ id = "", desc = true, identifier = "" }) => {
  const [searchParams] = useSearchParams();
  const _id = searchParams.get(`id${identifier ?? `_${identifier}`}`);
  const _desc = searchParams.get(`desc${identifier ?? `_${identifier}`}`);
  const [sorting, setSorting] = useState<SortingState>([
    {
      id: _id ? _id : id,
      desc: _desc ? _desc === "true" : desc,
    },
  ]);
  return [sorting, setSorting];
};
