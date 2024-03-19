//! Finance Library API.
//!
//! The 'finance_api` crate includes several traits that abstract multiple objects
//! related to financial markets and analysis. This crate does not implement any
//! particular feature, but defines a public API that other libraries can import and
//! use without knowing what implementation will be behind.
//!
//! Binary crates shall choose a crate that actually implements the needed traits
//! defined herein. Thus this crate targets other libraries, so these can abstract
//! whether a stock exchange is the _NASDAQ100_ or the _S&P500_.

use std::fmt;

/// A stock market description.
///
/// The [Market] trait provides an abstract definition of the functionality that is
/// expected for an object that implements a stock Market. This trait follows
/// a facade design-pattern, hence modules can rely on the interface defined herein
/// without worrying about the final implementation for a particular stock market.
///
/// A stock market is a data object that contains information relative to a particular
/// stock exchange, such as the _NASDAQ100_ or the _IBEX35_.
///
/// Key information for every stock market includes:
/// - Opening and closing time.
/// - A collection of stocks that are exchanged in the market.
/// - Monetary unit used.
pub trait Market {
    /// Get the name of the Market, for example: _NASDAQ100_ or _IBEX35_.
    fn market_name(&self) -> &str;

    /// Get a list of the stocks included in the market.
    ///
    /// # Description
    ///
    /// This method should build a list with the ticker identifier for each stock
    /// that is included in the market.
    ///
    /// ## Returns
    ///
    /// A vector with references to the tickers.
    fn list_tickers(&self) -> Vec<&String>;

    /// Get a reference to a [Company] object included in the market.
    ///
    /// # Description
    ///
    /// This method searches for stocks identified by `name` in the market. The given
    /// name is applied in a regular expression. This means that if the `name` is too
    /// ambiguous, multiple stocks might match it. For example, if **Bank** is given as
    /// `name`, multiple stocks might match such string.
    ///
    /// ## Returns
    ///
    /// A wrapped vector with a list of references to stock descriptors (objects that
    /// implement the [Company] trait) that match `name`. `None` is returned when no
    /// stocks have been found matching `name` with their respective names.
    fn stock_by_name(&self, name: &str) -> Option<Vec<&Box<dyn Company>>>;

    /// Get a reference to a [Company] object included in the market.
    ///
    /// # Description
    ///
    /// This method searches for a stock whose ticker is equal to `ticker`. An
    /// exhaustive match is applied between `ticker` and the ticker of a Company.
    /// This means that partial tickers won't produce a match.
    ///
    /// ## Returns
    ///
    /// In contrast to the method [stock_by_name](Market::stock_by_name), this method will
    /// return a wrapped reference to an object that implements the `Company` trait
    /// whose ticker is equal to `ticker`, otherwise `None` will be returned.
    fn stock_by_ticker(&self, ticker: &str) -> Option<&Box<dyn Company>>;

    /// Get the open time of the market (UTC).
    fn open_time(&self) -> &str;

    /// Get the close time of the market (UTC).
    fn close_time(&self) -> &str;

    /// Get the currency code (ISO 4217) of the market.
    fn currency(&self) -> &str;

    /// Get a list of the stock descriptors that are included in the market.
    ///
    /// # Description
    ///
    /// This method builds a vector with references to all the stock descriptors (
    /// objects that implement the [Company] trait) that are included in the market.
    fn get_companies(&self) -> Vec<&Box<dyn Company>>;
}

/// A company description.
///
/// The [Company] trait provides a data object that gathers information for a
/// company that is included in some stock market.
///
/// A `Company` is a description of the features that any company included in a
/// `Market` shall have. It is described as a trait and follows a facade design-pattern.
///
/// Key features of any `Company` object includes:
/// - Short name: This is the name that is most often used to refer to the company
///   in a stock exchange.
/// - Full name: This is the full official name that identifies the company.
/// - ISIN: International Securities Identification Number.
/// - Ticker: The abbreviation that identifies the stock in a market.
/// - Extra ID: National markets usually add internal identifiers to the companies.
/// - Market: A reference to the [Market] in which the company is included.
pub trait Company {
    /// Get the most common name of the stock.
    fn name(&self) -> &str;

    /// Get the legal or full name of the stock.
    ///
    /// # Description
    ///
    /// This method might return `None` if a full name was not provided for a
    /// particular stock. This is common in cases in which the short name is equal
    /// to the full name.
    fn full_name(&self) -> Option<&String>;

    /// Get the [ISIN](https://en.wikipedia.org/wiki/International_Securities_Identification_Number)
    /// of a stock.
    fn isin(&self) -> &str;

    /// Get the ticker of a stock.
    fn ticker(&self) -> &str;

    /// Get the extra ID of a stock.
    ///
    /// # Description
    ///
    /// Some countries add extra identity numbers to the companies, and these are useful for
    /// checking information related to the stock in national registries. As example, companies
    /// whose headquarters are registered in Spain, have an ID number called `NIF`. The property
    /// `extra_id` allows storing this information.
    ///
    /// ## Returns
    ///
    /// `None` when no special ID is linked to the stock. An ID otherwise.
    fn extra_id(&self) -> Option<&String>;
}

impl fmt::Display for dyn Company {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.ticker(), self.name())
    }
}

impl fmt::Debug for dyn Company {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
         .field(&self.full_name())
         .field(&self.name())
         .field(&self.ticker())
         .field(&self.isin())
         .field(&self.extra_id())
         .finish()
    }
}