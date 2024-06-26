import { useEffect, useState } from 'react';

const usePagination = () => {
  const [itemsAmount, setItemsAmount] = useState(0);
  const [limit, setLimit] = useState(10);
  const [page, setPage] = useState(1);
  const [totalPages, setTotalPages] = useState(1);

  useEffect(() => {
    setTotalPages(Math.ceil(itemsAmount / limit) || 1);
  }, [itemsAmount, limit]);

  return {
    limit, setLimit, page, setPage, itemsAmount, setItemsAmount, totalPages, setTotalPages,
  };
};

export default usePagination;
