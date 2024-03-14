use crate::imp;
use plygui_api::types;

pub type SimpleTextAdapter = types::imp::StringVecAdapter<imp::Text>;
pub type SimpleTextTreeAdapter = types::imp::StringTupleVecAdapter<imp::Text>;
pub type SimpleTextTableAdapter = types::imp::StringTableAdapter<imp::Text>;
