package problem;

import java.util.BitSet;
import java.util.stream.IntStream;

import static main.Config.NUM_UMPIRES;

public class Utility {

    public static BitSet calculateVec(int umpire, int currentRoundIndex, int[][] umpireScheduleByRound) {
        return IntStream.rangeClosed(0, umpire).mapToObj(umpireId -> {
            int previousRound = currentRoundIndex - 1;
            int previousGameId = Math.floorMod(umpireScheduleByRound[umpireId][previousRound], NUM_UMPIRES);
            int nextGameId = Math.floorMod(umpireScheduleByRound[umpireId][currentRoundIndex], NUM_UMPIRES);
            BitSet vec = new BitSet(2 * NUM_UMPIRES);
            vec.set(previousGameId);
            vec.set(NUM_UMPIRES + nextGameId);
            return vec;
        }).reduce(new BitSet(2 * NUM_UMPIRES), (acc, next) -> {
            acc.or(next);
            return acc;
        });
    }

}
