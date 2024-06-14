package problem;

import java.util.BitSet;
import java.util.stream.IntStream;

import static main.Config.NUM_UMPIRES;

public class Utility {

    private static final int PRIME = 31;
    private static final int MULTIPLIER = 1223;
    private static final int SET = 1231;
    private static final int UNSET = 1237;

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

    public static int blend(int round, BitSet vec) {
        int digest = 1;
        digest = mixMultiplicative(mixMultiplicative(digest, round), hashBitSet(vec));
        return digest;
    }

    public static int hashBitSet(BitSet vec) {
        int digest = 1;
        for (int b = 0; b < vec.length(); b++) {
            digest = PRIME * digest;
        }
        return digest;
    }

    public static int mixMultiplicative(int digest, int value) {
        return MULTIPLIER * digest + value;
    }


}
