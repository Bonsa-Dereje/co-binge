package com.offr.co.binge;

import java.awt.Font;

public class FontLoader {

    public static Font INTER;

    public static void loadFonts() {
        try {
            
            INTER = Font.createFont(
                    Font.TRUETYPE_FONT,
                    FontLoader.class.getResourceAsStream("/fonts/inter/Inter-Regular.otf") // corrected to .otf
            ).deriveFont(16f);

        } catch (Exception e) {
            e.printStackTrace();
            System.out.println("Failed to load Inter font, using fallback font.");

            
            INTER = new Font("Arial", Font.PLAIN, 16);
        }
    }
}