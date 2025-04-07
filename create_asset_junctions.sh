#!/bin/bash

cd crates/lumina_shared
ln -s ../../assets assets
cd ../..

cd crates/lumina_client
ln -s ../../assets assets
cd ../..

cd crates/lumina_server
ln -s ../../assets assets
cd ../..

cd crates/lumina_vfx
ln -s ../../assets assets
cd ../..

cd crates/bevy_radiance_cascades
ln -s ../../assets assets
cd ../..

cd crates/test_bed
ln -s ../../assets assets
cd ../..

echo "✅ All symbolic links created."
