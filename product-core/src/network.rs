use std::fmt::Display;

/// The size of the hex-encoded first 4 bytes of an IOTA network's genesis
/// transaction.
const CHAIN_ID_SIZE: usize = 8;
const MAINNET_CHAIN_ID: &str = "6364aad5";
const TESTNET_CHAIN_ID: &str = "2304aa97";
const DEVNET_CHAIN_ID: &str = "e678123a";

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum NetworkImpl {
    Mainnet,
    Testnet,
    Devnet,
    // Invariant: must be 8 lowercase ASCII hex-digits.
    Custom([u8; CHAIN_ID_SIZE]),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Network(NetworkImpl);

#[allow(non_upper_case_globals)]
impl Network {
    pub const Mainnet: Self = Self(NetworkImpl::Mainnet);
    pub const Testnet: Self = Self(NetworkImpl::Testnet);
    pub const Devnet: Self = Self(NetworkImpl::Devnet);

    /// Returns a string representation of this IOTA Network.
    pub fn as_str(&self) -> &str {
        match &self.0 {
            NetworkImpl::Mainnet => "mainnet",
            NetworkImpl::Testnet => "testnet",
            NetworkImpl::Devnet => "devnet",
            NetworkImpl::Custom(net) => str::from_utf8(net).expect("ascii string"),
        }
    }

    /// Returns this network's chain ID - i.e. the hex-encoded first
    /// 4 bytes of the network's genesis transaction digest.
    pub fn as_chain_id(&self) -> &str {
        match &self.0 {
            NetworkImpl::Mainnet => MAINNET_CHAIN_ID,
            NetworkImpl::Testnet => TESTNET_CHAIN_ID,
            NetworkImpl::Devnet => DEVNET_CHAIN_ID,
            NetworkImpl::Custom(net) => str::from_utf8(net).expect("ascii string"),
        }
    }

    /// Parses the given string into an IOTA network.
    /// # Examples
    /// ```
    /// # use new_product_core::network::Network;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mainnet = Network::parse("mainnet")?;
    /// assert_eq!(mainnet, Network::Mainnet);
    ///
    /// let testnet = Network::parse("2304aa97")?;
    /// assert_eq!(testnet, Network::Testnet);
    ///
    /// let custom = Network::parse("aaaaaaaa")?;
    /// assert!(custom.is_custom());
    ///
    /// let _err = Network::parse(">:/ grrr").unwrap_err();
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn parse(network: &str) -> Result<Self, NetworkParsingError> {
        match network {
            "mainnet" | MAINNET_CHAIN_ID => Ok(Self::Mainnet),
            "testnet" | TESTNET_CHAIN_ID => Ok(Self::Testnet),
            "devnet" | DEVNET_CHAIN_ID => Ok(Self::Devnet),
            s if is_chain_id(s) => {
                use std::io::Write;
                let mut buff = [0_u8; CHAIN_ID_SIZE];
                let _ = buff.as_mut_slice().write(s.as_bytes());
                Ok(Network(NetworkImpl::Custom(buff)))
            }
            _ => Err(NetworkParsingError {}),
        }
    }

    /// Returns `true` if this network is not an official IOTA Network.
    pub const fn is_custom(&self) -> bool {
        matches!(self.0, NetworkImpl::Custom(_))
    }
}

fn is_chain_id(s: impl AsRef<[u8]>) -> bool {
    let bytes = s.as_ref();
    if bytes.len() != CHAIN_ID_SIZE {
        return false;
    }

    let Ok(s) = str::from_utf8(bytes) else {
        return false;
    };

    s.chars()
        .all(|c| c.is_ascii_lowercase() && c.is_ascii_hexdigit())
}

impl Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for Network {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Debug, thiserror::Error)]
#[error(
    "failed to parse IOTA network; valid values are `mainnet`, `testnet`, `devnet`, or an IOTA chain ID"
)]
#[non_exhaustive]
pub struct NetworkParsingError {}
