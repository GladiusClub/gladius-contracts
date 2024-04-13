import { db } from './scripts/firebaseAdminSetup.js';
import { Address, nativeToScVal, xdr, scValToNative } from 'stellar-sdk';
import { AddressBook } from './utils/address_book.js';
import { getTokenBalance, getIsRole, getTotalCourses, invokeContract } from './utils/contract.js';
import { api_config } from './utils/api_config.js';
import { config } from './utils/env_config.js';
import { mintToken } from './scripts/mint_token.js';
import { group } from 'console';


const network = process.argv[2] || 'testnet';     
const folder = 'public'; 

let addressBook: AddressBook;
const loadedConfig = config(network);

if (folder == 'public') {
    addressBook = AddressBook.loadFromFile(network, folder);
} else {
    addressBook = AddressBook.loadFromFile(network);
}

export async function SignupGladius(
  addressBook: AddressBook, 
  parent_stellar_secret: string, 
  student_stellar_secret: string, 
  club_stellar_secret: string, 
  gladius_subscriptions_id: string, 
  gladius_course_index: string) {

  let gladius_admin = loadedConfig.admin;
  let payment_token_admin = loadedConfig.getUser('PAYMENT_TOKEN_ADMIN_SECRET');
 
  const sport_club = api_config(network, club_stellar_secret);
  const parent = api_config(network, parent_stellar_secret);
  const student = api_config(network, student_stellar_secret);

  async function getAllBalances() {

    let balanceParent = await getTokenBalance(
    addressBook.getContractId(network, 'token_id'),
    parent.publicKey(),
    parent
    );
	
    let balanceGLCStudent = await getTokenBalance(
      addressBook.getContractId(network, 'gladius_emitter_id'), // what token
      student.publicKey(), // balance of who?
      sport_club
    );

    let balanceCoinEmitter = await getTokenBalance(
      addressBook.getContractId(network, 'token_id'), // what token
      addressBook.getContractId(network, 'gladius_emitter_id'), // balance of who?
      sport_club
    );

    let balanceGladiusSubscriptions = await getTokenBalance(
      addressBook.getContractId(network, 'token_id'), // what token
      gladius_subscriptions_id, //addressBook.getContractId(network, 'gladius_subscriptions_id'), // balance of who?
      sport_club
    );

      let balanceSportClub = await getTokenBalance(
        addressBook.getContractId(network, 'token_id'),
        sport_club.publicKey(),
        sport_club
      );

      let balanceGLCSportClub = await getTokenBalance(
        addressBook.getContractId(network, 'gladius_emitter_id'), // what token
        sport_club.publicKey(), // balance of who?
        sport_club
      );


   console.log('« EURC balance Parent:', balanceParent);
   console.log("« EURC balance GladiusSubscriptions:", balanceGladiusSubscriptions)
   console.log("« EURC balance CoinEmitter:", balanceCoinEmitter)
    console.log("« EURC balance SportClub:", balanceSportClub)
    console.log("« GLC  balance SportClub:", balanceGLCSportClub)
    console.log("« GLC  balance Student:", balanceGLCStudent)

  }

  await getAllBalances();

  console.log('-------------------------------------------------------');
  console.log('Testing Gladius Contracts');
  console.log('-------------------------------------------------------');

  console.log(" 💰  Minting 200 EURC to parent")
  // Minting EURC tokens to the gladius admin account
  await mintToken(
    addressBook.getContractId(network, 'token_id'),
    200,
    parent.publicKey(), // to
    payment_token_admin
  );

  await getAllBalances();


  console.log("  🕵️  | Checking and Setting Roles")
  

  const isParentBefore = await getIsRole(
    gladius_subscriptions_id, //addressBook.getContractId(network, 'gladius_subscriptions_id'),
    'is_parent',
    parent.publicKey(),
    parent
    )
  const isStudentBefore = await getIsRole(
    gladius_subscriptions_id, //addressBook.getContractId(network, 'gladius_subscriptions_id'),
    'is_student',
    student.publicKey(),
    student
    )
  console.log("~ testGladius ~ isStudentBefore:", isStudentBefore)
  console.log("~ testGladius ~ isParentBefore:", isParentBefore)
  console.log("   ")
  
  
  const setIsParentParams: xdr.ScVal[] = [
    new Address(parent.publicKey()).toScVal(), // parent
    nativeToScVal(true, { type: 'bool' }), // is
  ];
  await invokeContract('gladius_subscriptions_id', addressBook, 'set_is_parent', setIsParentParams, gladius_admin);

  const setIsStudentParams: xdr.ScVal[] = [
    new Address(student.publicKey()).toScVal(), // student
    nativeToScVal(true, { type: 'bool' }), // is
  ];
  await invokeContract('gladius_subscriptions_id', addressBook, 'set_is_student', setIsStudentParams, gladius_admin);

  
  const isParentAfter = await getIsRole(
    gladius_subscriptions_id,// addressBook.getContractId(network, 'gladius_subscriptions_id'),
    'is_parent',
    parent.publicKey(),
    parent
    )
  const isStudentAfter = await getIsRole(
    gladius_subscriptions_id, // addressBook.getContractId(network, 'gladius_subscriptions_id'),
    'is_student',
    student.publicKey(),
    student
    )
  console.log("   ")
  console.log("~ testGladius ~ isStudentAfter:", isStudentAfter)
  console.log("~ testGladius ~ isParentAfter:", isParentAfter)

  console.log("   ")
  console.log("   ")

  console.log("  📝  | Checking and Subscribing to Courses")

//  const courseReponse = await invokeContract('gladius_subscriptions_id', addressBook, 'create_course', createCourseParams, sport_club);
  //const courseIndex = scValToNative(courseReponse.returnValue);
  const courseIndex = Number(gladius_course_index);

  console.log("~ testGladius ~ courseIndex:", courseIndex)
  
  const totalCourses = await getTotalCourses(
    gladius_subscriptions_id, //addressBook.getContractId(network, 'gladius_subscriptions_id'),
    gladius_admin
    );
  console.log(" ~ testGladius ~ totalCoursesAfter:", totalCourses)


  const getCourseParams: xdr.ScVal[] = [
    nativeToScVal(courseIndex, { type: 'u32' }), // index
  ];
  const gotCourse = await invokeContract('gladius_subscriptions_id', addressBook, 'get_course', getCourseParams, sport_club);
  const gotCourseNative = scValToNative(gotCourse.returnValue);
  console.log(" ~ testGladius ~ gotCourseNative:", gotCourseNative)
  

  console.log("   ")
  console.log("   ")


  console.log("  🎾  | Subscribing to Courses")

  await getAllBalances();

  const subscribeCourseParams: xdr.ScVal[] = [
    new Address(parent.publicKey()).toScVal(), // parent
    new Address(student.publicKey()).toScVal(), // student
    nativeToScVal(courseIndex, { type: 'u32' }), // course_index
  ];
  
  await invokeContract('gladius_subscriptions_id', addressBook, 'subscribe_course', subscribeCourseParams, parent);
  
  await getAllBalances();

}

console.log("Connecting to firebase");
  const ParentUID = 'gLdqODKRsxc2AznOyzw13WxzJPD2';
  const StudentUID = 'mf3fRzXNCJOqW4ltGQ5YMo6MLvu2';
  const ClubUID = '2';
  //const GroupUID = 'VnFvQxDpTL9LuvhiOSTt';
  const GroupUID = 'BH4gTmMRgOUYZLcTQupu';
  
  const StudentdocRef = db.collection('users').doc(StudentUID);
  const ParentdocRef = db.collection('users').doc(ParentUID);

  const clubRef = db.collection('clubs').doc(ClubUID);
  
  const StudentdocSnap = await StudentdocRef.get();
  const ParentdocSnap = await ParentdocRef.get();
  const clubSnap = await clubRef.get();
  
  if (ParentdocSnap.exists) { 
    const ParentData = ParentdocSnap.data();
    if (ParentData && ParentData.stellar_wallet && ParentData.email) { // Check if userData is truthy before accessing its properties
      console.log(`Parent doc with ID ${ParentUID} was found. User email: `, ParentData.email);
      
      const parent_stellar_wallet = ParentData.stellar_wallet
      const parent_stellar_secret = ParentData.stellar_secret
      console.log("Parent public key:", parent_stellar_wallet);

  if (StudentdocSnap.exists) { 
    const StudentData = StudentdocSnap.data();
    if (StudentData && StudentData.stellar_wallet && StudentData.email) 
      { // Check if userData is truthy before accessing its properties
      console.log(`Student doc with ID ${StudentUID} was found. User email: `, StudentData.email);
      
      const student_stellar_wallet = StudentData.stellar_wallet
      const student_stellar_secret = StudentData.stellar_secret
      console.log("Student public key:", student_stellar_wallet);

      if (clubSnap.exists) {
        const clubData = clubSnap.data();
        if (clubData && clubData.name && clubData.club_stellar_secret && clubData.club_stellar_wallet) 
          {
          console.log(`Club ID ${ClubUID} was found. It's ${clubData.name} `);
          
          const club_stellar_wallet = clubData.club_stellar_wallet;
          const club_stellar_secret = clubData.club_stellar_secret;
          console.log(`Club wallet ${club_stellar_wallet} `);

          const groupRef = clubRef.collection('groups').doc(GroupUID);
          const groupSnap = await groupRef.get();
          
          if (groupSnap.exists) 
            {
            const groupData = groupSnap.data();
            if (groupData ) {
              console.log(`Club ID ${ClubUID}. Group name is ${groupData.name}. Course title is ${groupData.gladius_course_title}`);
              if (groupData.gladius_subscriptions_id && groupData.gladius_course_index) {
                const gladius_subscriptions_id = groupData.gladius_subscriptions_id;
                const gladius_course_index = groupData.gladius_course_index;
                console.log('gladius_subscriptions_id: ', gladius_subscriptions_id);
                console.log('gladius_course_index: ', gladius_course_index); //default 0

                await SignupGladius(addressBook,  parent_stellar_secret ,student_stellar_secret, club_stellar_secret, gladius_subscriptions_id, gladius_course_index);

              } else {
                console.log(`Gladius smart contract data for ID ${GroupUID} is undefined`);
              }
            } else {
                console.log(`Group data for ID ${GroupUID} is undefined`);
            }
          }
        
        }
        else {
          console.log(`club with ID ${ClubUID} not found `);
        } 
    }
  } else {
    console.log(`No Student document with id ${StudentUID} `);  
  }
  
 }
  
  } else {
    console.log(`No Parent document with id ${StudentUID} `);  
  }

}
