package main;

import file.Reader;
import model.Game;
import model.Instance;
import problem.Tree;

import java.io.BufferedWriter;
import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.lang.management.ManagementFactory;
import java.lang.management.ThreadMXBean;

import static main.Config.*;


/**
 * Main class to run the TUP.
 * This class demonstrates the usage of the solver by testing it with various instances.
 * Each instance represents a different configuration of teams and constraints.
 */

public class Main {

    private void writeToCSV(String instanceFileName, double calculationTime) {
        String filePath = "calculation_times.csv";
        try (BufferedWriter writer = new BufferedWriter(new FileWriter(filePath, true))) {
            writer.write(instanceFileName + "," + String.format("%.2f", calculationTime) + "\n");
        } catch (IOException e) {
            System.err.println("Error writing to CSV: " + e.getMessage());
        }
    }

    private void writeHitRatioToCSV(String instanceFileName, double hitRatio, int accessCount) {
        String filePath = "hitratio.csv";
        try (BufferedWriter writer = new BufferedWriter(new FileWriter(filePath, true))) {
            writer.write(instanceFileName + "," + String.format("%.4f", hitRatio) + "," + accessCount  + "\n");
        } catch (IOException e) {
            System.err.println("Error writing to CSV: " + e.getMessage());
        }
    }

    public static void main(String[] args) throws IOException {
        Main main = new Main();

        System.out.println(purple +
                "/**\n" +
                " *  _______   ________  __    __   ______   __    __  __       __   ______   _______   __    __\n" +
                " * /       \\ /        |/  \\  /  | /      \\ /  |  /  |/  \\     /  | /      \\ /       \\ /  |  /  |\n" +
                " * $$$$$$$  |$$$$$$$$/ $$  \\ $$ |/$$$$$$  |$$ |  $$ |$$  \\   /$$ |/$$$$$$  |$$$$$$$  |$$ | /$$/\n" +
                " * $$ |__$$ |$$ |__    $$$  \\$$ |$$ |  $$/ $$ |__$$ |$$$  \\ /$$$ |$$ |__$$ |$$ |__$$ |$$ |/$$/ \n" +
                " * $$    $$< $$    |   $$$$  $$ |$$ |      $$    $$ |$$$$  /$$$$ |$$    $$ |$$    $$< $$  $$<  \n" +
                " * $$$$$$$  |$$$$$/    $$ $$ $$ |$$ |   __ $$$$$$$$ |$$ $$ $$/$$ |$$$$$$$$ |$$$$$$$  |$$$$$  \\ \n" +
                " * $$ |__$$ |$$ |_____ $$ |$$$$ |$$ \\__/  |$$ |  $$ |$$ |$$$/ $$ |$$ |  $$ |$$ |  $$ |$$ |$$  \\ \n" +
                " * $$    $$/ $$       |$$ | $$$ |$$    $$/ $$ |  $$ |$$ | $/  $$ |$$ |  $$ |$$ |  $$ |$$ | $$  |\n" +
                " * $$$$$$$/  $$$$$$$$/ $$/   $$/  $$$$$$/  $$/   $$/ $$/      $$/ $$/   $$/ $$/   $$/ $$/   $$/\n" +
                " */" + reset
        );

        System.out.println(blue + "------------------------------- 4 TEAMS -------------------------------" + reset);
        main.testInstance("resources/umps4.txt", 2, 1, 5176);
        System.out.println(blue + "------------------------------- 8 TEAMS -------------------------------" + reset);
        main.testInstance("resources/umps8.txt", 4, 2, 34311);
        main.testInstance("resources/umps8A.txt", 4, 2, 31490);
        main.testInstance("resources/umps8B.txt", 4, 2, 32731);
        main.testInstance("resources/umps8C.txt", 4, 2, 29879);
        System.out.println(blue + "------------------------------- 10 TEAMS -------------------------------" + reset);
        main.testInstance("resources/umps10.txt", 5, 2, 48942);
        main.testInstance("resources/umps10A.txt", 5, 2, 46551);
        main.testInstance("resources/umps10B.txt", 5, 2, 45609);
        main.testInstance("resources/umps10C.txt", 5, 2, 43149);
        System.out.println(blue + "------------------------------- 12 TEAMS -------------------------------" + reset);
        main.testInstance("resources/umps12.txt", 7, 2, 86889);
        main.testInstance("resources/umps12.txt", 5, 3, 93679);
        main.testInstance("resources/umps12.txt", 4, 3, 89826);
        System.out.println(blue + "------------------------------- 14 TEAMS -------------------------------" + reset);

        main.testInstance("resources/umps14.txt", 7, 2, 146656);
        main.testInstance("resources/umps14.txt", 8, 3, 172177);
        main.testInstance("resources/umps14.txt", 8, 2, 147824);
        main.testInstance("resources/umps14.txt", 7, 3, 164440);
        main.testInstance("resources/umps14.txt", 6, 2, 145124);
        main.testInstance("resources/umps14.txt", 6, 3, 158875);
        main.testInstance("resources/umps14.txt", 5, 3, 154962);
        main.testInstance("resources/umps14.txt", 5, 2, 143357);

        main.testInstance("resources/umps14A.txt", 8, 3, 166184);
        main.testInstance("resources/umps14A.txt", 8, 2, 143043);
        main.testInstance("resources/umps14A.txt", 7, 3, 158760);
        main.testInstance("resources/umps14A.txt", 7, 2, 140562);
        main.testInstance("resources/umps14A.txt", 6, 3, 152981);
        main.testInstance("resources/umps14A.txt", 6, 2, 138927);
        main.testInstance("resources/umps14A.txt", 5, 3, 149331);
        main.testInstance("resources/umps14A.txt", 5, 2, 137853);

        main.testInstance("resources/umps14B.txt", 8, 3, 165026);
        main.testInstance("resources/umps14B.txt", 8, 2, 141312);
        main.testInstance("resources/umps14B.txt", 7, 3, 157884);
        main.testInstance("resources/umps14B.txt", 7, 2, 138998);
        main.testInstance("resources/umps14B.txt", 6, 3, 152740);
        main.testInstance("resources/umps14B.txt", 6, 2, 138241);
        main.testInstance("resources/umps14B.txt", 5, 3, 149455);
        main.testInstance("resources/umps14B.txt", 5, 2, 136069);

        main.testInstance("resources/umps14C.txt", 8, 3, 161262);
        main.testInstance("resources/umps14C.txt", 8, 2, 141015);
        main.testInstance("resources/umps14C.txt", 7, 3, 154913);
        main.testInstance("resources/umps14C.txt", 7, 2, 138832);
        main.testInstance("resources/umps14C.txt", 6, 3, 150858);
        main.testInstance("resources/umps14C.txt", 6, 2, 136394);
        main.testInstance("resources/umps14C.txt", 5, 3, 148349);
        main.testInstance("resources/umps14C.txt", 5, 2, 134916);

        System.out.println(blue + "------------------------------- 16 TEAMS -------------------------------" + reset);
        main.testInstance("resources/umps16.txt", 7, 4, 197028);

    }

   /**
     * Tests a single instance of the TUP.
     * It reads the instance from a file, solves it, and compares the solution to an expected value.
     * The result of the test (pass/fail) is printed to the console.
     *
     * @param instanceFileName The file name of the instance to test.
     * @param Q1               The first parameter for the instance.
     * @param Q2               The second parameter for the instance.
     * @param expectedValue    The expected upper bound value of the solution.
     * @throws IOException     If there is an error reading the instance file.
     */


    private void testInstance(String instanceFileName, int Q1, int Q2, int expectedValue) throws IOException {
        // resets the settings, otherwise it would break when multiple TUP instances are tested
        Game.gameId = 0;
        Config.FILE_NAME = instanceFileName;
        Config.Q1 = Q1;
        Config.Q2 = Q2;

        ThreadMXBean threadMXBean = ManagementFactory.getThreadMXBean();
        Instance instance = Reader.read(new File(instanceFileName));
        Tree tree = new Tree(instance, 0, NUM_ROUNDS - 1, false);
        long startExecution = threadMXBean.getCurrentThreadCpuTime();
        tree.startGlobalTraversal();
        long endExecution = threadMXBean.getCurrentThreadCpuTime();

        int upperbound = tree.getShortestDistance();
        String solutionMessage = (upperbound == Integer.MAX_VALUE) ? "Infeasible" : String.valueOf(upperbound);

        if (upperbound == expectedValue) {
            // Result is the same as the expected value
            System.out.println(green + "[PASSED]  " + reset + instanceFileName + " {" + Q1 + "," + Q2 + "} : Expected " + expectedValue + ", Actual " + solutionMessage + " - " + String.format("%.2f", (endExecution - startExecution) / 1_000_000_000.0) + " sec");
        } else {
            // Result is different from the expected value
            System.out.println(red + "[FAILED]  " + reset + instanceFileName + " {" + Q1 + "," + Q2 + "} : Expected " + expectedValue + ", Actual " + solutionMessage + " - " + String.format("%.2f", (endExecution - startExecution) / 1_000_000_000.0) + " sec");
        }

        writeToCSV(instanceFileName, ((endExecution - startExecution) / 1_000_000_000.0));
        writeHitRatioToCSV(instanceFileName, tree.getMatcher().getHitRatio(), tree.getMatcher().getAccessCount());

        // Prints the diagonal LB array
        if (PRINT_LB_ARRAY) {
            System.out.println("==================================== LOWERBOUNDS ====================================");
            if (Config.LOWERBOUND_ENABLED) {
                int[][] lb = tree.getLowerboundCalculator().roundLBs;
                for (int[] ints : lb) {
                    for (int j = 0; j < lb[0].length; j++) {
                        System.out.print(ints[j] + " ");
                    }
                    System.out.println();
                }
            }
        }

        // Timing for camparisons LB Matching Strategies:
        // - 2-deep Tree search
        // - Hungarian/Jonker-Volgenant
        tree.getLowerboundCalculator().clearLBs();
        tree.getLowerboundCalculator().timeAndLogLBMatchAlgorithms();
        //tree.getMatcher().printCacheHitRatio();
    }
}
