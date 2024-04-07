gcloud functions deploy fetchGladiusNFT --gen2 --runtime nodejs18 \
--trigger-http \
--entry-point=fetchGladiusNFT \
--region=europe-west1 \
--allow-unauthenticated 