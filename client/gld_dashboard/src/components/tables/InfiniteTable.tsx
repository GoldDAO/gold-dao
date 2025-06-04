import { useMemo, Fragment, useEffect } from "react";
import {
  useReactTable,
  getCoreRowModel,
  flexRender,
  ColumnDef,
} from "@tanstack/react-table";
import { UseInfiniteQueryResult, InfiniteData } from "@tanstack/react-query";
import { useInView } from "react-intersection-observer";

interface ColumnMeta {
  className?: string;
}

interface InfiniteTableProps<T extends object> {
  columns: (ColumnDef<T, unknown> & { meta?: ColumnMeta })[];
  infiniteQuery: UseInfiniteQueryResult<InfiniteData<{ data: T[] }>, unknown>;
}

const InfiniteTable = <T extends object>({
  columns,
  infiniteQuery,
}: InfiniteTableProps<T>) => {
  // Flatten all loaded pages
  const data = useMemo<T[]>(
    () =>
      infiniteQuery.data
        ? infiniteQuery.data.pages.flatMap((page: { data: T[] }) => page.data)
        : [],
    [infiniteQuery.data]
  );

  const table = useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
  });

  // Intersection Observer for infinite scroll
  const { ref, inView } = useInView({
    threshold: 0,
    triggerOnce: false,
  });

  // Fetch next page when bottom is in view
  useEffect(() => {
    if (
      inView &&
      infiniteQuery.hasNextPage &&
      !infiniteQuery.isFetchingNextPage
    ) {
      infiniteQuery.fetchNextPage();
    }
  }, [inView, infiniteQuery]);

  return (
    <div className="bg-surface-primary rounded-xl">
      <div className="overflow-x-auto w-full">
        <table className="table-auto w-full rounded-xl">
          <thead className="bg-surface-primary text-content/60">
            {table.getHeaderGroups().map((headerGroup) => (
              <tr key={headerGroup.id}>
                {headerGroup.headers.map((header) => (
                  <th
                    key={header.id}
                    colSpan={header.colSpan}
                    className="py-4 px-8 font-normal text-sm"
                  >
                    {header.isPlaceholder ? null : (
                      <div
                        className={`flex items-center ${
                          (header.column.columnDef.meta as ColumnMeta)
                            ?.className ?? "justify-center"
                        }`}
                      >
                        {flexRender(
                          header.column.columnDef.header,
                          header.getContext()
                        )}
                      </div>
                    )}
                  </th>
                ))}
              </tr>
            ))}
          </thead>
          <tbody>
            {table.getRowModel().rows.map((row) => (
              <Fragment key={row.id}>
                <tr className="bg-surface-primary border-b last:border-none border-border text-sm">
                  {row.getVisibleCells().map((cell) => (
                    <td
                      key={cell.id}
                      className={`px-8 py-4 overflow-hidden text-ellipsis whitespace-nowrap ${
                        (cell.column.columnDef.meta as ColumnMeta)?.className ??
                        "text-center"
                      }`}
                    >
                      {flexRender(
                        cell.column.columnDef.cell,
                        cell.getContext()
                      )}
                    </td>
                  ))}
                </tr>
              </Fragment>
            ))}
          </tbody>
        </table>

        <div ref={ref} />
      </div>
      <div className="p-1 w-full flex items-center justify-center">
        {infiniteQuery.isFetchingNextPage && <span>Loading more...</span>}
        {!infiniteQuery.hasNextPage && <span>No more data</span>}
      </div>
    </div>
  );
};

export default InfiniteTable;
