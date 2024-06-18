package problem;

/**
 * Enumeration representing different branch strategies for tree traversal.
 * - DFS: Depth-First Search
 * - SHUFFLE: Random shuffling of branches (unused in the current implementation)
 * - BFS_DISTANCE: Best-First Search with distance-based ordering
 * Note: Additional BFS variants can be added as needed.
 */

public enum BranchStrategy {
    DFS,
    SHUFFLE,
    BFS_DISTANCE,
}
