package problem;

import model.Instance;

import java.util.*;
import java.util.concurrent.CompletableFuture;
import java.util.stream.IntStream;

import static main.Config.*;
import static model.Instance.*;

public class Tree {

    private final Instance instance;
    private LowerboundCalculator lowerboundCalculator;
    private final Pruner pruner;
    private final boolean isSub;

    private final int[][] solution;
    private int[][] UBSolution;

    private final int startRoundIndex;
    private final int endRoundIndex;

    private int upperbound = Integer.MAX_VALUE;
    private int lowerbound;

    public final int[][] umpireScheduleByRound;
    private final int[] gameUmpireLookup;

    private final int branchStart;

    private HashSet<Integer> prunedGames;

    private int partialDistance;
    private int totalDistance;
    private int eval;

    private final Map<BranchStrategy, Runnable> strategyMap = new HashMap<>();

    public Tree(Instance instance, int startRoundIndex, int endRoundIndex, boolean isSub) {
        this.isSub = isSub;
        this.instance = instance;
        this.startRoundIndex = startRoundIndex;
        this.endRoundIndex = endRoundIndex;
        this.branchStart = startRoundIndex * NUM_UMPIRES;
        this.gameUmpireLookup = new int[NUM_GAMES];
        this.umpireScheduleByRound = new int[NUM_UMPIRES][NUM_ROUNDS];
        this.solution = new int[NUM_ROUNDS][NUM_UMPIRES];
        this.pruner = new Pruner(this);
    }

    public void startGlobalTraversal() {
        preventSolutionRotation();
        if (LOWERBOUND_ENABLED) {
            CompletableFuture<Void> lowerboundFuture = ASYNC ? startLowerBoundCalculationAsync() : null;
            performTraversal(0, startRoundIndex + 1);
            cancelLowerBoundCalculation(lowerboundFuture);
        } else {
            //System.out.println("geen lowerbound");
            performTraversal(0, startRoundIndex + 1);
        }
    }

    public void startSubTraversal(LowerboundCalculator lowerboundCalculator) {
        this.lowerboundCalculator = lowerboundCalculator;
        preventSolutionRotation();
        performTraversal(0, startRoundIndex + 1);
    }

    public void cancelLowerBoundCalculation(CompletableFuture<Void> lowerboundFuture) {
        if (lowerboundFuture != null) {
            lowerboundFuture.cancel(true);
        }
    }

    public CompletableFuture<Void> startLowerBoundCalculationAsync() {
        lowerboundCalculator = new LowerboundCalculator(this);
        return CompletableFuture.runAsync(lowerboundCalculator::calculateLBs);
    }

    public int[] getFeasibleAllocations(int umpire, int currentRoundIndex) {
        prunedGames = pruner.pruneGames(umpire, currentRoundIndex);
        if (DEBUG_PRUNER) {
            pruner.printPruningInfo();
        }
        int[][] gameGreedyDistance = createGameGreedyDistanceArray(umpire, currentRoundIndex);

        strategyMap.put(BranchStrategy.BFS_DISTANCE, () -> sortGameGreedyDistanceArray(gameGreedyDistance));
        strategyMap.put(BranchStrategy.SHUFFLE, () -> {/* todo */});
        strategyMap.put(BranchStrategy.DFS, () -> {/* no action required */});
        strategyMap.getOrDefault(BRANCH_STRATEGY, () -> {/* default action */}).run();
        return extractResultFromGameGreedyDistance(gameGreedyDistance);
    }

    public int[][] createGameGreedyDistanceArray(int umpire, int currentRoundIndex) {
        long startTime = System.currentTimeMillis();
        int[][] gameGreedyDistance = IntStream.range(0, NUM_UMPIRES).filter(umpireId -> !prunedGames.contains(umpireId)).mapToObj(umpireId -> {
            int branchStart = NUM_UMPIRES * currentRoundIndex;
            int gameCurrentRound = branchStart + umpireId;
            int previousRound = currentRoundIndex - 1;
            int gamePreviousRound = umpireScheduleByRound[umpire][previousRound];
            int greedyDistance = getInterStadiumDistance(gamePreviousRound, gameCurrentRound);
            return new int[]{gameCurrentRound, greedyDistance};
        }).toArray(int[][]::new);
        long endTime = System.currentTimeMillis();
        if (DEBUG_TREE) {
            System.out.println("Creating the greedy matrix took: " + (endTime - startTime) + " msec.");
        }
        return gameGreedyDistance;
    }

    public int[] extractResultFromGameGreedyDistance(int[][] gameGreedyDistance) {
        int[] sortedListOfFeasibleAllocations = new int[gameGreedyDistance.length];
        for (int ggd = 0; ggd < gameGreedyDistance.length; ggd++) {
            sortedListOfFeasibleAllocations[ggd] = gameGreedyDistance[ggd][0];
        }
        return sortedListOfFeasibleAllocations;
    }

    public void sortGameGreedyDistanceArray(int[][] gameGreedyDistance) {
        Arrays.sort(gameGreedyDistance, Comparator.comparingInt(ggd -> ggd[1]));
    }

    // Algorithm 2.1: Branch-and-bound algorithm
    public void performTraversal(int umpire, int currentRoundIndex) {
        int[] sortedListOfFeasibleAllocations = getFeasibleAllocations(umpire, currentRoundIndex);
        // Iterate through each feasible allocation
        for (int a : sortedListOfFeasibleAllocations) {
            if (a != UNASSIGNED) {
                assign(a, umpire);

                // Calculate the lower bound for the current round, untill the end round
                lowerbound = LOWERBOUND_ENABLED ? lowerboundCalculator.getLBOfRounds(currentRoundIndex, endRoundIndex) : 0;
                if (DEBUG_TREE && LOWERBOUND_ENABLED) {
                    System.out.println("LB: {" + currentRoundIndex + " - " + endRoundIndex + "}, " + lowerbound);
                }

                //System.out.println("PartialDist: " + partialDistance + ", upperb: " + upperbound);
                if (partialDistance + lowerbound >= upperbound) {
                    unassign(a, umpire);
                    continue; // Prune the branch
                }

                if (isReadyForLocalSearch(umpire, currentRoundIndex)) {
                    IntStream.rangeClosed(branchStart, NUM_UMPIRES * (1 + endRoundIndex) - 1).forEach(g -> solution[getGame(g).getRound()][gameUmpireLookup[g]] = g);
                    if (evaluate() < upperbound) {
                        setUpperbound();
                    }
                } else {
                    // Recur to the next umpire and round
                    performTraversal(getNextUmpireId(umpire), getNextRoundIndex(umpire, currentRoundIndex));
                }
                unassign(a, umpire);
            }
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
        //System.out.println("Fixed round " + startRoundIndex);
    }

    // ********** EVALUATION
    public int evaluate() {
        totalDistance = IntStream.range(startRoundIndex, endRoundIndex).map(round -> IntStream.range(0, NUM_UMPIRES).map(umpireId -> {
            int nextRound = round + 1;
            int nextStadium = getGame(solution[nextRound][umpireId]).getHomePlayerId();
            int currentStadium = getGame(solution[round][umpireId]).getHomePlayerId();
            return getTravelDistanceBetween(nextStadium, currentStadium);
        }).sum()).sum();

        if (!isSub) {
            evaluateGlobalConstraint();
        }
        // eval != 0 -> infeasible
        return eval != 0 ? Integer.MAX_VALUE : totalDistance;
    }

    public void evaluateGlobalConstraint() {
        int[][] stadiumCount = new int[NUM_UMPIRES][NUM_TEAMS];
        calculateStadiumCount(stadiumCount);
        eval = evaluateStadiumCounts(stadiumCount);
    }

    public void calculateStadiumCount(int[][] stadiumCount) {
        IntStream.range(0, NUM_ROUNDS).forEach(round -> IntStream.range(0, NUM_UMPIRES).forEach(umpireId -> stadiumCount[umpireId][getGame(solution[round][umpireId]).getHomePlayerId()] += 1));
    }

    public int evaluateStadiumCounts(int[][] stadiumCount) {
        return IntStream.range(0, NUM_UMPIRES).flatMap(umpireId -> IntStream.range(0, NUM_TEAMS).filter(stadium -> stadiumCount[umpireId][stadium] < 1)).map(stadium -> INFEASIBLE_WEIGHT).sum();
    }

    // ************** CHECKS

    public int getNextUmpireId(int currentUmpireId) {
        return isLastUmpire(currentUmpireId) ? 0 : currentUmpireId + 1;
    }

    public int getNextRoundIndex(int currentUmpireId, int currentRoundIndex) {
        return isLastUmpire(currentUmpireId) ? currentRoundIndex + 1 : currentRoundIndex;
    }

    public boolean isReadyForLocalSearch(int umpireId, int currentRoundIndex) {
        // end of branch
        return isLastUmpire(umpireId) && currentRoundIndex == endRoundIndex;
    }

    public boolean isLastUmpire(int umpireId) {
        return umpireId == NUM_UMPIRES - 1;
    }

    // ******** SETTERS
    public void setUpperbound() {
        UBSolution = solution;
        upperbound = evaluate();
    }

    // ******** GETTERS
    public int getUpperbound() {
        return upperbound;
    }

    public int getStartRoundIndex() {
        return startRoundIndex;
    }

    public Instance getInstance() {
        return instance;
    }

    public int getTotalDistance() {
        return totalDistance;
    }
}