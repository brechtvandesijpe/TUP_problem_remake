package main;

import problem.BranchStrategy;
import subproblem.match.MatchType;

public class Config {
    public static int Q1;
    public static int Q2;
    public static int NUM_TEAMS;
    public static int NUM_ROUNDS;
    public static int NUM_UMPIRES;
    public static int NUM_GAMES;

    public static String FILE_NAME;

    // SETTINGS COMPONENTS
    public static boolean LOWERBOUND_ENABLED = true;
    public static boolean ASYNC = true;
    public static BranchStrategy BRANCH_STRATEGY = BranchStrategy.BFS_DISTANCE;
    public static MatchType MATCH_TYPE = MatchType.JONKER_VOLGENANT;

    // SETTINGS DEBUGGING
    public static boolean DEBUG_TREE = false;
    public static boolean DEBUG_PRUNER = false;
    public static boolean DEBUG_LOWERBOUND_CALCULATOR = false;
    public static boolean DEBUG_COSTARRAY = true;

    // PARAMETERS
    public static int INFEASIBLE_WEIGHT = 10;
    public static int UNASSIGNED = -99999;

    // COLORS
    public static String reset = "\u001B[0m";
    public static String green = "\u001B[32;1m";
    public static String red = "\u001B[31;1m";
    public static String blue = "\u001B[34;1m";
    public static String purple = "\u001B[35;1m";
}
