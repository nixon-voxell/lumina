cd crates\lumina_shared
mklink /J assets\ ..\..\assets\
cd ../..

cd crates\lumina_client
mklink /J assets\ ..\..\assets\
cd ../..

cd crates\lumina_server
mklink /J assets\ ..\..\assets\
cd ../..

cd crates\lumina_vfx
mklink /J assets\ ..\..\assets\
cd ../..

cd crates\bevy_radiance_cascades
mklink /J assets\ ..\..\assets\
cd ../..

cd crates\bevy_post_process
mklink /J assets\ ..\..\assets\
cd ../..

echo "✅ All symbolic links created."
