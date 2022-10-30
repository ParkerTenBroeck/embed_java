import BasicIO.BasicForm;
import BasicIO.GeneralCanvas;
import Media.Turtle;
import Media.TurtleDisplayer;

import javax.swing.*;
import java.awt.*;
import java.awt.event.KeyEvent;
import java.awt.event.KeyListener;
import java.awt.image.BufferedImage;
import java.io.InputStream;
import java.lang.reflect.Field;
import java.util.ArrayList;
import java.util.Random;

public class Main {


    public static void main(String[] args) throws Exception{
        Turtle yertle = new Turtle();
        TurtleDisplayer display = new TurtleDisplayer(yertle, 720, 480);

        BasicForm basicForm;
        GeneralCanvas canvas;
        BufferedImage image;
        JFrame frame;
        // all this below is reflection madness don't look at it for too long, or you'll lose your mind >:)
        {
            {
                {
                    Field basicFormField = TurtleDisplayer.class.getDeclaredField("bf");
                    basicFormField.setAccessible(true);
                    basicForm = (BasicForm) basicFormField.get(display);
                }
                {
                    Field anglepanelField = Turtle.class.getDeclaredField("canvas");
                    anglepanelField.setAccessible(true);
                    canvas = (GeneralCanvas) anglepanelField.get(yertle);
                }
                {
                    Field imageField = GeneralCanvas.class.getDeclaredField("img");
                    imageField.setAccessible(true);
                    image = (BufferedImage) imageField.get(canvas);
                }
            }
            Object form;

            {
                Field test = basicForm.getClass().getDeclaredField("form");
                test.setAccessible(true);
                form = test.get(basicForm);
            }

            {
                Field test = form.getClass().getDeclaredField("frame");
                test.setAccessible(true);
                frame = (JFrame) test.get(form);
            }
        }

        Graphics g = image.getGraphics();


        basicForm.setTitle("My Window now :)");

        JRootPane root_pane = (JRootPane) frame.getComponent(0);

        KeyRememberer test = new KeyRememberer();
        frame.addKeyListener(test);

        //14, 19, 20
        Random ran = new Random(27);
        recursive_removal_fun(frame, root_pane, 0, ran);

        root_pane.removeAll();
        root_pane.add(canvas);
        Dimension size = new Dimension(image.getWidth(), image.getHeight() + frame.getInsets().top);
        frame.setMinimumSize(size);
        frame.setMaximumSize(size);
        frame.setSize(size);

        g.setColor(Color.WHITE);
        g.fillRect(0,0, image.getWidth(), image.getHeight());

        root_pane.setDoubleBuffered(true);
        frame.pack();
        frame.repaint();
        frame.requestFocus();
        frame.setDefaultCloseOperation(WindowConstants.EXIT_ON_CLOSE);

        frame.setTitle("JRVM");

        runVm(image, frame, test);
    }

    // my ide doesn't like sleep in loops (who would have guessed)
    // maybe that's why its so cranky
    public static void crankySleep(long millis){
        try{
            Thread.sleep(millis);
        }catch (Exception ignore){

        }
    }

    public static byte[] read_bin(){
        try{
            InputStream in = Main.class.getResourceAsStream("bin.bin");
            return in.readAllBytes();
        }catch ( Exception e){
            throw new RuntimeException(e);
        }
    }

    public static void runVm(BufferedImage image, JFrame rootFrame, KeyRememberer keys) throws Exception{
        VirtualMachine vm = new VirtualMachine(); //create vm
        vm.memory = new int[2 << 25]; //allocate memory (256 MiB)
        vm.v_interface = new BrockVirtualInterface(image, rootFrame, keys); //construct interface (syscalls)

        //this program executes 6,442,254,338 instructions and can be used to bench the VM
        //00: lui $2, 0x7FFF
        //04: add $1, $0, $0
        //08: addi $1, $1, 1
        //0C: beq $2, $1, 0x14
        //10: j 0x8
        //14: syscall 0
        //byte[] bytes = new byte[] {0x3c,0x02,0x7F, (byte)0xFF, 0x00, 0x00, 0x08, 0x20, 0x20, 0x21, 0x00, 0x01, 0x10, 0x22, 0x00, 0x01, 0x08, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x0C};

        //load binary into memory
        byte[] bytes = read_bin();
        for(int i = 0; i < bytes.length; i ++){
            vm.setByte(i, bytes[i]);
        }
        while (true){
            long startTime = System.nanoTime();
            vm.run();
            long endTime = System.nanoTime();
            System.out.printf("VM Execution time: %.8fs%n", (endTime - startTime) / 1000000000.0);

            while (true){
                if (keys.isKeyCharPressed('r')){
                    vm.reset();
                    for(int i = 0; i < bytes.length; i ++){
                        vm.setByte(i, bytes[i]);
                    }
                    for(int i = (bytes.length + 3) / 4; i < vm.memory.length; i ++){
                        vm.memory[i] = 0;
                    }
                    break;
                }else if (keys.isKeyCharPressed('e')){
                    System.out.println("Exiting");
                    return;
                }
                crankySleep(1);
            }

        }

    }

    // OooOOoOoOO hacker
    private static void recursive_removal_fun(JFrame main, JComponent root_component, int sleep, Random rand){

        Component[] components = root_component.getComponents();

        for (int i = 0; i < components.length; ++i) {
            int index = rand.nextInt(components.length - i);
            Component tmp = components[components.length - 1 - i];
            components[components.length - 1 - i] = components[index];
            components[index] = tmp;
        }

        for(Component c: components){
            if (c instanceof JComponent){
                JComponent jc = (JComponent) c;
                if (jc.getComponentCount() < 1){
                    root_component.remove(jc);
                }else{
                    recursive_removal_fun(main, jc, sleep, rand);
                    root_component.remove(jc);
                }

                main.repaint();
            }else{
                root_component.remove(c);
            }
            try{
                Thread.sleep(sleep);
            }catch (Exception ignore){

            }
        }
    }

    public static class KeyRememberer /* great name if I do say so */ implements KeyListener{

        private final ArrayList<Character> pressed = new ArrayList<>();

        public boolean isKeyCharPressed(char key){
            return pressed.contains(key);
        }

        @Override
        public void keyTyped(KeyEvent e) {
            //System.out.println(e);
        }

        @Override
        public void keyPressed(KeyEvent e) {
            pressed.remove((Character) e.getKeyChar()); // just to be sure that we are unique + im lazy + ratio
            pressed.add(e.getKeyChar());
        }

        @Override
        public void keyReleased(KeyEvent e) {
            pressed.remove((Character) e.getKeyChar());
        }
    }
}