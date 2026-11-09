use crate::scan::scan;

pub fn parse(source_code: &str) -> Vec<u16> {
    let lines = source_code
        .split("\n")
        .map(|s| s.split(';').next().unwrap_or(s).trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let mut out = Vec::new();

    for line in lines {
        out.push(if let Some((opcode, parameters)) = scan(line) {
            match (opcode, parameters.as_slice()) {
                ("NOP", []) => 0x0000,                                  // NOP
                ("CLS", []) => 0x00E0,                                  // clear screen
                ("RET", []) => 0x00EE,                                  // return from subroutine
                ("JP", [nnn]) => 0x1000 | nnn,                          // Jump
                ("CALL", [nnn]) => 0x2000 | nnn,                        // Call subroutine
                ("SE", [x, nn]) => 0x3000 | (x << 8) | nn, // Skip next instruction if VX == NN
                ("SNE", [x, nn]) => 0x4000 | (x << 8) | nn, // Skip next instruction if VX != NN
                ("SEV", [x, y]) => 0x5000 | (x << 8) | (y << 4), // Skip next if VX == VY
                ("LDVN", [x, nn]) => 0x6000 | (x << 8) | nn, // VX = NN
                ("ADD", [x, nn]) => 0x7000 | (x << 8) | nn, // VX += NN
                ("LDVY", [x, y]) => 0x8000 | (x << 8) | (y << 4), // VX = VY
                ("OR", [x, y]) => 0x8001 | (x << 8) | (y << 4), // OR
                ("AND", [x, y]) => 0x8002 | (x << 8) | (y << 4), // AND
                ("XOR", [x, y]) => 0x8003 | (x << 8) | (y << 4), // XOR
                ("ADDC", [x, y]) => 0x8004 | (x << 8) | (y << 4), // VX += NN \w carry
                ("SUB", [x, y]) => 0x8005 | (x << 8) | (y << 4), // VX -= VY
                ("SHR", [x, y]) => 0x8006 | (x << 8) | (y << 4), // VX >>= 1
                ("SUBN", [x, y]) => 0x8007 | (x << 8) | (y << 4), // VX = VY - VX
                ("SHL", [x, y]) => 0x800E | (x << 8) | (y << 4), // VX <<= 1
                ("SNEV", [x, y]) => 0x9000 | (x << 8) | (y << 4), // Skip next op if VX != VY
                ("LDI", [nnn]) => 0xA000 | nnn,            // Set address pointer
                ("JPV", [nnn]) => 0xB000 | nnn,            // Jump to V0 + NNN
                ("RND", [x, nn]) => 0xC000 | (x << 8) | nn, // VX = rand & NN
                ("DRW", [x, y, n]) => 0xD000 | (x << 8) | (y << 4) | n, // Draw sprite
                ("SKP", [x]) => 0xE09E | (x << 8),         // Skip next if VX pressed
                ("SKNP", [x]) => 0xE0A1 | (x << 8),        // Skip next if VX not pressed
                ("LDDT", [x]) => 0xF007 | (x << 8),        // Read delay timer
                ("LDP", [x]) => 0xF00A | (x << 8),         // Wait for key press
                ("LDDV", [x]) => 0xF015 | (x << 8),        // Set delay timer
                ("LDSV", [x]) => 0xF018 | (x << 8),        // Set sound timer
                ("ADDI", [x]) => 0xF01E | (x << 8),        // Increment I with VX
                ("LDIF", [x]) => 0xF029 | (x << 8),        // Set i to font address
                ("BCD", [x]) => 0xF033 | (x << 8),         // BCD of VX at I
                ("LDIV", [x]) => 0xF055 | (x << 8),        // Store V0-VX to I
                ("LDVI", [x]) => 0xF065 | (x << 8),        // Load I into V0-VX
                _ => panic!("Failed to parse: {line}"),
            }
        } else {
            panic!("Failed to parse: {line}");
        });
    }

    out
}
