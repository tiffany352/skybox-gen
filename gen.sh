seed=$RANDOM
echo "Seed: $seed"
for face in up down north south east west; do
    echo "./skybox $1 $face $seed $face.png";
    ./skybox $1 $face $seed $face.png &
done

