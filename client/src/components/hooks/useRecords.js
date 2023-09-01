import { useEffect, useState } from "react";
import { useCanister } from "@connect2ic/react";

export const useRecords = (rowsPerPage, currentPage) => {

    const [records, setRecords] = useState([]);
    const [total, setTotal] = useState();
    const gldtCoreActor = useCanister('gldtCoreCanister')
    currentPage = currentPage > 0 ? currentPage + 1 : currentPage
    useEffect(() => {
        const fetchRecords = async () => {
            const fetchedRecords = await queryRecords(gldtCoreActor, rowsPerPage, currentPage);
            setRecords(fetchedRecords.records);
            setTotal(fetchedRecords.total)
        };
        fetchRecords();
    }, [rowsPerPage, currentPage]);
    return { records, total }
}

const queryRecords = async (actor, rowsPerPage, currentPage) => {
    const records = await Promise.resolve(actor[0].get_records({ page: [currentPage], limit: [rowsPerPage] }))
    return { records: records.data[0].reverse(), total: records.total }
}