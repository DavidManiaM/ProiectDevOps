package org.example.proiectlab;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
public class ProiectLabApplication {

    public static void main(String[] args) {
        loadSecrets();
        SpringApplication.run(ProiectLabApplication.class, args);
    }

    private static void loadSecrets() {
        // Load DB Password
        String dbPasswordFile = System.getenv("DB_PASSWORD_FILE");
        if (dbPasswordFile != null) {
            try {
                String password = java.nio.file.Files.readString(java.nio.file.Path.of(dbPasswordFile)).trim();
                System.setProperty("DB_PASSWORD", password);
            } catch (java.io.IOException e) {
                System.err.println("Failed to read DB password secret: " + e.getMessage());
            }
        }

        // Load JWT Secret (optional, as JwtService also handles it, but this makes it
        // available to environment)
        String jwtSecretFile = System.getenv("JWT_SECRET_FILE");
        if (jwtSecretFile != null) {
            try {
                String secret = java.nio.file.Files.readString(java.nio.file.Path.of(jwtSecretFile)).trim();
                System.setProperty("JWT_SECRET", secret);
            } catch (java.io.IOException e) {
                System.err.println("Failed to read JWT secret: " + e.getMessage());
            }
        }
    }

}
