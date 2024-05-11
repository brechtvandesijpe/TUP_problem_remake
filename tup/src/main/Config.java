package main;

public class Config {
    public static int Q1;
    public static int Q2;
    public static int NUM_TEAMS;
    public static int NUM_ROUNDS;
    public static int NUM_UMPIRES;
    public static int NUM_GAMES;

    public static String FILE_NAME;

    // SETTINGS DEBUGGING
    public static boolean DEBUG_TREE = false;

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
