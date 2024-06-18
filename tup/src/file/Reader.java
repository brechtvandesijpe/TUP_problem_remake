package file;

import model.Instance;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.util.Arrays;

public class Reader {

    /**
     * Reads data from the tup file and constructs an Instance object.
     */

    public static Instance read(File input) throws IOException {
        String str;
        int numTeams = 0;
        try (BufferedReader reader = new BufferedReader(new FileReader(input))) {
            int[][] opponents = null;
            int[][] distances = null;
            while ((str = reader.readLine()) != null) {
                if (str.contains("nTeams")) {
                    numTeams = Integer.parseInt(str.split("=")[1].split(";")[0].trim());
                }
                if (str.contains("dist")) {
                    distances = readArray(reader, numTeams);
                }
                if (str.contains("opponents")) {
                    opponents = readArray(reader, 2 * numTeams - 2);
                }
            }
            assert distances != null;
            return new Instance(opponents, distances);
        }
    }

    /**
     * Helpermethod that reads a 2D array from the provided BufferedReader.
     */

    public static int[][] readArray(BufferedReader reader, int rows) throws IOException {
        int[][] array = new int[rows][];
        for (int row = 0; row < rows; row++) {
            array[row] = Arrays.stream(reader.readLine().split("[\\[\\]\\s]+"))
                    .filter(part -> !part.isEmpty())
                    .mapToInt(Integer::parseInt)
                    .toArray();
        }

        return array;
    }

    public static void write() throws IOException {

    }

}