# Raytracing

## Programming
As I have extended 3b from the first part, I have again implemented this
project using rust

in order to build and run the app all you need 
to do is to navigate to ray_tracing project 
in the terminal and run the following script

```cargo run``` <br>

a window will open where you can look for the example xml files in ```ray_tracing\assets\scenes``` <br>
just select one of the examples and wait for the app to finish 
rendering.
you can find the result as a png file in the following path

```ray_tracing\output``` <br>


## Claim

I have tried to implement, T1, T2, T3, T4, T5 and T6.

In my own opinion I have achieved real good results in T1, T2, T3 and T5.
Unfortunately though I can not say the same for T4 and T6 which are the texture mapping tasks. 
Just so I let you know I managed to import, deserialize and assign the texture
files to both Spheres and meshes, you can check the respective structs, but I
have failed with algorithms of actually mapping them to the objects with barycentrics
so although example 6 and 7 run if you wait long enough for the render to finish,
the resulted pictures do not show the textured materials.

Also for T5 which is camera transformations I added custom_scene_camera_transformation.xml
as test example, which is basically the example7.xml with textured materials removed.

you can find the structs used for this tasks in model directory, and
the services such as xml and obj parsing and render in services directory

## Tested environment

I used RustRover on my Windows pc for development  

## Additional Remarks
Nothing Special, 
Thank you for your time and support through the semester.
Wish all the best.