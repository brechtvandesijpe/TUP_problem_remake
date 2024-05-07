package main;

import file.Reader;

import java.io.File;
import java.io.IOException;

public class Main {
    public static void main(String[] args) throws IOException {
        System.out.println("Hello world!");

        Reader.read(new File("resources/umps8.txt"));

    }

}