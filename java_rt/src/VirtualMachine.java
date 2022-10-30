/**
 * A simple implementation of a slightly modified MIPSr3 instruction set in java
 * (the main difference is lack of FPU, 64bit mode, paging, Branch delay slot and memory delay slot)
 *
 * Information about the ISA is found here https://s3-eu-west-1.amazonaws.com/downloads-mips/documents/MD00086-2B-MIPS32BIS-AFP-05.04.pdf
 * And here https://s3-eu-west-1.amazonaws.com/downloads-mips/documents/MD00565-2B-MIPS32-QRC-01.01.pdf
 */
public class VirtualMachine {

    // Program counter
    protected int pc = 0;
    // Special low register
    protected int low = 0;
    // Special high register
    protected int high = 0;
    // General purpose registers
    protected int[] registers = new int[32];
    // VM's working memory
    protected volatile int[] memory = new int[0];
    protected boolean running = false;

    protected long instructionsRan = 0;

    // this can only have 2 valid values 0 or 1 (yes I know it should be a bool but this is cheaper)
    // this is used for atomic read and writes any modification to memory should set this to 0
    protected volatile byte LLVal = 0;

    // The interface to the outside world brought to you by system calls. (and kinda sorta some breakpoints)
    protected volatile VirtualInterface v_interface;

    public void reset(){
        this.registers = new int[32];
        this.low = 0;
        this.high = 0;
        this.pc = 0;
        this.LLVal = 0;
        this.instructionsRan = 0;
    }

    public void run() throws Exception{
        try{

            this.running = true;
            int t;
            int s;
            int d;
            int a;
            int ZEi;
            int SEi;
            int SEa;
            int id;

            while (this.running){
                int opCode = memory[this.pc >> 2];
                this.pc += 4;

                switch (opCode >>> 26) {
                    case 0:
                        {
                           switch (opCode & 0b111111){
                               // REGISTER formatted instructions

                               //special
                               case 0b001111:
                                   break;

                                //arithmetic
                               case 0b100000:
                                   //ADD
                                   s = (opCode >> 21) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   d = (opCode >> 11) & 0b11111;
                                   try{
                                       this.registers[d] = Math.addExact(this.registers[s], this.registers[t]);
                                   }catch (ArithmeticException e){
                                       this.registers[d] = 0;
                                       throw e;
                                   }
                                   break;
                               case 0b100001:
                                   //ADDU
                                   s = (opCode >> 21) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   d = (opCode >> 11) & 0b11111;
                                   this.registers[d] = this.registers[s] + this.registers[t];
                                   break;
                               case 0b100100:
                                   //AND
                                   s = (opCode >> 21) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   d = (opCode >> 11) & 0b11111;
                                   this.registers[d] = this.registers[s] & this.registers[t];
                                   break;
                               case 0b011010:
                                   //DIV
                                   s = (opCode >> 21) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   this.low = this.registers[s] / this.registers[t];
                                   this.high = this.registers[s] % this.registers[t];
                                   break;
                               case 0b011011:
                                   //DIVU
                                   s = (opCode >> 21) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   {
                                       long tt = ((long)this.registers[t]) & 0xFFFFFFFFL;
                                       long ts = ((long)this.registers[s]) & 0xFFFFFFFFL;
                                       this.low = (int)(ts / tt);
                                       this.high = (int)(ts % tt);
                                   }
                                   break;
                               case 0b011000:
                                   //MULT
                                   s = (opCode >> 21) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   {
                                       long res = ((long)this.registers[t]) * ((long)this.registers[s]);
                                       this.low = (int)(res & 0xFFFFFFFFL);
                                       this.high = (int)(res >>> 32);
                                   }
                                   break;
                               case 0b011001:
                                   //MULTU
                                   s = (opCode >> 21) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   {
                                       long res = (((long)this.registers[t]) & 0xFFFFFFFFL) * (((long)this.registers[s]) & 0xFFFFFFFFL);
                                       this.low = (int)(res & 0xFFFFFFFFL);
                                       this.high = (int)(res >>> 32);
                                   }
                                   break;
                               case 0b100111:
                                   //NOR
                                   s = (opCode >> 21) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   d = (opCode >> 11) & 0b11111;
                                   this.registers[d] = ~(this.registers[s] | this.registers[t]);
                                   break;
                               case 0b100101:
                                   //OR
                                   s = (opCode >> 21) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   d = (opCode >> 11) & 0b11111;
                                   this.registers[d] = this.registers[s] | this.registers[t];
                                   break;
                               case 0b100110:
                                   //XOR
                                   s = (opCode >> 21) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   d = (opCode >> 11) & 0b11111;
                                   this.registers[d] = this.registers[s] ^ this.registers[t];
                                   break;
                               case 0b000000:
                                   //SLL
                                   a = (opCode >> 6) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   d = (opCode >> 11) & 0b11111;
                                   this.registers[d] = this.registers[t] << a;
                                   break;
                               case 0b000100:
                                   //SLLV
                                   s = (opCode >> 21) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   d = (opCode >> 11) & 0b11111;
                                   this.registers[d] = this.registers[t] << (this.registers[s] & 0b11111);
                                   break;
                               case 0b000011:
                                   //SRA
                                   a = (opCode >> 6) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   d = (opCode >> 11) & 0b11111;
                                   this.registers[d] = this.registers[t] >> a;
                                   break;
                               case 0b000111:
                                   //SRAV
                                   s = (opCode >> 21) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   d = (opCode >> 11) & 0b11111;
                                   this.registers[d] = this.registers[t] >> (this.registers[s] & 0b11111);
                                   break;
                               case 0b000010:
                                   //SRL
                                   a = (opCode >> 6) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   d = (opCode >> 11) & 0b11111;
                                   this.registers[d] = this.registers[t] >>> a;
                                   break;
                               case 0b000110:
                                   //SRLV
                                   s = (opCode >> 21) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   d = (opCode >> 11) & 0b11111;
                                   this.registers[d] = this.registers[t] >>> (this.registers[s]);
                                   break;
                               case 0b100010:
                                   //SUB
                                   s = (opCode >> 21) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   d = (opCode >> 11) & 0b11111;
                                   try{
                                       this.registers[d] = Math.subtractExact(this.registers[s], this.registers[t]);
                                   }catch (ArithmeticException e){
                                       this.registers[d] = 0;
                                       throw e;
                                   }
                                   break;
                               case 0b100011:
                                   //SUBU
                                   s = (opCode >> 21) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   d = (opCode >> 11) & 0b11111;
                                   this.registers[d] = this.registers[s] - this.registers[t];
                                   break;

                                //comparason
                               case 0b101010:
                                   //SLT
                                   s = (opCode >> 21) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   d = (opCode >> 11) & 0b11111;
                                   this.registers[d] = this.registers[s] < this.registers[t] ? 1 : 0;
                                   break;
                               case 0b101011:
                                   //SLTU
                                   s = (opCode >> 21) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   d = (opCode >> 11) & 0b11111;
                                   this.registers[d] = (
                                           (((long) this.registers[s]) & 0xFFFFFFFFL) < (((long) this.registers[t]) & 0xFFFFFFFFL)
                                   ) ? 1 : 0;
                                   break;

                                //jump
                               case 0b001001:
                                   //JALR
                                   s = (opCode >> 21) & 0b11111;
                                   this.registers[31] = this.pc;
                                   this.pc = this.registers[s];
                                   break;
                               case 0b001000:
                                   //JR
                                   s = (opCode >> 21) & 0b11111;
                                   this.pc = this.registers[s];
                                   break;

                               //data movement
                               case 0b010000:
                                   //MFHI
                                   d = (opCode >> 11) & 0b11111;
                                   this.registers[d] = this.high;
                                   break;
                               case 0b010010:
                                   //MFLO
                                   d = (opCode >> 11) & 0b11111;
                                   this.registers[d] = this.low;
                                   break;
                               case 0b010001:
                                   //MTHI
                                   s = (opCode >> 21) & 0b11111;
                                   this.high = this.registers[s];
                                   break;
                               case 0b010011:
                                   //MTLO
                                   s = (opCode >> 21) & 0b11111;
                                   this.low = this.registers[s];
                                   break;

                                //special
                               case 0b001100:
                                   //syscall
                                    id = (opCode >> 6) & 0b11111111111111111111;
                                    this.v_interface.system_call(this, id);
                                   break;
                               case 0b001101:
                                   //breakpoint
                                   id = (opCode >> 6) & 0b11111111111111111111;
                                   this.v_interface.breakpoint(this, id);
                                    break;
                               case 0b110100:
                                   //TEQ
                                   s = (opCode >> 21) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   if (this.registers[s] == this.registers[t]) {
                                       id = (opCode >> 6) & 0b1111111111;
                                       this.v_interface.system_call(this, id);
                                   }
                                   break;
                               case 0b110000:
                                   //TGE
                                   s = (opCode >> 21) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   if (this.registers[s] >= this.registers[t]) {
                                       id = (opCode >> 6) & 0b1111111111;
                                       this.v_interface.system_call(this, id);
                                   }
                                   break;
                               case 0b110001:
                                   //TGEU
                                   s = (opCode >> 21) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   if ((((long)this.registers[s]) & 0xFFFFFFFFL) >= ((((long)this.registers[t]) & 0xFFFFFFFFL))) {
                                       id = (opCode >> 6) & 0b1111111111;
                                       this.v_interface.system_call(this, id);
                                   }
                                   break;
                               case 0b110010:
                                   //TIT
                                   s = (opCode >> 21) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   if (this.registers[s] < this.registers[t]) {
                                       id = (opCode >> 6) & 0b1111111111;
                                       this.v_interface.system_call(this, id);
                                   }
                                   break;
                               case 0b110011:
                                   //TITU
                                   s = (opCode >> 21) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   if ((((long)this.registers[s]) & 0xFFFFFFFFL) < ((((long)this.registers[t]) & 0xFFFFFFFFL))) {
                                       id = (opCode >> 6) & 0b1111111111;
                                       this.v_interface.system_call(this, id);
                                   }
                                   break;
                               case 0b110110:
                                   //TNE
                                   s = (opCode >> 21) & 0b11111;
                                   t = (opCode >> 16) & 0b11111;
                                   if (this.registers[s] != this.registers[t]) {
                                       id = (opCode >> 6) & 0b1111111111;
                                       this.v_interface.system_call(this, id);
                                   }
                                   break;
                               default:
                                   throw new Exception("Invalid Instruction: " + opCode + " at: " + (this.pc - 4));
                           }
                        }
                        break;
                    //Jump instructions
                    case 0b000010:
                        //jump
                        this.pc = (this.pc & 0b11110000000000000000000000000000) | ((opCode & 0b00000011111111111111111111111111) << 2);
                        break;
                    case 0b000011:
                        //jal
                        this.registers[31] = this.pc;
                        this.pc = (this.pc & 0b11110000000000000000000000000000) | ((opCode & 0b00000011111111111111111111111111) << 2);
                        break;

                    // IMMEDIATE formmated instructions
                    // arthmetic
                    case 0b001000:
                        //ADDI
                        s = (opCode >>> 21) & 0B11111;
                        t = (opCode >>> 16) & 0B11111;
                        SEi = (opCode << 16) >> 16;
                        try{
                            this.registers[t] = Math.addExact(this.registers[s], SEi);
                        }catch (ArithmeticException e){
                            this.registers[t] = 0;
                            throw e;
                        }
                        break;
                    case 0b001001:
                        //ADDIU
                        s = (opCode >>> 21) & 0B11111;
                        t = (opCode >>> 16) & 0B11111;
                        SEi = (opCode << 16) >> 16;
                        this.registers[t] = this.registers[s] + SEi;
                        break;
                    case 0b001100:
                        //ANDI
                        s = (opCode >>> 21) & 0B11111;
                        t = (opCode >>> 16) & 0B11111;
                        ZEi = (opCode << 16) >>> 16;
                        this.registers[t] = this.registers[s] & ZEi;
                        break;
                    case 0b001101:
                        //ORI
                        s = (opCode >>> 21) & 0B11111;
                        t = (opCode >>> 16) & 0B11111;
                        ZEi = (opCode << 16) >>> 16;
                        this.registers[t] = this.registers[s] | ZEi;
                        break;
                    case 0b001110:
                        //XORI
                        s = (opCode >>> 21) & 0B11111;
                        t = (opCode >>> 16) & 0B11111;
                        ZEi = (opCode << 16) >>> 16;
                        this.registers[t] = this.registers[s] ^ ZEi;
                        break;

                    // constant manupulating inctructions
                    case 0b001111:
                        //LUI
                        t = (opCode >>> 16) & 0B11111;
                        //ZEi = (opCode << 16) >>> 16; this is redundant
                        this.registers[t] = opCode << 16;
                        break;

                    // comparison Instructions
                    case 0b001010:
                        //SLTI
                        s = (opCode >>> 21) & 0B11111;
                        t = (opCode >>> 16) & 0B11111;
                        SEi = (opCode << 16) >> 16;
                        this.registers[t] = this.registers[s] < SEi ? 1: 0;
                        break;
                    case 0b001011:
                        //SLTIU
                        s = (opCode >>> 21) & 0B11111;
                        t = (opCode >>> 16) & 0B11111;
                        SEi = (opCode << 16) >> 16;
                        this.registers[t] = (
                                (((long)this.registers[s]) & 0xFFFFFFFFL) < (((long)SEi) & 0xFFFFFFFFL)
                                ) ? 1: 0;
                        break;

                     // branch instructions
                    case 0b000100:
                        //BEQ
                        s = (opCode >>> 21) & 0B11111;
                        t = (opCode >>> 16) & 0B11111;
                        SEa = (opCode << 16) >> 14;
                        if (this.registers[s] == this.registers[t]) {
                            this.pc += SEa;
                        }else{
                            this.pc += 4;
                        }
                        break;
                    case 0b000001:
                        t = (opCode >>> 16) & 0b11111;
                        s = (opCode >>> 21) & 0B11111;
                        SEa = (opCode << 16) >> 14;
                        if (t == 0b00001){
                            //BGEZ
                            if (this.registers[s] >= 0){
                                this.pc += SEa;
                            }else{
                                this.pc += 4;
                            }
                        }else if (t == 0b00000){
                            //BLTZ
                            if (this.registers[s] < 0){
                                this.pc += SEa;
                            }else{
                                this.pc += 4;
                            }
                        }else if (t == 0b10001){
                            this.registers[31] = this.pc;
                            this.pc += SEa;
                        }else{
                            throw new Exception("Invalid OpCode: " + opCode + " at: " + (this.pc - 4));
                        }
                        break;
                    case 0b000111:
                        //BGTZ
                        s = (opCode >>> 21) & 0B11111;
                        SEa = (opCode << 16) >> 14;
                        if (this.registers[s] > 0) {
                            this.pc += SEa;
                        }else{
                            this.pc += 4;
                        }
                        break;
                    case 0b000110:
                        //BLEZ
                        s = (opCode >>> 21) & 0B11111;
                        SEa = (opCode << 16) >> 14;
                        if (this.registers[s] <= 0) {
                            this.pc += SEa;
                        }else{
                            this.pc += 4;
                        }
                        break;
                    case 0b000101:
                        //BNE
                        s = (opCode >>> 21) & 0B11111;
                        t = (opCode >>> 16) & 0B11111;
                        SEa = (opCode << 16) >> 14;
                        if (this.registers[s] != this.registers[t]) {
                            this.pc += SEa;
                        }else{
                            this.pc += 4;
                        }
                        break;

                    //load unaligned instructions
                    case 0b100010:
                        //LWL
                        {
                            s = (opCode >>> 21) & 0B11111;
                            t = (opCode >>> 16) & 0B11111;
                            SEi = (opCode << 16) >> 16;
                            int address = this.registers[s] + SEi;
                            byte b1 = this.getByte(address);
                            byte b2 = this.getByte(address + 1);
                            this.registers[t] &= 0x0000FFFF;
                            this.registers[t] |= (((int)b1) & 0xFF) << 24;
                            this.registers[t] |= (((int)b2) & 0xFF) << 16;
                        }
                        break;
                    case 0b100110:
                        //LWR
                        {
                            s = (opCode >>> 21) & 0B11111;
                            t = (opCode >>> 16) & 0B11111;
                            SEi = (opCode << 16) >> 16;
                            int address = this.registers[s] + SEi;
                            byte b1 = this.getByte(address);
                            byte b2 = this.getByte(address - 1);
                            this.registers[t] &= 0xFFFF0000;
                            this.registers[t] |= (((int)b1) & 0xFF);
                            this.registers[t] |= (((int)b2) & 0xFF) << 8;
                        }
                        break;

                    //save unaligned instructions
                    case 0b101010:
                        //SWL
                        {
                            s = (opCode >>> 21) & 0B11111;
                            t = (opCode >>> 16) & 0B11111;
                            SEi = (opCode << 16) >> 16;
                            int address = this.registers[s] + SEi;
                            this.LLVal = 0;
                            this.setByte(address, (byte)(this.registers[t] >> 24));
                            this.setByte(address + 1, (byte)(this.registers[t] >> 16));
                        }
                        break;
                    case 0b101110:
                        //SWR
                        {
                            s = (opCode >>> 21) & 0B11111;
                            t = (opCode >>> 16) & 0B11111;
                            SEi = (opCode << 16) >> 16;
                            int address = this.registers[s] + SEi;
                            this.LLVal = 0;
                            this.setByte(address, (byte)(this.registers[t]));
                            this.setByte(address - 1, (byte)(this.registers[t] >> 8));
                        }
                        break;

                    // load instructions
                    case 0b100000:
                        //LB
                        s = (opCode >>> 21) & 0B11111;
                        t = (opCode >>> 16) & 0B11111;
                        SEi = (opCode << 16) >> 16;
                        this.registers[t] = this.getByte(this.registers[s] + SEi);
                        break;
                    case 0b100100:
                        //LBU
                        s = (opCode >>> 21) & 0B11111;
                        t = (opCode >>> 16) & 0B11111;
                        SEi = (opCode << 16) >> 16;
                        this.registers[t] = ((int)this.getByte(this.registers[s] + SEi)) & 0xFF;
                        break;
                    case 0b100001:
                        //LH
                        s = (opCode >>> 21) & 0B11111;
                        t = (opCode >>> 16) & 0B11111;
                        SEi = (opCode << 16) >> 16;
                        this.registers[t] = this.getHalf(this.registers[s] + SEi);
                        break;
                    case 0b100101:
                        //LHU
                        s = (opCode >>> 21) & 0B11111;
                        t = (opCode >>> 16) & 0B11111;
                        SEi = (opCode << 16) >> 16;
                        this.registers[t] = ((int)this.getHalf(this.registers[s] + SEi)) & 0x0000FFFF;
                        break;
                    case 0b100011:
                        //LW
                        s = (opCode >>> 21) & 0B11111;
                        t = (opCode >>> 16) & 0B11111;
                        SEi = (opCode << 16) >> 16;
                        this.registers[t] = this.memory[(this.registers[s] + SEi) >>> 2];
                        break;

                    case 0b110000:
                        //LL
                        s = (opCode >>> 21) & 0B11111;
                        t = (opCode >>> 16) & 0B11111;
                        SEi = (opCode << 16) >> 16;
                        this.LLVal = 1;
                        this.registers[t] = this.memory[(this.registers[s] + SEi) >>> 2];
                        break;
                    case 0b111000:
                        //SC
                        s = (opCode >>> 21) & 0B11111;
                        t = (opCode >>> 16) & 0B11111;
                        SEi = (opCode << 16) >> 16;
                        if (this.LLVal == 1){
                            this.memory[(this.registers[s] + SEi) >>> 2] = this.registers[t];
                        }
                        this.registers[t] = this.LLVal;
                        break;


                    // store instructions
                    case 0b101000:
                        //SB
                        s = (opCode >>> 21) & 0B11111;
                        t = (opCode >>> 16) & 0B11111;
                        SEi = (opCode << 16) >> 16;
                        this.LLVal = 0;
                        this.setByte(this.registers[s] + SEi, (byte)(this.registers[t]));
                        break;
                    case 0b101001:
                        //SH
                        s = (opCode >>> 21) & 0B11111;
                        t = (opCode >>> 16) & 0B11111;
                        SEi = (opCode << 16) >> 16;
                        this.LLVal = 0;
                        this.setHalf(this.registers[s] + SEi, (short)(this.registers[t]));
                        break;
                    case 0b101011:
                        //SW
                        s = (opCode >>> 21) & 0B11111;
                        t = (opCode >>> 16) & 0B11111;
                        SEi = (opCode << 16) >> 16;
                        this.LLVal = 0;
                        this.memory[(this.registers[s] + SEi) >>> 2] = this.registers[t];
                        break;
                    default:
                        throw new Exception("Invalid Instruction: " + opCode + " at: " + (this.pc - 4));
                }
                this.instructionsRan += 1;
            }
        }catch(Exception e){
            System.err.println("Run time exception: " + e);
            try{
                System.out.printf("0x%08X: 0x%08X%n", this.pc, this.memory[this.pc >> 2]);
            }catch (Exception ignore){
            }
            this.running = false;
            throw e;
        }
    }

    public void setWord(int address, int data){
        this.memory[address >>> 2] = data;
    }

    public void setHalf(int address, short data){
        if ((address & 0b10) == 0) {
            address >>>= 2;
            this.memory[address] &= 0x0000FFFF;
            this.memory[address] |= data << 16;
        }else{
            address >>>= 2;
            this.memory[address] &= 0xFFFF0000;
            this.memory[address] |= ((int)data) & 0xFFFF;
        }
    }

    public void setByte(int address, byte data){
        if ((address & 0b11) == 0) {
            address >>>= 2;
            this.memory[address] &= 0x00FFFFFF;
            this.memory[address] |= (((int)data) & 0xFF) << 24;
        }else if ((address & 0b11) == 1) {
            address >>>= 2;
            this.memory[address] &= 0xFF00FFFF;
            this.memory[address] |= (((int)data) & 0xFF) << 16;
        }else if ((address & 0b11) == 2){
            address >>>= 2;
            this.memory[address] &= 0xFFFF00FF;
            this.memory[address] |= (((int)data) & 0xFF) << 8;
        }else{
            address >>>= 2;
            this.memory[address] &= 0xFFFFFF00;
            this.memory[address] |= ((int)data) & 0xFF;
        }
    }

    public int getWord(int address){
        return this.memory[address >>> 2];
    }

    public short getHalf(int address){
        if ((address & 0b10) == 0){
            return (short)((this.memory[address >>> 2]) >> 16);
        }else{
            return (short)(this.memory[address >>> 2]);
        }
    }

    public byte getByte(int address){
        if ((address & 0b11) == 0){
            return (byte)((this.memory[address >>> 2]) >> 24);
        }else if ((address & 0b11) == 1){
            return (byte)((this.memory[address >>> 2]) >> 16);
        }else if ((address & 0b11) == 2){
            return (byte)((this.memory[address >>> 2]) >> 8);
        }else{
            return (byte)(this.memory[address >>> 2]);
        }
    }

    public interface VirtualInterface{ 
        void system_call(VirtualMachine emu, int call_id);
        void breakpoint(VirtualMachine emu, int call_id);
    }
}
