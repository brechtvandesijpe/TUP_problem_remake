package problem;

import main.Config;
import model.Instance;

import java.util.HashSet;
import java.util.stream.IntStream;

import static main.Config.NUM_UMPIRES;
import static model.Instance.determineGameForPlayer;
import static model.Instance.getGame;

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

    public void initPruner(int currentRoundIndex) {
        this.startRoundForQ1Constraint = Math.max(currentRoundIndex + 1 - Config.Q1, tree.getStartRoundIndex());
        this.startRoundForQ2Constraint = Math.max(currentRoundIndex + 1 - Config.Q2, tree.getStartRoundIndex());
        this.prunedGames = new HashSet<>();
    }

    public HashSet<Integer> pruneGames(int umpire, int currentRoundIndex) {
        initPruner(currentRoundIndex);
        pruneBasedOnQ1Constraint(umpire, currentRoundIndex);
        pruneBasedOnQ2Constraint(umpire, currentRoundIndex);
        pruneBasedOnPreviousAssignments(umpire, currentRoundIndex);
        numPrunedGames += prunedGames.size();
        return prunedGames;
    }

    public void pruneBasedOnQ1Constraint(int umpire, int roundIndex) {
        for (int r = startRoundForQ1Constraint; r < roundIndex; r++) {
            int playerId = getGame(tree.umpireScheduleByRound[umpire][r]).getHomePlayerId();
            if (isAssigned(Instance.roundStadium[roundIndex][playerId])) {
                int stadiumIndex = Math.floorMod(Instance.roundStadium[roundIndex][playerId], NUM_UMPIRES);
                if (isAssigned(stadiumIndex)) {
                    prunedGames.add(stadiumIndex);
                }
            }
        }
        numPrunedBasedAfterQ1 += prunedGames.size();
    }

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

    public void pruneBasedOnPreviousAssignments(int umpire, int roundIndex) {
        IntStream.range(0, umpire).mapToObj(uid -> Math.floorMod(tree.umpireScheduleByRound[uid][roundIndex], NUM_UMPIRES)).forEach(prunedGames::add);
        numPrunedBasedAfterPreviousAssignments += prunedGames.size();
    }

    public boolean isAssigned(int gameId) {
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
