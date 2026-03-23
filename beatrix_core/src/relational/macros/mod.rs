#[macro_export]
macro_rules! impl_comparison_for_column {
    ($($t:ty),*) => {
        $(
            impl $crate::relational::column::Column<$t> {
                pub fn gt(self, other: $t) -> $crate::relational::filters::Filters<$t, $t> {
                    $crate::relational::filters::Filters {
                        column: self,
                        not: false,
                        value: other,
                        expression: $crate::relational::filters::Expression::Gt,
                    }
                }

                pub fn gte(self, other: $t) -> $crate::relational::filters::Filters<$t, $t> {
                    $crate::relational::filters::Filters {
                        column: self,
                        not: false,
                        value: other,
                        expression: $crate::relational::filters::Expression::Gte,
                    }
                }

                pub fn lt(self, other: $t) -> $crate::relational::filters::Filters<$t, $t> {
                    $crate::relational::filters::Filters {
                        column: self,
                        not: false,
                        value: other,
                        expression: $crate::relational::filters::Expression::Lt,
                    }
                }

                pub fn lte(self, other: $t) -> $crate::relational::filters::Filters<$t, $t> {
                    $crate::relational::filters::Filters {
                        column: self,
                        not: false,
                        value: other,
                        expression: $crate::relational::filters::Expression::Lte,
                    }
                }

                pub fn between(self, lower: $t, upper: $t) -> $crate::relational::filters::BetweenFilter<$t, $t> {
                    $crate::relational::filters::BetweenFilter {
                        column: self,
                        lower,
                        upper,
                        not: false,
                    }
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! impl_column_for_characters {
    ($($t:ty),*) => {
        $(
            impl Column<$t> {
                pub fn like(self, other: $t) -> $crate::relational::filters::Filters<$t, $t> {
                    $crate::relational::filters::Filters {
                        column: self,
                        not: false,
                        value: other,
                        expression: Expression::Like,
                    }
                }

                pub fn not_like(self, other: $t) -> $crate::relational::filters::Filters<$t, $t> {
                    $crate::relational::filters::Filters {
                        column: self,
                        not: true,
                        value: other,
                        expression: Expression::NotLike,
                    }
                }
            }
        )*
    };
}
