use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, Clone, CandidType)]
pub struct IcrcStandard {
    pub name: &'static str,
    pub url: &'static str,
    pub filename: &'static str,
}

pub trait SupportedStandards {
    const ICRC_STANDARDS: &'static [IcrcStandard];

    fn parse(standard: &str) -> Option<&'static IcrcStandard> {
        Self::ICRC_STANDARDS
            .iter()
            .find(|s| s.filename.trim_end_matches(".rs") == standard)
    }
}

#[allow(non_camel_case_types)]
pub mod icrc_standards {
    pub struct icrc3;
    pub struct icrc7;
    pub struct icrc10;
    pub struct icrc21;
    pub struct icrc37;
    pub struct icrc28;
}

pub struct IcrcSupportedStandards;

impl SupportedStandards for IcrcSupportedStandards {
    const ICRC_STANDARDS: &'static [IcrcStandard] = &[
        IcrcStandard {
            name: "ICRC-3",
            url: "https://github.com/dfinity/ICRC-1/blob/main/standards/ICRC-3/README.md",
            filename: "icrc3.rs",
        },
        IcrcStandard {
            name: "ICRC-7",
            url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-7/ICRC-7.md",
            filename: "icrc7.rs",
        },
        IcrcStandard {
            name: "ICRC-10",
            url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-10/ICRC-10.md",
            filename: "icrc10.rs",
        },
        IcrcStandard {
            name: "ICRC-21",
            url: "https://github.com/dfinity/wg-identity-authentication/blob/main/topics/ICRC-21/icrc_21_consent_msg.md",
            filename: "icrc21.rs",
        },
        IcrcStandard {
            name: "ICRC-37",
            url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-37/ICRC-37.md",
            filename: "icrc37.rs",
        },
        IcrcStandard {
            url: "https://github.com/dfinity/wg-identity-authentication/blob/main/topics/icrc_28_trusted_origins.md",
            name: "ICRC-28",
            filename: "icrc28.rs",
        },
    ];
}

pub mod icrc10_supported_standards {
    use super::*;

    #[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
    pub struct SupportedStandard {
        pub name: String,
        pub url: String,
    }
    pub type SupportedStandardsResponse = Vec<SupportedStandard>;

    impl From<&IcrcStandard> for SupportedStandard {
        fn from(icrc: &IcrcStandard) -> Self {
            SupportedStandard {
                name: icrc.name.to_string(),
                url: icrc.url.to_string(),
            }
        }
    }

    pub type Args = ();
    pub type Response = SupportedStandardsResponse;
}

#[macro_export]
macro_rules! generate_icrc10_supported_standards_query {
    (
        $( $standard:ident ),* $(,)?
    ) => {
        pub use utils::icrcs::icrc10;
        pub use utils::icrcs::icrc10::SupportedStandards;
        pub use icrc10::icrc10_supported_standards::Response as SupportedStandardsResponse;
        #[ic_cdk::query]
        pub fn icrc10_supported_standards() -> SupportedStandardsResponse {
            let mut vec = Vec::new();
            $(
                {
                    let _marker: icrc10::icrc_standards::$standard;

                    let name = stringify!($standard);

                    if let Some(s) = icrc10::IcrcSupportedStandards::parse(name) {
                        vec.push(icrc10::icrc10_supported_standards::SupportedStandard {
                            name: s.name.to_string(),
                            url: s.url.to_string(),
                        });
                    } else {
                        panic!("Standard '{}' has a marker but is not found in ICRC_STANDARDS", name);
                    }
                }
            )*
            vec
        }
    };
}

#[macro_export]
macro_rules! generate_icrc10_query_interface {
    () => {
        pub type Args = ();
        pub type Response = utils::icrcs::icrc10::icrc10_supported_standards::Response;
    };
}
