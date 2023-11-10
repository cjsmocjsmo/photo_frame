import gi
gi.require_version("Gtk", "3.0")
from gi.repository import Gtk, Gdk
from gi.repository import GdkPixbuf
# Create a window
window = Gtk.Window(title="Display Image")
window.set_default_size(1360, 800)
window.override_background_color(Gtk.StateFlags.NORMAL, Gdk.RGBA(0, 0, 0, 1))
# window.fullscreen()


pixbuf = GdkPixbuf.Pixbuf.new_from_file("/home/charliepi/test2.png")

# Set the maximum image size
max_width = 800
max_height = 400
pixbuf = pixbuf.scale_simple(max_width, max_height, GdkPixbuf.InterpType.BILINEAR)
# Create an image widget
# image = Gtk.Image()

# # Load the image
# image.set_from_file("/home/charliepi/test2.png")

# Add the image widget to the window

image = Gtk.Image()
image.set_from_pixbuf(pixbuf)

window.add(image)

# Show the window
window.show_all()

# Start the main loop
Gtk.main()