// The time in milliseconds between timer ticks
#define TIMERMSECS 33

// Global variables for measuring time (in milli-seconds)
static double startTime;
static double prevTime;

static animated_parameter_t<GLfloat>
    xrot(15.0f, 15.0f, -15.0f, 15.0f, animated_parameter_t<GLfloat>::PingPongLoop),
    yrot(0.0, 45.0f),
    fov(45.0f, 30.0f, 45.f, 90.f, animated_parameter_t<GLfloat>::PingPongLoop);

static void animate(int /*value*/)
{
    std::this_thread::sleep_for(std::chrono::milliseconds(TIMERMSECS));

    // Measure the elapsed time
    double currTime = glfwGetTime();
    double timeSincePrevFrame = currTime - prevTime;

    std::cout << "Elapsed time " << (timeSincePrevFrame*1000) << std::endl;

    // Rotate the model
    yrot.animate(timeSincePrevFrame*1000);
    xrot.animate(timeSincePrevFrame*1000);
    fov.animate(timeSincePrevFrame*1000);

    prevTime = currTime;
}

static void init(GLsizei w, GLsizei h)
{
    glEnable(GL_LIGHTING);
    glEnable(GL_LIGHT0);

    glLightfv(GL_LIGHT0, GL_POSITION, LightPos);
    glLightfv(GL_LIGHT0, GL_AMBIENT,  Ambient);

    glShadeModel(GL_SMOOTH); // enable Gouraud
}
