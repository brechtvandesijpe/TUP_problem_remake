package problem;

import main.Config;
import model.Instance;

import java.util.HashSet;
import java.util.stream.IntStream;

import static main.Config.*;
import static model.Instance.determineGameForPlayer;
import static model.Instance.getGame;

/**
 * The Pruner class is responsible for pruning games from the scheduling possibilities
*/

public class Pruner {
    private final Tree tree;
    private HashSet<Integer> prunedGames;
    private int startRoundForQ1Constraint;
    private int startRoundForQ2Constraint;
    private int numPrunedBasedAfterQ1 = 0;
    private int numPrunedBasedAfterQ2 = 0;
    private int numPrunedBasedAfterPreviousAssignments = 0;
    public static int numPrunedGames = 0;

    public Pruner(Tree tree) {
        this.tree = tree;
    }

    /**
     * Initializes the pruner for a given round.
     */

    public void initPruner(int currentRoundIndex) {
        // check Q1 from this round and onwards
        this.startRoundForQ1Constraint = Math.max(currentRoundIndex + 1 - Config.Q1, tree.getStartRoundIndex());
        // check Q2 from this round and onwards
        this.startRoundForQ2Constraint = Math.max(currentRoundIndex + 1 - Config.Q2, tree.getStartRoundIndex());
        this.prunedGames = new HashSet<>();
    }

    /**
     * Prunes games based on specific constraints.
     */

    public HashSet<Integer> pruneGames(int umpire, int currentRoundIndex) {
        initPruner(currentRoundIndex);
        pruneBasedOnQ1Constraint(umpire, currentRoundIndex);
        pruneBasedOnQ2Constraint(umpire, currentRoundIndex);
        pruneBasedOnPreviousAssignments(umpire, currentRoundIndex);
        // Only prune global if the tree is not a sub tree, and if it's enabled in the config
        if(!tree.isSub() && PREPRUNE_GLOBAL) {
            pruneGlobal(currentRoundIndex);
        }
        numPrunedGames += prunedGames.size();
        return prunedGames;
    }


    /**
     * Prunes games globally based on the remaining rounds and stadium counts.
     */

    public void pruneGlobal(int roundIndex) {
        int[][] stadiumCounts = tree.getStadiumCount();

        for(int umpire = 0; umpire < NUM_UMPIRES-1; umpire++) {
            int numLeft = NUM_TEAMS;
            for(int stadium = 0; stadium < NUM_TEAMS-1; stadium++) {
                if(stadiumCounts[umpire][stadium] > 0) {
                    numLeft--;
                }
            }
            //System.out.println("num left: " + numLeft + ", " + (tree.getEndRoundIndex() - roundIndex));
            if(numLeft > tree.getEndRoundIndex() - roundIndex) {
                int gameId = tree.umpireScheduleByRound[umpire][roundIndex];
                //int gameId = Math.floorMod(tree.umpireScheduleByRound[umpire][roundIndex], NUM_UMPIRES);
               // System.out.println("GameId: " + gameId);
                prunedGames.add(gameId);
            }
        }

    }

    /**
     * Prunes games based on the Q1 constraint.
     */

    public void pruneBasedOnQ1Constraint(int umpire, int roundIndex) {
        for (int r = startRoundForQ1Constraint; r < roundIndex; r++) {
            int playerId = getGame(tree.umpireScheduleByRound[umpire][r]).getHomePlayerId();
             // isAssigned is false if it's an infeasible game.
            if (isAssigned(Instance.roundStadium[roundIndex][playerId])) {
                int stadiumIndex = Math.floorMod(Instance.roundStadium[roundIndex][playerId], NUM_UMPIRES);
                // isAssigned is false if it's an infeasible game.
                if (isAssigned(stadiumIndex)) {
                    prunedGames.add(stadiumIndex);
                }
            }
        }
        numPrunedBasedAfterQ1 += prunedGames.size();
    }

    /**
     * Prunes games based on the Q2 constraint.
     */

    public void pruneBasedOnQ2Constraint(int umpire, int roundIndex) {
        IntStream.range(startRoundForQ2Constraint, roundIndex).flatMap(r -> {
            int gameId = tree.umpireScheduleByRound[umpire][r];
            int homePlayerId = getGame(gameId).getHomePlayerId();
            int outPlayerId = getGame(gameId).getOutPlayerId();

            int game1 = determineGameForPlayer(roundIndex, homePlayerId);
            int game2 = determineGameForPlayer(roundIndex, outPlayerId);

            return IntStream.of(game1, game2);
        }).forEach(prunedGames::add);
        numPrunedBasedAfterQ2 += prunedGames.size();
    }

    /**
     * Prunes games based on previous assignments.
     */

    public void pruneBasedOnPreviousAssignments(int umpire, int roundIndex) {
        IntStream.range(0, umpire).mapToObj(uid -> Math.floorMod(tree.umpireScheduleByRound[uid][roundIndex], NUM_UMPIRES)).forEach(prunedGames::add);
        numPrunedBasedAfterPreviousAssignments += prunedGames.size();
    }


    /**
     * Checks if a game is assigned based on gameId
     */

    public boolean isAssigned(int gameId) {
        // negative game id -> infeasible
        return 0 <= gameId;
    }

    public void printPruningInfo() {
        System.out.println("Pruning Information:");
        System.out.println("Number of pruned games: " + numPrunedGames);
        System.out.println("Number of pruned games based after Q1 constraint: " + numPrunedBasedAfterQ1);
        System.out.println("Number of pruned games based after Q2 constraint: " + numPrunedBasedAfterQ2);
        System.out.println("Number of pruned games based after previous assignments: " + numPrunedBasedAfterPreviousAssignments);
    }
}
