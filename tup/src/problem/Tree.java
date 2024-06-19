package problem;

import model.Instance;

import java.text.DecimalFormat;
import java.text.SimpleDateFormat;
import java.util.*;
import java.util.concurrent.CompletableFuture;
import java.util.stream.IntStream;

import static main.Config.*;
import static model.Instance.*;
import static problem.Utility.calculateVec;

public class Tree {
    private final Instance instance;
    private LowerboundCalculator lowerboundCalculator;
    private final Pruner pruner;
    private final Matcher matcher;

    private final boolean isSub; // true if sub-tree, false if global tree

    private final int[][] solution; // current solution
    private int[][] UBSolution; // assignment of lowest distance

    private final int startRoundIndex; // 0 for global, startRound for sub
    private final int endRoundIndex; // NUM_ROUNDS - 1 for global, endRound for sub

    private int upperbound = Integer.MAX_VALUE; // best found distance is infinity
    private int lowerbound;

    public final int[][] umpireScheduleByRound;
    private final int[] gameUmpireLookup;

    private final int branchStart; // first game of the tree / sub-tree

    private HashSet<Integer> prunedGames; // pruned games by Pruner

    private int partialMatchingDistance; // distance from the Matcher's Matching Problem
    private int partialDistance; // the extra cost bc of the assignment
    private int totalDistance; // total distance of the solution
    private int eval; // infeasibility

    private final Map<BranchStrategy, Runnable> strategyMap = new HashMap<>();
    
    // debugging
    private int skips = 0;
    private int numSkippedBranches = 0;
    private int numSkippedBranchesAfterPM = 0;
    private int numSkippedBranchesBeforePM = 0;

    // for the global constraint
    public final int[][] stadiumCount = new int[NUM_UMPIRES][NUM_TEAMS];

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
        this.matcher = new Matcher(instance);
    }

    /**
     * Initiates the global traversal (all rounds)
     */

    public void startGlobalTraversal() {
        // fixates the first round to prevent rotation of the solution
        preventSolutionRotation();
        if (LOWERBOUND_ENABLED) {
            //System.out.println("wel lowerbound");
            CompletableFuture<Void> lowerboundFuture = ASYNC ? startLowerBoundCalculationAsync() : null;
            // start from the round after the fixated round
            performTraversal(0, startRoundIndex + 1);
            cancelLowerBoundCalculation(lowerboundFuture);
        } else {
            //System.out.println("geen lowerbound");
            // start from the round after the fixated round
            performTraversal(0, startRoundIndex + 1);
        }
    }

    /**
     * Initiates a sub-traversal of the tree. (from start round to end round)
     */

    public void startSubTraversal(LowerboundCalculator lowerboundCalculator) {
        this.lowerboundCalculator = lowerboundCalculator;
        // fixates the first round to prevent rotation of the solution
        preventSolutionRotation();
        // start from the round after the fixated round
        performTraversal(0, startRoundIndex + 1);
    }

    /**
     * Cancels the lower bound calculation.
     */

    public void cancelLowerBoundCalculation(CompletableFuture<Void> lowerboundFuture) {
        if (lowerboundFuture != null) {
            lowerboundFuture.cancel(true);
        }
    }

    /**
     * Initiates the lower bound calculation asynchronously. (async with the global traversal)
     */

    public CompletableFuture<Void> startLowerBoundCalculationAsync() {
        lowerboundCalculator = new LowerboundCalculator(this);
        // do the 2 round matching + LB incremental Tree search
        return CompletableFuture.runAsync(lowerboundCalculator::calculateLBs);
    }

    /**
     * Gets the feasible allocations for a given umpire and round index.
     */

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

    /**
     * Creates the game greedy distance array for a given umpire and round index.
     */

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

    /**
     * Extracts the result from the game greedy distance array.
     */

    public int[] extractResultFromGameGreedyDistance(int[][] gameGreedyDistance) {
        int[] sortedListOfFeasibleAllocations = new int[gameGreedyDistance.length];
        for (int ggd = 0; ggd < gameGreedyDistance.length; ggd++) {
            sortedListOfFeasibleAllocations[ggd] = gameGreedyDistance[ggd][0];
        }
        return sortedListOfFeasibleAllocations;
    }

    /**
     * Sorts the game greedy distance array by dist.
     */

    public void sortGameGreedyDistanceArray(int[][] gameGreedyDistance) {
        Arrays.sort(gameGreedyDistance, Comparator.comparingInt(ggd -> ggd[1]));
    }

    public void printDebugInfo() {
        if (PRINT_GAP) {
            DecimalFormat df = new DecimalFormat("0.00%");
            double gapPercentage = (double) (upperbound - lowerboundCalculator.roundLBs[0][NUM_ROUNDS - 1]) / upperbound;
            SimpleDateFormat dateFormat = new SimpleDateFormat("dd-MM-yyyy HH:mm:ss");
            String currentTimeStamp = dateFormat.format(new Date());
            System.out.println(lightGrey + "[" + currentTimeStamp + "]" + reset + " GAP: " + df.format(gapPercentage) + ", LB: " + lowerboundCalculator.roundLBs[0][NUM_ROUNDS - 1] + ", UB: " + upperbound + orange + " [UB ↓]" + reset);
        }
        if (PRINT_PRUNING_INFO) {
            printPrunedBranches();
        }
    }

    public void printPrunedBranches() {
        System.out.println("Num pruned branches before partial matching: " + numSkippedBranchesBeforePM);
        System.out.println("Num pruned branches after partial matching: " + numSkippedBranchesAfterPM);
        System.out.println("Total: " + numSkippedBranches);
    }

    /**
     * Algorithm 2.1: Branch-and-bound algorithm
     */

    public void performTraversal(int umpire, int currentRoundIndex) {
        int[] sortedListOfFeasibleAllocations = getFeasibleAllocations(umpire, currentRoundIndex);
        // Iterate through each feasible allocation
        for (int a : sortedListOfFeasibleAllocations) {
            // continue if it's not an infeasible game..
            if (a != UNASSIGNED) {
                assign(a, umpire);

                // Calculate the lower bound for the current round, untill the end round
                lowerbound = LOWERBOUND_ENABLED ? lowerboundCalculator.roundLBs[currentRoundIndex][endRoundIndex] : 0;
                if (DEBUG_TREE && LOWERBOUND_ENABLED) {
                    System.out.println("LB: {" + currentRoundIndex + " - " + endRoundIndex + "}, " + lowerbound);
                }

                //System.out.println("PartialDist: " + partialDistance + ", upperb: " + upperbound);
                if (!isPromisingBeforePartialMatch()) {
                    // no point continueing
                    unassign(a, umpire);
                    numSkippedBranches++;
                    numSkippedBranchesBeforePM++;
                    continue; // Prune the branch
                }

                // Solve the sub graph for the Partial Matching
                if (ENABLE_PARTIAL_MATCHING) {
                    // Calculate schedule
                    BitSet vec = calculateVec(umpire, currentRoundIndex, umpireScheduleByRound);
                    int subgraphSize = NUM_UMPIRES - 1 - umpire;
                    partialMatchingDistance = matcher.calculatePartialMatchingCost(vec, subgraphSize, currentRoundIndex - 1);
                }
                // System.out.println("partialMatchingDistance: " +  partialMatchingDistance);
                if (!isPromisingAfterPartialMatch()) {
                    // no point continueing
                    unassign(a, umpire);
                    numSkippedBranches++;
                    numSkippedBranchesAfterPM++;
                    continue; // Prune the branch
                }
                // We're done. (Local Search not implemented)
                if (isReadyForLocalSearch(umpire, currentRoundIndex)) {
                    IntStream.rangeClosed(branchStart, NUM_UMPIRES * (1 + endRoundIndex) - 1).forEach(g -> solution[getGame(g).getRound()][gameUmpireLookup[g]] = g);
                    if (evaluate(currentRoundIndex) < upperbound) {
                        // Better solution found. Set the UB.
                        setShortestDistance();
                        if (!isSub) {
                            // Only print gap if it's not a sub-tree.
                            printDebugInfo();
                        }
                    }
                } else {
                    // Recur to the next umpire and round
                    performTraversal(getNextUmpireId(umpire), getNextRoundIndex(umpire, currentRoundIndex));
                }

                // Unassign. This way we don't have to take copies of the assignments.
                unassign(a, umpire);
            }
        }
    }

    /**
     * Assigns a game to an umpire. (bidirectional link)
     */

    public void assign(int gameId, int umpire) {
        int roundIndex = getGame(gameId).getRound();
        int previousRoundIndex = roundIndex - 1;
        int gameIdPreviousRound = umpireScheduleByRound[umpire][previousRoundIndex];
        gameUmpireLookup[gameId] = umpire;
        umpireScheduleByRound[umpire][roundIndex] = gameId;
        partialDistance += getInterStadiumDistance(gameIdPreviousRound, gameId);
        if(!isSub) {
            stadiumCount[umpire][getGame(gameId).getHomePlayerId()]++;
        }
        // System.out.println("partial distance after assignment: " + partialDistance);
    }

    /**
     * Unassigns a game from an umpire. (bidirectional link)
     */

    public void unassign(int gameId, int umpire) {
        int roundIndex = getGame(gameId).getRound();
        int previousRoundIndex = roundIndex - 1;
        int gameIdPreviousRound = umpireScheduleByRound[umpire][previousRoundIndex];
        partialDistance -= getInterStadiumDistance(gameIdPreviousRound, gameId);
        if(!isSub) {
            stadiumCount[umpire][getGame(gameId).getHomePlayerId()]--;
        }
        // System.out.println("partial distance after unassignment: " + partialDistance);
    }

    /**
     * Prevents the rotation of solutions over rounds.
     */

    public void preventSolutionRotation() {
        if(PREVENT_SOLUTION_ROTATION) {
            IntStream.range(0, NUM_UMPIRES).forEach(umpireId -> {
                int gameId = branchStart + umpireId;
                umpireScheduleByRound[umpireId][startRoundIndex] = gameId;
                gameUmpireLookup[gameId] = umpireId;
            });
            //System.out.println("Fixed round " + startRoundIndex);
        }else{
            // don't do anything
        }
    }

    // ********** EVALUATION

    /**
     * Evaluates the solution.
     */

    public int evaluate(int roundId) {
        // Calculate the distance.
        totalDistance = IntStream.range(startRoundIndex, endRoundIndex).map(round -> IntStream.range(0, NUM_UMPIRES).map(umpireId -> {
            int nextRound = round + 1;
            int nextStadium = getGame(solution[nextRound][umpireId]).getHomePlayerId();
            int currentStadium = getGame(solution[round][umpireId]).getHomePlayerId();
            return getTravelDistanceBetween(nextStadium, currentStadium);
        }).sum()).sum();

        // Only evaluate global constraint if it's the global tree.
        if (!isSub) {
            evaluateGlobalConstraint(roundId);
        }
        // eval != 0 -> infeasible
        return eval != 0 ? Integer.MAX_VALUE : totalDistance;
    }

    /**
     * Evaluates the global constraints.
     */

    public void evaluateGlobalConstraint(int round) {
        int[][] stadiumCount = new int[NUM_UMPIRES][NUM_TEAMS];
        calculateStadiumCount(stadiumCount);
        eval = evaluateStadiumCounts(stadiumCount, round);
    }

    /**
     * Calculates the stadium count for global constr.
     */

    public void calculateStadiumCount(int[][] stadiumCount) {
        IntStream.range(0, NUM_ROUNDS).forEach(round -> IntStream.range(0, NUM_UMPIRES).forEach(umpireId -> stadiumCount[umpireId][getGame(solution[round][umpireId]).getHomePlayerId()] += 1));
    }

    /**
     * Evaluates the stadium counts against global constr.
     */

    /*
    public int evaluateStadiumCounts(int[][] stadiumCount) {
        if(GLOBAL_CONSTRAINT_STRAT1) {
            int eval = IntStream.range(0, NUM_UMPIRES).flatMap(umpireId -> IntStream.range(0, NUM_TEAMS).filter(stadium -> stadiumCount[umpireId][stadium] < 1)).map(stadium -> INFEASIBLE_WEIGHT).sum();
           // System.out.println("eval: " + eval);
            return eval;
        }else{

            return 0;
        }

    }
    */

    public int evaluateStadiumCounts(int[][] stadiumCount, int round) {

        if (GLOBAL_CONSTRAINT_STRAT1) {
            int eval = 0;
            for (int umpireId = 0; umpireId < NUM_UMPIRES; umpireId++) {
                for (int stadium = 0; stadium < NUM_TEAMS; stadium++) {
                    if (stadiumCount[umpireId][stadium] < 1) {
                        //System.out.println("round: " + round + ", endRound: " + endRoundIndex);
                        return INFEASIBLE_WEIGHT;
                    }
                }
            }
           // System.out.println("eval: " + eval);
            return eval;
        } else {
            int eval = 0;
            for (int umpireId = 0; umpireId < NUM_UMPIRES; umpireId++) {
                for (int stadium = 0; stadium < NUM_TEAMS; stadium++) {
                    if (stadiumCount[umpireId][stadium] < 1) {
                        eval += INFEASIBLE_WEIGHT;
                    }
                }
            }
        }
        return 0;
    }

    /**
     * Gets the next umpire ID if there is a next one.
     */

    public int getNextUmpireId(int currentUmpireId) {
        return isLastUmpire(currentUmpireId) ? 0 : currentUmpireId + 1;
    }

    /**
     * Gets the next round index if there is a next one.
     */

    public int getNextRoundIndex(int currentUmpireId, int currentRoundIndex) {
        return isLastUmpire(currentUmpireId) ? currentRoundIndex + 1 : currentRoundIndex;
    }

    /**
     * Checks if the traversal is ready for local search. -> solution complete
     */

    public boolean isReadyForLocalSearch(int umpireId, int currentRoundIndex) {
        // end of branch
        return isLastUmpire(umpireId) && currentRoundIndex == endRoundIndex;
    }

    public boolean isLastUmpire(int umpireId) {
        return umpireId == NUM_UMPIRES - 1;
    }

    public boolean isPromisingAfterPartialMatch() {
        return lowerbound + partialDistance + partialMatchingDistance < upperbound;
    }

    public boolean isPromisingBeforePartialMatch() {
        return lowerbound + partialDistance < upperbound;
    }

    public void setShortestDistance() {
        UBSolution = solution;
        upperbound = evaluate(-1);
    }

    public int getShortestDistance() {
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

    public Pruner getPruner() {
        return pruner;
    }

    public Matcher getMatcher() {
        return matcher;
    }

    public boolean isSub() {
        return isSub;
    }

    public int[][] getSolution() {
        return solution;
    }

    public int[][] getUBSolution() {
        return UBSolution;
    }

    public int getEndRoundIndex() {
        return endRoundIndex;
    }

    public int getLowerbound() {
        return lowerbound;
    }

    public int[][] getUmpireScheduleByRound() {
        return umpireScheduleByRound;
    }

    public int[] getGameUmpireLookup() {
        return gameUmpireLookup;
    }

    public int getBranchStart() {
        return branchStart;
    }

    public HashSet<Integer> getPrunedGames() {
        return prunedGames;
    }

    public int getPartialMatchingDistance() {
        return partialMatchingDistance;
    }

    public int getPartialDistance() {
        return partialDistance;
    }

    public int getEval() {
        return eval;
    }

    public Map<BranchStrategy, Runnable> getStrategyMap() {
        return strategyMap;
    }

    public int getSkips() {
        return skips;
    }

    public int getNumSkippedBranches() {
        return numSkippedBranches;
    }

    public int getNumSkippedBranchesAfterPM() {
        return numSkippedBranchesAfterPM;
    }

    public int getNumSkippedBranchesBeforePM() {
        return numSkippedBranchesBeforePM;
    }

    public int[][] getStadiumCount() {
        return stadiumCount;
    }

    public LowerboundCalculator getLowerboundCalculator() {
        return lowerboundCalculator;
    }
}
