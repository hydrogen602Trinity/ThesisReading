# Simple pygame program

# Import and initialize the pygame library
from time import sleep
import pygame
pygame.init()

# Set up the drawing window
screen = pygame.display.set_mode([500, 500])


last_data = ''
def file_reader():
    global running, last_data
    with open('pipe', buffering=1) as f:
        while True:
            sleep(0.0001)
            last_data = f.readline()
            if last_data == 'END' or last_data == '':
                break

    running = False

import threading
th = threading.Thread(target=file_reader)
th.start()

# Run until the user asks to quit
running = True
while running:

    # Did the user click the window close button?
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False

    # Fill the background with white
    screen.fill((255, 255, 255))

    if not last_data:
        continue

    import itertools
    
    colors = [(0, 0, 255), (255, 0, 0), (0, 255, 0), (255, 255, 0)]
    for c, particle in zip(itertools.repeat((0, 0, 255)), last_data.split('|')):
        x, y, z = (float(n)*5 for n in particle.split(','))

        # Draw a solid blue circle in the center
        pygame.draw.circle(screen, c, (x + 250, 250 - y), 5)

    # Flip the display
    pygame.display.flip()

# Done! Time to quit.
pygame.quit()
th.join()
