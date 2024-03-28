gcloud functions deploy GladiusContracts --gen2 --runtime nodejs20 \
--trigger-http \
--entry-point=GladiusContracts \
--region=europe-west1 \
--allow-unauthenticated 