// dfx canister call --network staging sns_rewards get_maturity_history_of_neuron '(record { neuron_id = record { id = blob "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01" }; })'

// (
//   record {
//     neuron_info = record {
//       rewarded_maturity = vec {
//         record { variant { ICP }; 12_345_678 : nat64 };
//         record { variant { OGY }; 2_135_436_787 : nat64 };
//         record { variant { WTN }; 1_254_326_537_687 : nat64 };
//         record { variant { GOLDAO }; 1_245_326_537_584_769 : nat64 };
//         record { variant { GLDT }; 213_542_653_768 : nat64 };
//         record { variant { ICP }; 213_452_562_546 : nat64 };
//         record { variant { OGY }; 124_362_356 : nat64 };
//         record { variant { WTN }; 1_246_234 : nat64 };
//       };
//       accumulated_maturity = 1_346_134 : nat64;
//       last_synced_maturity = 13_461_234 : nat64;
//       last_disburse_event_considered = opt (13_462_345 : nat64);
//     };
//     neuron_id = record {
//       id = blob "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01";
//     };
//   },
// )

// (
//   vec {
//     record {
//       0 : nat64;
//       record {
//         rewarded_maturity = vec {
//           record { variant { ICP }; 213_452_562_546 : nat64 };
//           record { variant { OGY }; 124_362_356 : nat64 };
//           record { variant { GLDT }; 213_542_653_768 : nat64 };
//           record { variant { GOLDAO }; 1_245_326_537_584_769 : nat64 };
//           record { variant { WTN }; 1_246_234 : nat64 };
//         };
//         accumulated_maturity = 1_346_134 : nat64;
//         last_synced_maturity = 13_461_234 : nat64;
//         last_disburse_event_considered = opt (13_462_345 : nat64);
//       };
//     };
//   },
// )
