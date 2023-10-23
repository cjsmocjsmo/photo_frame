import cv2
import os

def calculate_perceptual_hash(image):
    """Calculates the perceptual hash of an image.

    Args:
        image: A NumPy array representing the image.

    Returns:
        A 64-bit integer representing the perceptual hash of the image.
    """

    image = cv2.resize(image, (8, 8))
    image = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
    image = cv2.dct(image)
    image = image[0:8, 0:8]
    image = image / 255.0
    image = image.flatten()

    image = cv2.phash(image, cv2.HASH_SIZE_64)
    return image

def check_for_duplicate_photos(photo_gallery_directory):
    """Checks for duplicate photos in a photo gallery directory.

    Args:
        photo_gallery_directory: The path to the photo gallery directory.

    Returns:
        A list of lists of duplicate photo paths.
    """

    # Create a hash table to store the perceptual hashes of the images.
    hashes = {}

    # Iterate over the images in the photo gallery directory.
    for image_path in os.listdir(photo_gallery_directory):
        image = cv2.imread(os.path.join(photo_gallery_directory, image_path))
        image = image.astype('float32')

        # Calculate the perceptual hash of the image.
        perceptual_hash = calculate_perceptual_hash(image)

        # Add the perceptual hash to the hash table.
        if perceptual_hash not in hashes:
            hashes[perceptual_hash] = []

        hashes[perceptual_hash].append(image_path)

    # Find duplicate photos.
    duplicate_photos = []
    for perceptual_hash, image_paths in hashes.items():
        if len(image_paths) > 1:
            duplicate_photos.append(image_paths)

    return duplicate_photos

if __name__ == '__main__':
    # Get the path to the photo gallery directory.
    photo_gallery_directory = "/media/pi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Converted/"

    # Check for duplicate photos in the photo gallery directory.
    duplicate_photos = check_for_duplicate_photos(photo_gallery_directory)

    # Display the results to the user.
    if duplicate_photos:
        print('Duplicate photos found:')
        for image_paths in duplicate_photos:
            print('  {}'.format(', '.join(image_paths)))
    else:
        print('No duplicate photos found.')