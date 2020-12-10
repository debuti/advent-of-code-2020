use regex::Regex;

/*
--- Day 8: Handheld Halting ---

Your flight to the major airline hub reaches cruising altitude without incident. While you consider checking the in-flight menu for one of those drinks that come with a little umbrella, you are interrupted by the kid sitting next to you.

Their handheld game console won't turn on! They ask if you can take a look.

You narrow the problem down to a strange infinite loop in the boot code (your puzzle input) of the device. You should be able to fix it, but first you need to be able to run the code in isolation.

The boot code is represented as a text file with one instruction per line of text. Each instruction consists of an operation (acc, jmp, or nop) and an argument (a signed number like +4 or -20).

    acc increases or decreases a single global value called the accumulator by the value given in the argument. For example, acc +7 would increase the accumulator by 7. The accumulator starts at 0. After an acc instruction, the instruction immediately below it is executed next.
    jmp jumps to a new instruction relative to itself. The next instruction to execute is found using the argument as an offset from the jmp instruction; for example, jmp +2 would skip the next instruction, jmp +1 would continue to the instruction immediately below it, and jmp -20 would cause the instruction 20 lines above to be executed next.
    nop stands for No OPeration - it does nothing. The instruction immediately below it is executed next.

For example, consider the following program:

nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6

These instructions are visited in this order:

nop +0  | 1
acc +1  | 2, 8(!)
jmp +4  | 3
acc +3  | 6
jmp -3  | 7
acc -99 |
acc +1  | 4
jmp -4  | 5
acc +6  |

First, the nop +0 does nothing. Then, the accumulator is increased from 0 to 1 (acc +1) and jmp +4 sets the next instruction to the other acc +1 near the bottom. After it increases the accumulator from 1 to 2, jmp -4 executes, setting the next instruction to the only acc +3. It sets the accumulator to 5, and jmp -3 causes the program to continue back at the first acc +1.

This is an infinite loop: with this sequence of jumps, the program will run forever. The moment the program tries to run any instruction a second time, you know it will never terminate.

Immediately before the program would run an instruction a second time, the value in the accumulator is 5.

Run your copy of the boot code. Immediately before any instruction is executed a second time, what value is in the accumulator?

Your puzzle answer was 1930.
--- Part Two ---

After some careful analysis, you believe that exactly one instruction is corrupted.

Somewhere in the program, either a jmp is supposed to be a nop, or a nop is supposed to be a jmp. (No acc instructions were harmed in the corruption of this boot code.)

The program is supposed to terminate by attempting to execute an instruction immediately after the last instruction in the file. By changing exactly one jmp or nop, you can repair the boot code and make it terminate correctly.

For example, consider the same program from above:

nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6

If you change the first instruction from nop +0 to jmp +0, it would create a single-instruction infinite loop, never leaving that instruction. If you change almost any of the jmp instructions, the program will still eventually find another jmp instruction and loop forever.

However, if you change the second-to-last instruction (from jmp -4 to nop -4), the program terminates! The instructions are visited in this order:

nop +0  | 1
acc +1  | 2
jmp +4  | 3
acc +3  |
jmp -3  |
acc -99 |
acc +1  | 4
nop -4  | 5
acc +6  | 6

After the last instruction (acc +6), the program terminates by attempting to run the instruction below the last instruction in the file. With this change, after the program terminates, the accumulator contains the value 8 (acc +1, acc +1, acc +6).

Fix the program so that it terminates normally by changing exactly one jmp (to nop) or nop (to jmp). What is the value of the accumulator after the program terminates?

Your puzzle answer was 1688.
*/

fn main() {
    let data = "acc +22
acc +0
jmp +1
acc +49
jmp +203
jmp +545
acc +26
jmp +326
acc +34
acc +23
nop +93
jmp +346
nop +513
acc -5
jmp +413
jmp +560
acc -1
jmp +344
acc +44
acc -14
nop +570
acc +12
jmp +137
jmp +411
jmp +1
jmp +128
acc -4
acc +28
acc +42
jmp +206
jmp -7
nop +386
acc +9
jmp +280
jmp +63
acc +39
acc +13
acc +30
acc +19
jmp +25
jmp -9
acc +43
jmp +180
acc -16
acc -8
acc +17
acc +11
jmp +550
acc +29
acc +40
jmp -44
jmp -10
nop +425
acc -12
jmp +19
acc +38
acc +1
jmp -9
jmp +317
acc +46
acc -15
acc +10
acc -1
jmp +382
acc +3
acc +7
nop +126
jmp +378
acc +48
jmp -21
jmp +547
acc +28
jmp +266
acc -15
acc +11
acc +11
jmp +499
acc +5
acc +38
acc +17
acc -7
jmp +444
nop +357
acc +14
acc +8
acc +1
jmp +264
nop +37
acc +15
acc +4
jmp +372
acc -1
jmp +416
acc +42
acc +44
nop +58
jmp +494
acc +24
acc +8
jmp +158
acc +19
nop +384
jmp +43
acc +0
jmp +27
jmp +479
acc +37
jmp +332
acc -5
acc +49
jmp -87
acc -2
acc +41
jmp +50
acc -7
acc +8
acc -8
acc +3
jmp +68
jmp +1
acc -11
nop +117
jmp +403
jmp +348
jmp -33
jmp +1
acc +20
jmp +300
jmp +148
jmp +1
jmp +361
acc +1
acc +12
acc +42
jmp -111
acc +36
acc +1
acc +18
jmp -10
jmp +20
jmp +464
nop -89
nop +152
jmp +2
jmp +48
acc +17
acc +29
nop +100
nop -96
jmp +27
acc -3
acc +18
jmp +293
jmp +222
acc -19
acc +35
acc +46
acc +3
jmp +230
nop -147
acc +39
jmp +46
jmp +123
acc +23
acc -9
acc +33
acc +30
jmp +444
acc -8
nop +188
acc +24
jmp -113
jmp -156
acc -10
acc +30
jmp +24
acc +49
acc +33
nop -75
acc -14
jmp -52
acc +33
jmp -156
jmp +401
acc -9
jmp +14
acc +37
acc +4
jmp +37
acc +29
nop +57
jmp +243
acc +10
nop +382
acc +19
acc +13
jmp +216
acc +17
jmp +177
nop +405
nop +9
acc +43
jmp +30
nop +387
jmp -51
jmp +97
jmp +348
jmp +397
jmp +219
nop +148
acc +34
jmp -12
acc -16
acc +5
acc +33
jmp +29
acc +49
jmp +126
acc +19
acc -11
acc -11
jmp +333
acc +10
jmp -14
jmp +89
acc +0
acc +11
jmp -196
acc +33
jmp +1
acc +31
jmp +353
jmp +268
nop +170
jmp +218
jmp +90
acc -18
jmp -45
jmp -156
jmp -227
acc +5
acc -13
jmp -136
jmp +1
jmp -52
acc +24
jmp +104
nop -3
acc +16
acc +0
acc +50
jmp -7
acc +37
acc +7
acc -19
acc -14
jmp +171
acc +12
acc +42
acc -15
jmp +12
acc +21
acc +37
jmp -56
jmp +1
acc -3
jmp -147
nop -84
acc -14
acc +19
nop +221
jmp -132
acc +10
jmp +27
acc +0
jmp +250
acc +12
acc -9
acc +5
nop +263
jmp +30
jmp +1
acc +10
acc -17
jmp -27
acc +5
acc +40
acc -12
acc -7
jmp +99
acc +45
acc +3
acc +39
jmp -229
acc +50
acc +17
acc +31
jmp -12
nop -41
jmp +89
jmp -36
jmp +49
jmp +1
nop +214
acc +25
acc +23
jmp +211
nop +180
acc +45
jmp +245
acc -10
jmp +225
jmp -120
acc -4
acc +45
jmp +214
acc +6
acc +50
acc +26
jmp -180
nop +83
jmp +91
acc +37
acc +42
jmp -115
jmp +146
acc +31
jmp -144
acc -14
jmp -238
acc +43
acc +31
jmp -149
acc -19
jmp +157
acc -8
acc -16
jmp +274
acc +21
acc -14
jmp -135
acc +40
jmp -272
acc +33
acc -11
jmp -51
acc +35
acc +31
acc +14
jmp -267
acc +38
acc -16
acc +43
jmp -25
acc +37
nop +40
jmp +219
acc +23
nop -166
jmp +126
jmp -241
acc +37
acc +39
nop -187
acc +21
jmp -179
acc +32
jmp +72
acc +14
acc +0
acc +12
acc +22
jmp -15
nop -30
jmp -339
acc +21
jmp -160
acc +14
acc +17
acc -18
nop +210
jmp +110
acc +46
jmp -325
acc +27
acc -13
acc -4
jmp -259
acc -19
acc -11
acc +19
acc +36
jmp -357
nop -60
jmp +190
acc +34
acc -4
nop +20
jmp +1
jmp -152
acc +35
acc -18
jmp -77
nop -264
acc -2
acc +4
acc +4
jmp -224
nop -75
acc +6
acc -14
jmp -270
acc -14
jmp -365
acc +23
acc -19
jmp +61
acc -1
acc +7
acc +0
acc +11
jmp +176
acc +17
acc -5
acc +12
acc +38
jmp +45
jmp +1
acc +22
acc -11
acc +10
jmp -396
acc +36
jmp -280
acc +23
nop +56
acc -7
jmp -421
jmp -77
acc +31
nop -97
acc +29
jmp -401
nop -324
jmp -237
acc +24
acc +6
acc -9
jmp -337
acc -7
acc -3
jmp -445
acc +24
acc +11
acc +47
acc +47
jmp -359
acc -6
acc -6
jmp +1
jmp +1
jmp -34
acc -12
acc +34
acc +36
acc +3
jmp +11
acc -18
acc +26
acc +43
jmp -454
acc +6
acc +46
acc +45
acc +37
jmp -23
jmp -412
acc +31
acc +2
acc -9
acc +24
jmp -469
nop -114
acc -19
jmp -127
jmp -313
jmp -367
acc +0
jmp +34
acc +22
jmp -152
acc +18
acc +14
acc +43
jmp +56
nop -61
acc -14
acc +22
nop -71
jmp -408
nop -359
acc -15
acc +14
acc +5
jmp -266
acc -10
acc -14
jmp -95
acc +5
acc -11
acc +42
jmp -485
acc +0
acc +32
acc +14
acc +16
jmp +74
nop +5
jmp +1
jmp -32
acc +31
jmp -34
jmp -452
acc +15
jmp -7
acc -12
jmp +16
nop -515
jmp -404
nop +33
jmp -290
acc -5
acc +43
acc +6
acc +27
jmp -462
jmp +1
acc +37
acc +2
acc +17
jmp -220
jmp +43
acc +49
acc -10
acc -3
jmp +17
nop -523
nop -456
acc +8
jmp -396
jmp -182
nop +11
jmp +1
jmp -434
acc +36
acc +50
nop -486
acc +31
jmp -220
acc +15
acc -15
jmp -44
acc -17
acc +5
nop -332
acc +46
jmp -184
acc -12
acc +46
jmp -219
acc +27
acc +31
jmp -155
acc +44
jmp +30
nop -5
acc +11
acc +0
acc -11
jmp -455
acc +30
acc -3
acc -2
jmp -444
jmp +6
acc +44
acc +15
acc +21
acc -12
jmp -417
nop -229
jmp -494
acc -12
acc +16
acc +21
acc +5
jmp -34
nop -353
acc -19
acc +15
acc -16
jmp -448
acc +18
jmp -427
acc +43
nop -589
acc +26
jmp -297
acc +0
acc +15
jmp -249
acc +16
acc -7
jmp -337
nop -566
acc +35
jmp -471
acc -8
acc +18
nop -549
acc +15
jmp +1";
    let _test = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    let _data: Vec<_> = data.split("\n").collect();
    let _test: Vec<_> = _test.split("\n").collect();
    let code = parse_code(_data);

    //println!("{:#?}", code);

    let _ = run_code(&code);

    fix_code(&code);
}

#[derive(Debug, Clone)]
enum Opcode {
    ACC(i32),
    JMP(i32),
    NOP(i32),
}

fn parse_code(listing: Vec<&str>) -> Vec<Opcode> {
    let mut result: Vec<Opcode> = Vec::new();
    for line in &listing {
        match Regex::new(r"^(.*) (.*)$").unwrap().captures(line) {
            Some(x) => {
                let opcode = x.get(1).unwrap().as_str();
                let value = x.get(2).unwrap().as_str().parse::<i32>().unwrap();

                result.push(match opcode {
                    "acc" => Opcode::ACC(value),
                    "jmp" => Opcode::JMP(value),
                    "nop" => Opcode::NOP(value),
                    _ => unreachable!(),
                })
            }
            None => {
                println!("Parsing failed with {}", line);
                unreachable!()
            }
        }
    }
    result
}

fn run_code(code: &Vec<Opcode>) -> Result<i32, ()> {
    let mut accumulator: i32 = 0;
    let mut pc: i32 = 0;
    let mut traces: Vec<bool> = vec![false; code.len()];
    loop {
        if pc >= code.len() as i32 {
            return Result::Ok(accumulator);
        }
        let fetch = &code[pc as usize];
        if traces[pc as usize] == true {
            println!("Infinite loop detected. ACC {} PC {}", accumulator, pc);
            return Result::Err(());
        } else {
            traces[pc as usize] = true;
        }
        //println!("{:#?} {} {}", fetch, accumulator, pc);
        match fetch {
            Opcode::ACC(v) => {
                accumulator += v;
                pc += 1;
            }
            Opcode::JMP(v) => pc += v,
            Opcode::NOP(_) => pc += 1,
        }
    }
}

fn fix_code(code: &Vec<Opcode>) -> i32 {
    for idx in 0..code.len() {
        let mut modified = code.clone();
        modified[idx] = match modified[idx] {
            Opcode::ACC(v) => Opcode::ACC(v),
            Opcode::JMP(v) => Opcode::NOP(v),
            Opcode::NOP(v) => Opcode::JMP(v),
        };
        if let Result::Ok(acc) = run_code(&modified) {
            println!("Success by changing instruction {}! Acc: {}", idx, acc);
            return acc;
        }
    }
    return 0;
}
