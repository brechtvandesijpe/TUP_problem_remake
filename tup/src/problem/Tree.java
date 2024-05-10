package problem;

import model.Instance;

import java.util.Arrays;
import java.util.HashSet;
import java.util.Set;
import java.util.stream.IntStream;

import static main.Config.*;
import static model.Instance.getGame;
import static model.Instance.getInterStadiumDistance;

public class Tree {
    private final Instance instance;
    private final Pruner pruner;
    private final int startRoundIndex;
    private final int endRoundIndex;
    private final boolean isSub;
    public final int[][] umpireScheduleByRound;
    private final int[] gameUmpireLookup;
    private HashSet<Integer> prunedGames;
    private final int branchStart;
    private int partialDistance;

    public Tree(Instance instance, int startRoundIndex, int endRoundIndex, boolean isSub) {
        this.isSub = isSub;
        this.instance = instance;
        this.startRoundIndex = startRoundIndex;
        this.endRoundIndex = endRoundIndex;
        this.branchStart = startRoundIndex * NUM_UMPIRES;
        this.gameUmpireLookup = new int[NUM_GAMES];
        this.umpireScheduleByRound = new int[NUM_UMPIRES][NUM_ROUNDS];
        this.pruner = new Pruner(this);
    }

    public void startGlobalTraversal() {
        preventSolutionRotation();
    }

    public Set<Integer> getFeasibleAllocations(int umpire, int currentRoundIndex) {
        prunedGames = pruner.pruneGames(umpire, currentRoundIndex);
        return prunedGames;
    }

    // Algorithm 2.1: Branch-and-bound algorithm
    public void performTraversal(int umpire, int currentRoundIndex) {
        Set<Integer> sortedListOfFeasibleAllocations = getFeasibleAllocations(umpire, currentRoundIndex);
        // Iterate through each feasible allocation
        for (int a : sortedListOfFeasibleAllocations) {

        }
    }

    public void assign(int gameId, int umpire) {
        int roundIndex = getGame(gameId).getRound();
        int previousRoundIndex = roundIndex - 1;
        int gameIdPreviousRound = umpireScheduleByRound[umpire][previousRoundIndex];
        gameUmpireLookup[gameId] = umpire;
        umpireScheduleByRound[umpire][roundIndex] = gameId;
        partialDistance += getInterStadiumDistance(gameIdPreviousRound, gameId);
    }

    public void unassign(int gameId, int umpire) {
        int roundIndex = getGame(gameId).getRound();
        int previousRoundIndex = roundIndex - 1;
        int gameIdPreviousRound = umpireScheduleByRound[umpire][previousRoundIndex];
        partialDistance -= getInterStadiumDistance(gameIdPreviousRound, gameId);
    }

    public void preventSolutionRotation() {
        IntStream.range(0, NUM_UMPIRES).forEach(umpireId -> {
            int gameId = branchStart + umpireId;
            umpireScheduleByRound[umpireId][startRoundIndex] = gameId;
            gameUmpireLookup[gameId] = umpireId;
        });
        System.out.println("Fixed round " + startRoundIndex);
    }
}
