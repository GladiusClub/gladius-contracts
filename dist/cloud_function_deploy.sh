gcloud functions deploy getStudentBalanceByID --gen2 --runtime nodejs18 \
--trigger-http \
--entry-point=getStudentBalanceByID \
--region=europe-west1 \
--allow-unauthenticated 