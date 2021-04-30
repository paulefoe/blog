from PIL import Image, ImageDraw, ImageFont
import os
 
def text_on_img(filename='01.png', text="Hello", size=12, color=(255,255,0), bg='white'):
    font = ImageFont.truetype('/home/paulefou/other_projects/rubish/source-code-pro/OTF/SourceCodePro-Regular.otf', size=30)
    image = Image.new(mode="RGB", size=(1200, 1200), color=bg)
    draw = ImageDraw.Draw(image)
    w, h = draw.textsize(text, font=font)
    h += int(h * 0.21)
    draw.text(((1200 - w) / 2, (1200 - h) / 2), text=text, fill='black', font=font)

    # draw.text((10,10), text, font=font, fill=(255,255,0))
    image.save(filename)
    os.system(filename)
 
 
text_on_img(text="Fighting bad internet habits with ancient long forgotten weapons", size=300, bg='white')