use super::*;

impl<L: Language> std::fmt::Debug for Variable<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UniversalVar(arg0) => write!(f, "{:?}", arg0),
            Self::ExistentialVar(arg0) => write!(f, "{:?}", arg0),
            Self::BoundVar(arg0) => write!(f, "{:?}", arg0),
        }
    }
}

impl<L: Language> std::fmt::Debug for UniversalVar<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let UniversalVar { var_index, kind } = self;
        write!(f, "!{:?}_{:?}", kind, var_index)
    }
}

impl<L: Language> std::fmt::Debug for ExistentialVar<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ExistentialVar { var_index, kind } = self;
        write!(f, "?{:?}_{:?}", kind, var_index)
    }
}

impl std::fmt::Debug for VarIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.index)
    }
}

impl<L: Language> std::fmt::Debug for BoundVar<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoundVar {
                debruijn: None,
                var_index,
                kind,
            } => write!(f, "^{:?}_{:?}", kind, var_index),
            BoundVar {
                debruijn: Some(db),
                var_index,
                kind,
            } => write!(f, "^{:?}{:?}_{:?}", kind, db.index, var_index),
        }
    }
}

impl std::fmt::Debug for DebruijnIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "^{}", self.index)
    }
}
