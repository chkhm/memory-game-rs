from PIL import Image

from PIL import ImageFont
from PIL import ImageDraw

for i in range(33):
    img = Image.new('RGB', (500,707), (250,20,150))
    draw = ImageDraw.Draw(img)
    font = ImageFont.truetype("LeckerliOne-Regular.otf", 380)
    if i < 10:
        txt = "0"+str(i)
    else:
        txt = str(i)
    draw.text((10, 70),txt,(40,180,180),font=font)
    if i < 10:
        img.save('card_0'+str(i)+'.png')
    else:
        img.save('card_'+str(i)+'.png')
    