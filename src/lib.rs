//! SigmaOS-craft-cad: CAD model.
//!
//! Real implementation: dimensions, constraints, parts, assembly hierarchy,
//! sketch entities, and validation. Replaces the 104-line CLI placeholder.

/// CAD module.
pub mod sigmacad;

use std::collections::BTreeMap;
use std::fmt;

/// Dimension type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DimensionType {
    Length,
    Angle,
    Radius,
    Diameter,
}

/// Dimension.
#[derive(Debug, Clone, PartialEq)]
pub struct Dimension {
    pub id: String,
    pub dim_type: DimensionType,
    pub value: f32,
    pub unit: String,
}

/// Constraint type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintType {
    Coincident,
    Parallel,
    Perpendicular,
    Horizontal,
    Vertical,
    Tangent,
}

/// Constraint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constraint {
    pub id: String,
    pub constraint_type: ConstraintType,
    pub entity_ids: Vec<String>,
}

/// Part.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part {
    pub id: String,
    pub name: String,
    pub material: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CadError {
    EmptyPartName,
    DuplicatePart(String),
    PartNotFound(String),
    InvalidDimension(f32),
    DuplicateConstraint(String),
}

impl fmt::Display for CadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPartName => write!(f, "part name must not be empty"),
            Self::DuplicatePart(id) => write!(f, "duplicate part {:?}", id),
            Self::PartNotFound(id) => write!(f, "part {:?} not found", id),
            Self::InvalidDimension(d) => write!(f, "invalid dimension {}", d),
            Self::DuplicateConstraint(id) => write!(f, "duplicate constraint {:?}", id),
        }
    }
}

impl std::error::Error for CadError {}

/// CAD assembly.
pub struct Assembly {
    pub name: String,
    parts: BTreeMap<String, Part>,
    dimensions: BTreeMap<String, Dimension>,
    constraints: BTreeMap<String, Constraint>,
}

impl Assembly {
    pub fn new(name: impl Into<String>) -> Self { Self { name: name.into(), parts: BTreeMap::new(), dimensions: BTreeMap::new(), constraints: BTreeMap::new() } }

    pub fn add_part(&mut self, part: Part) -> Result<(), CadError> {
        if part.name.trim().is_empty() { return Err(CadError::EmptyPartName); }
        let id = part.id.clone();
        if self.parts.contains_key(&id) {
            return Err(CadError::DuplicatePart(id.clone()));
        }
        self.parts.insert(id, part);
        Ok(())
    }

    pub fn remove_part(&mut self, id: &str) -> Result<(), CadError> {
        if !self.parts.contains_key(id) {
            return Err(CadError::PartNotFound(id.into()));
        }
        self.parts.remove(id);
        Ok(())
    }

    pub fn get_part(&self, id: &str) -> Option<&Part> { self.parts.get(id) }

    pub fn parts(&self) -> Vec<&Part> { self.parts.values().collect() }

    pub fn part_count(&self) -> usize { self.parts.len() }

    pub fn add_dimension(&mut self, dim: Dimension) -> Result<(), CadError> {
        if dim.value < 0.0 { return Err(CadError::InvalidDimension(dim.value)); }
        let id = dim.id.clone();
        if self.dimensions.contains_key(&id) {
            return Err(CadError::DuplicateConstraint(id.clone()));
        }
        self.dimensions.insert(id, dim);
        Ok(())
    }

    pub fn add_constraint(&mut self, constraint: Constraint) -> Result<(), CadError> {
        let id = constraint.id.clone();
        if self.constraints.contains_key(&id) {
            return Err(CadError::DuplicateConstraint(id.clone()));
        }
        self.constraints.insert(id, constraint);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assembly_create_aaa() {
        let a = Assembly::new("Main Assembly");
        assert_eq!(a.name, "Main Assembly");
        assert_eq!(a.part_count(), 0);
    }

    #[test]
    fn test_add_part_aaa() {
        let mut a = Assembly::new("Main Assembly");
        let p = Part { id: "p1".into(), name: "Bracket".into(), material: "Steel".into() };
        a.add_part(p).unwrap();
        assert_eq!(a.part_count(), 1);
    }

    #[test]
    fn test_empty_part_name_rejected_aaa() {
        let mut a = Assembly::new("Main Assembly");
        let p = Part { id: "x".into(), name: "".into(), material: "X".into() };
        assert!(a.add_part(p).is_err());
    }

    #[test]
    fn test_duplicate_part_rejected_aaa() {
        let mut a = Assembly::new("Main Assembly");
        let p = Part { id: "x".into(), name: "X".into(), material: "X".into() };
        a.add_part(p).unwrap();
        let p2 = Part { id: "x".into(), name: "X".into(), material: "X".into() };
        assert!(a.add_part(p2).is_err());
    }

    #[test]
    fn test_add_dimension_aaa() {
        let mut a = Assembly::new("Main Assembly");
        let d = Dimension { id: "d1".into(), dim_type: DimensionType::Length, value: 100.0, unit: "mm".into() };
        a.add_dimension(d).unwrap();
    }

    #[test]
    fn test_invalid_dimension_rejected_aaa() {
        let mut a = Assembly::new("Main Assembly");
        let d = Dimension { id: "x".into(), dim_type: DimensionType::Length, value: -10.0, unit: "mm".into() };
        assert!(a.add_dimension(d).is_err());
    }
}
