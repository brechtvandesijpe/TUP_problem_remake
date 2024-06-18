package main;

import metaheuristics.MetaType;
import problem.BranchStrategy;
import problem.LowerboundMatchType;
import subproblem.match.MatchType;


/**
 * Configuration class for the TUP.
 * This class holds all the static configuration parameters used throughout the application.
 * It includes settings for the problem instance, algorithmic strategies, and debugging options.
 */

public class Config {
    // Instance parameters
    public static int Q1;
    public static int Q2;
    public static int NUM_TEAMS; // = Total number of stadiums
    public static int NUM_ROUNDS;
    public static int NUM_UMPIRES;
    public static int NUM_GAMES;

    public static String FILE_NAME;

    // Algorithmic strategy settings
    public static boolean LOWERBOUND_ENABLED = true; // Enable/disable lower bound calculation
    public static boolean ASYNC = true; // Enable/disable asynchronous lower bound calculation
    public static BranchStrategy BRANCH_STRATEGY = BranchStrategy.BFS_DISTANCE; // Branching strategy for the Tree class
    public static MatchType MATCH_TYPE = MatchType.JONKER_VOLGENANT;  // Matching algorithm for Partial Matching & Lowerbound Matching
    public static MetaType META_HEURISTIC = MetaType.STEEPEST_DESCENT; // Unused
    public static LowerboundMatchType LB_MATCH = LowerboundMatchType.MATCH_ALGORITHM; // Lowerbound matching algorithm : 2-deep Tree search or Matching Algorithm
    public static boolean MATCH_LOWERBOUND = true; // Enable/disable 2-round matching lower bound
    public static boolean PREVENT_SOLUTION_ROTATION = true; // Fixate first round to prevent solution rotation
    public static boolean ENABLE_PARTIAL_MATCHING = true; // Enable/disable partial matching in UB & LB Tree   
    public static boolean ENABLE_HASHING = true; // Enable/disable hashing in partial matching
    public static boolean PREPRUNE_GLOBAL = false; // Prune before/after generation
    public static boolean GLOBAL_CONSTRAINT_STRAT1 = true; // Enable/disable global constraint strategy 1
    
    // Debugging and logging settings
    public static boolean DEBUG_TREE = false;
    public static boolean DEBUG_PRUNER = false;
    public static boolean DEBUG_LOWERBOUND_CALCULATOR = false;
    public static boolean DEBUG_COSTARRAY = false;
    public static boolean DEBUG_LOWERBOUND_MATCHER = false;
    public static boolean PRINT_LB_ARRAY = false;
    public static boolean PRINT_GAP = false;
    public static boolean PRINT_PRUNING_INFO = false;
    public static boolean DEBUG_MATCHER = false;
    public static boolean PRINT_HIT_RATIO = false; // Enable/disable hit ratio printing of memoization scheme in Partial Matching

    // Parameters for algorithm behavior
    public static int INFEASIBLE_WEIGHT = 10;
    public static int UNASSIGNED = -99999;

     // Console colors for logging
    public static String reset = "\u001B[0m";
    public static String green = "\u001B[32;1m";
    public static String red = "\u001B[31;1m";
    public static String blue = "\u001B[34;1m";
    public static String purple = "\u001B[35;1m";
    public static String lightGrey = "\u001B[37m";
    public static String orange = "\u001B[38;5;208m";
    public static String yellow = "\u001B[33m";
}
