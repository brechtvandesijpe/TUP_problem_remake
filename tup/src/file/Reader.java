package file;

import model.Instance;

import java.io.*;
import java.util.*;

public class Reader {

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