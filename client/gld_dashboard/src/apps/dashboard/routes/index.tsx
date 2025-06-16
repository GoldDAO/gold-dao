import { Outlet, RouteObject } from "react-router-dom";
import { Suspense } from "react";

import Home from "./home/Home";

import { SNS } from "./sns/SNS";
import GoldDAOCanisters from "./sns/gold_dao_canisters/GoldDAOCanisters";
import GLDGovTransactions from "./sns/gldgov_transactions/GLDGovTransactions";
import GLDGovTransactionDetails from "./sns/gldgov_transaction_details/GLDGovTransactionDetails";
import GoldDAOProposals from "./sns/gold_dao_proposals/GoldDAOProposals";
import GoldDAOProposalDetails from "./sns/gold_dao_proposal_details/GoldDAOProposalDetails";
import GoldDAONeurons from "./sns/gold_dao_neurons/GoldDAONeurons";
import GoldDAONeuronDetails from "./sns/gold_dao_neuron_details/GoldDAONeuronDetails";
import GLDGovAccount from "./sns/gldgov_account/GLDGovAccount";

const routes: RouteObject[] = [
  {
    path: "dashboard",
    element: <Outlet />,
    children: [
      {
        index: true,
        element: (
          <Suspense fallback={<div>Loading...</div>}>
            <Home />
          </Suspense>
        ),
      },
      {
        path: "sns",
        children: [
          {
            index: true,
            element: (
              <Suspense fallback={<div>Loading...</div>}>
                <SNS />
              </Suspense>
            ),
          },
          {
            path: "canisters",
            element: <GoldDAOCanisters />,
          },
          {
            path: "transactions",
            children: [
              {
                index: true,
                element: <GLDGovTransactions />,
              },
              {
                path: ":id",
                element: <GLDGovTransactionDetails />,
              },
            ],
          },
          {
            path: "proposals",
            children: [
              {
                index: true,
                element: <GoldDAOProposals />,
              },
              {
                path: ":id",
                element: <GoldDAOProposalDetails />,
              },
            ],
          },
          {
            path: "neurons",
            children: [
              {
                index: true,
                element: <GoldDAONeurons />,
              },
              {
                path: ":id",
                element: <GoldDAONeuronDetails />,
              },
            ],
          },
          {
            path: "accounts/:id",
            element: <GLDGovAccount />,
          },
        ],
      },
    ],
  },
];

export default routes;
