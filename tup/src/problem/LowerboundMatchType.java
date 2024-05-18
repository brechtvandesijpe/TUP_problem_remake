package problem;

/**
 * Enumeration representing different lowerbound match strats.
 * - MATCH_ALGORITHM: use LAPJV, Hungarian, Backtrack, Solver..
 * - BRANCH_AND_BOUND_2_DEEP : a 2-deep branch-and-bound
 * Note: Additional Match variants can be added as needed.
 */

public enum LowerboundMatchType {
    MATCH_ALGORITHM,
    BRANCH_AND_BOUND_2_DEEP
}
