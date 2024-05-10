package problem;

import main.Config;

import java.util.HashSet;
import java.util.stream.IntStream;

import static main.Config.NUM_UMPIRES;

public class Pruner {
    private final Tree tree;
    private HashSet<Integer> prunedGames;
    private int startRoundForQ1Constraint;
    private int startRoundForQ2Constraint;
    public static int numPrunedGames = 0;

    public Pruner(Tree tree) {
        this.tree = tree;
    }

    public void initPruner(int currentRoundIndex) {
        this.startRoundForQ1Constraint = Math.max(currentRoundIndex + 1 - Config.Q1, tree.getStartRoundIndex());
        this.startRoundForQ2Constraint = Math.max(currentRoundIndex + 1 - Config.Q2, tree.getStartRoundIndex());
        this.prunedGames = new HashSet<>();
    }

    public HashSet<Integer> pruneGames(int umpire, int currentRoundIndex) {
        initPruner(currentRoundIndex);
        pruneBasedOnPreviousAssignments(umpire, currentRoundIndex);
        numPrunedGames += prunedGames.size();
        System.out.println("NumPruendGames: " + numPrunedGames);
        return prunedGames;
    }

    public void pruneBasedOnPreviousAssignments(int umpire, int roundIndex) {
        IntStream.range(0, umpire).mapToObj(uid -> Math.floorMod(tree.umpireScheduleByRound[uid][roundIndex], NUM_UMPIRES)).forEach(prunedGames::add);
    }

}
