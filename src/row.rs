use std::rc::Rc;

use crate::value::Value;
use crate::table_error::TableError;


// playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=fbac8bab1dc26bc89edf35e6d62b3170

// playground for Row & Iterators: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=5b1ead8cdf0cbaac2941ec9e15a942d5


/// A row with ref values for a `Table` or `TableSlice`.
#[derive(Debug)]
pub struct RefRow<'a> {
    pub(crate) columns: &'a Vec<String>,
    pub(crate) row: &'a Vec<Value>
}

/// A row with mut ref values for a `Table` or `TableSlice`.
#[derive(Debug)]
pub struct MutRefRow<'a> {
    pub(crate) columns: &'a Vec<String>,
    pub(crate) row: &'a mut Vec<Value>
}

/// A owned row for a `Table` or `TableSlice`.
#[derive(Debug)]
pub struct OwnedRow {
    pub(crate) columns: Rc<Vec<String>>,
    pub(crate) row: Vec<Value>
}

//impl <'a> Row<'a> {
//    /// Create a new Row given a list of columns and the list of values.
////    pub fn new(columns :&'a Vec<String>, row :&'a Vec<Value>) -> Result<Row<'a>, TableError> {
////        if columns.len() != row.len() {
////            let err_str = format!("Length of columns does not match length of row: {} != {}", columns.len(), row.len());
////            Err(TableError::new(err_str.as_str()))
////        } else {
////            Ok(Row { columns, values: row })
////        }
////    }
//
//    /// Return the contents of a cell by column name.
//    pub fn get(&'a self, column :&str) -> Result<&'a Value, TableError> {
//        let pos = self.columns.iter().position(|c| c == column);
//
//        match pos {
//            Some(p) => Ok(&self.values[p]),
//            None => {
//                let err_str = format!("Could not find column {} in row", column);
//                Err(TableError::new(err_str.as_str()))
//            }
//        }
//    }
//
//    /// Return the contents of a cell by column index.
//    pub fn at(&'a self, index :usize) -> Result<&'a Value, TableError> {
//        if index >= self.values.len() {
//            let err_str = format!("Index {} is greater than row width {}", index, self.values.len());
//            Err(TableError::new(err_str.as_str()))
//        } else {
//            Ok(&self.values[index])
//        }
//    }
//
//    #[inline]
//    pub fn width(&self) -> usize {
//        self.values.len()
//    }
//
//    /// Return an `Iterator` over the values in the row.
//    pub fn iter(&self) -> ValueIterator {
//        unimplemented!()
//    }
//}

/// An iterator over the `Value`s in a `Row`.
pub struct ValueIterator<'a> {
    iter: core::slice::Iter<'a, Value>
}

impl <'a> Iterator for ValueIterator<'a> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
