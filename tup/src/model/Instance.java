package model;

import static main.Config.NUM_ROUNDS;
import static main.Config.NUM_GAMES;
import static main.Config.NUM_TEAMS;
import static main.Config.NUM_UMPIRES;

public class Instance {
    private static int[][] distances;
    private static int[][] opponents;

    public Instance(int[][] opponents, int[][] distances) {
        NUM_TEAMS = distances.length;
        NUM_UMPIRES = distances.length / 2;
        NUM_ROUNDS = distances.length * 2 - 2;
        NUM_GAMES = NUM_ROUNDS * distances.length / 2;
        Instance.distances = distances;
        Instance.opponents = opponents;
    }

}
