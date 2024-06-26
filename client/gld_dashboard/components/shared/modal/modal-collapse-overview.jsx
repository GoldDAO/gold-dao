/* eslint-disable no-nested-ternary */
import React from 'react';
import OverviewItem from '../../Home/Items/OverviewItem';

const ModalCollapseOverView = ({ overview }) => (
    <div className="mb-[55%]">
      {overview.loading ? (
        <article className="collapse w-full border-[1px] rounded-none flex justify-center">
          <section className="collapse-title flex gap-2 justify-center items-center h-20">
            <span className="loading loading-spinner"></span>
            Loading overview...
          </section>
        </article>
      ) : overview?.data?.length > 0 ? (
        overview?.data?.map((c) => <OverviewItem key={c.id} {...c} />)
      ) : (
        <article className="collapse w-full border-[1px] rounded-none flex justify-center">
          <section className="collapse-title flex gap-2 justify-center items-center h-20">
            Fail to fetch overview data. Please, retry again.
          </section>
        </article>
      )}
    </div>
);

export default ModalCollapseOverView;
