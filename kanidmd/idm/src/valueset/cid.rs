use crate::be::dbvalue::DbCidV1;
use crate::prelude::*;
use crate::repl::cid::Cid;
use crate::schema::SchemaAttribute;
use crate::valueset::DbValueSetV2;
use crate::valueset::ValueSet;

use smolset::SmolSet;

#[derive(Debug, Clone)]
pub struct ValueSetCid {
    set: SmolSet<[Cid; 1]>,
}

impl ValueSetCid {
    pub fn new(u: Cid) -> Box<Self> {
        let mut set = SmolSet::new();
        set.insert(u);
        Box::new(ValueSetCid { set })
    }

    pub fn push(&mut self, u: Cid) -> bool {
        self.set.insert(u)
    }

    pub fn from_dbvs2(data: Vec<DbCidV1>) -> Result<ValueSet, OperationError> {
        let set = data
            .into_iter()
            .map(|dc| Cid {
                d_uuid: dc.domain_id,
                s_uuid: dc.server_id,
                ts: dc.timestamp,
            })
            .collect();
        Ok(Box::new(ValueSetCid { set }))
    }

    pub fn from_iter<T>(iter: T) -> Option<Box<Self>>
    where
        T: IntoIterator<Item = Cid>,
    {
        let set = iter.into_iter().collect();
        Some(Box::new(ValueSetCid { set }))
    }
}

impl ValueSetT for ValueSetCid {
    fn insert_checked(&mut self, value: Value) -> Result<bool, OperationError> {
        match value {
            Value::Cid(u) => Ok(self.set.insert(u)),
            _ => {
                debug_assert!(false);
                Err(OperationError::InvalidValueState)
            }
        }
    }

    fn clear(&mut self) {
        self.set.clear();
    }

    fn remove(&mut self, pv: &PartialValue) -> bool {
        match pv {
            PartialValue::Cid(u) => self.set.remove(u),
            _ => {
                debug_assert!(false);
                true
            }
        }
    }

    fn contains(&self, pv: &PartialValue) -> bool {
        match pv {
            PartialValue::Cid(u) => self.set.contains(u),
            _ => false,
        }
    }

    fn substring(&self, _pv: &PartialValue) -> bool {
        false
    }

    fn lessthan(&self, pv: &PartialValue) -> bool {
        match pv {
            PartialValue::Cid(c2) => self.set.iter().any(|c1| c1 < c2),
            _ => false,
        }
    }

    fn len(&self) -> usize {
        self.set.len()
    }

    fn generate_idx_eq_keys(&self) -> Vec<String> {
        Vec::with_capacity(0)
    }

    fn syntax(&self) -> SyntaxType {
        SyntaxType::Cid
    }

    fn validate(&self, _schema_attr: &SchemaAttribute) -> bool {
        true
    }

    fn to_proto_string_clone_iter(&self) -> Box<dyn Iterator<Item = String> + '_> {
        Box::new(
            self.set
                .iter()
                .map(|c| format!("{:?}_{}_{}", c.ts, c.d_uuid, c.s_uuid)),
        )
    }

    fn to_db_valueset_v2(&self) -> DbValueSetV2 {
        DbValueSetV2::Cid(
            self.set
                .iter()
                .map(|c| DbCidV1 {
                    domain_id: c.d_uuid,
                    server_id: c.s_uuid,
                    timestamp: c.ts,
                })
                .collect(),
        )
    }

    fn to_partialvalue_iter(&self) -> Box<dyn Iterator<Item = PartialValue> + '_> {
        Box::new(self.set.iter().cloned().map(|u| PartialValue::new_cid(u)))
    }

    fn to_value_iter(&self) -> Box<dyn Iterator<Item = Value> + '_> {
        Box::new(self.set.iter().cloned().map(|u| Value::new_cid(u)))
    }

    fn equal(&self, other: &ValueSet) -> bool {
        if let Some(other) = other.as_cid_set() {
            &self.set == other
        } else {
            debug_assert!(false);
            false
        }
    }

    fn merge(&mut self, other: &ValueSet) -> Result<(), OperationError> {
        if let Some(b) = other.as_cid_set() {
            mergesets!(self.set, b)
        } else {
            debug_assert!(false);
            Err(OperationError::InvalidValueState)
        }
    }

    /*
    fn to_cid_single(&self) -> Option<Cid> {
        if self.set.len() == 1 {
            self.set.iter().cloned().take(1).next()
        } else {
            None
        }
    }
    */

    fn as_cid_set(&self) -> Option<&SmolSet<[Cid; 1]>> {
        Some(&self.set)
    }

    /*
    fn as_cid_iter(&self) -> Option<Box<dyn Iterator<Item = Cid> + '_>> {
        Some(Box::new(self.set.iter().copied()))
    }
    */
}
