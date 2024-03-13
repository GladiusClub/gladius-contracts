import { Address, nativeToScVal, xdr, scValToNative } from 'stellar-sdk';
import { AddressBook } from '../utils/address_book.js';

import { getTokenBalance, getIsRole, getTotalCourses, invokeContract } from '../utils/contract.js';
import { config } from '../utils/env_config.js';
import { mintToken } from './mint_token.js';



export async function testGladius(addressBook: AddressBook) {


  let gladius_admin = loadedConfig.admin;
  let payment_token_admin = loadedConfig.getUser('PAYMENT_TOKEN_ADMIN_SECRET');
  let sport_club = loadedConfig.getUser('SPORT_CLUB_SECRET');
  let parent = loadedConfig.getUser('PARENT_SECRET');
  let student = loadedConfig.getUser('STUDENT_SECRET');

  async function getAllBalances() {
      let balanceParent = await getTokenBalance(
        addressBook.getContractId(network, 'token_id'),
        parent.publicKey(),
        parent
      );
      let balanceSportClub = await getTokenBalance(
        addressBook.getContractId(network, 'token_id'),
        sport_club.publicKey(),
        sport_club
      );
      let balanceCoinEmitter = await getTokenBalance(
        addressBook.getContractId(network, 'token_id'), // what token
        addressBook.getContractId(network, 'gladius_emitter_id'), // balance of who?
        sport_club
      );
      let balanceGladiusSubscriptions = await getTokenBalance(
        addressBook.getContractId(network, 'token_id'), // what token
        addressBook.getContractId(network, 'gladius_subscriptions_id'), // balance of who?
        sport_club
      );
      let balanceGLCGladiusSubscriptions = await getTokenBalance(
        addressBook.getContractId(network, 'gladius_emitter_id'), // what token
        addressBook.getContractId(network, 'gladius_subscriptions_id'), // balance of who?
        sport_club
      );
      let balanceGLCStudent = await getTokenBalance(
        addressBook.getContractId(network, 'gladius_emitter_id'), // what token
        student.publicKey(), // balance of who?
        sport_club
      );
      let balanceGLCSportClub = await getTokenBalance(
        addressBook.getContractId(network, 'gladius_emitter_id'), // what token
        sport_club.publicKey(), // balance of who?
        sport_club
      );

      console.log("« EURC balance GladiusSubscriptions:", balanceGladiusSubscriptions)
      console.log("« EURC balance CoinEmitter:", balanceCoinEmitter)
      console.log('« EURC balance Parent:', balanceParent);
      console.log("« EURC balance SportClub:", balanceSportClub)
      console.log("« GLC  balance GladiusSubscriptions:", balanceGLCGladiusSubscriptions)
      console.log("« GLC  balance Student:", balanceGLCStudent)
      console.log("« GLC  balance SportClub:", balanceGLCSportClub)

  }

  console.log('-------------------------------------------------------');
  console.log('Testing Gladius Contracts');
  console.log('-------------------------------------------------------');

  console.log(" 💰  Minting 5000 EURC to parent")
  // Minting EURC tokens to the gladius admin account
  await mintToken(
    addressBook.getContractId(network, 'token_id'),
    5000,
    parent.publicKey(), // to
    payment_token_admin
  );

  await getAllBalances();

  console.log("  ")
  console.log("  ")

  console.log("  🕵️  | Checking and Setting Roles")
  
  const isSportClubBefore = await getIsRole(
    addressBook.getContractId(network, 'gladius_subscriptions_id'),
    'is_sport_club',
    sport_club.publicKey(),
    sport_club
    );
  const isParentBefore = await getIsRole(
    addressBook.getContractId(network, 'gladius_subscriptions_id'),
    'is_parent',
    parent.publicKey(),
    parent
    )
  const isStudentBefore = await getIsRole(
    addressBook.getContractId(network, 'gladius_subscriptions_id'),
    'is_student',
    student.publicKey(),
    student
    )
  console.log("~ testGladius ~ isStudentBefore:", isStudentBefore)
  console.log("~ testGladius ~ isParentBefore:", isParentBefore)
  console.log("~ testGladius ~ isSportClubBefore:", isSportClubBefore)
  console.log("   ")

  const setIsSportClubParams: xdr.ScVal[] = [
    new Address(sport_club.publicKey()).toScVal(), // sport_club
    nativeToScVal(true, { type: 'bool' }), // is
  ];
  await invokeContract('gladius_subscriptions_id', addressBook, 'set_is_sport_club', setIsSportClubParams, gladius_admin);

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

  const isSportClubAfter = await getIsRole(
    addressBook.getContractId(network, 'gladius_subscriptions_id'),
    'is_sport_club',
    sport_club.publicKey(),
    sport_club
    );
  const isParentAfter = await getIsRole(
    addressBook.getContractId(network, 'gladius_subscriptions_id'),
    'is_parent',
    parent.publicKey(),
    parent
    )
  const isStudentAfter = await getIsRole(
    addressBook.getContractId(network, 'gladius_subscriptions_id'),
    'is_student',
    student.publicKey(),
    student
    )
  console.log("   ")
  console.log("~ testGladius ~ isStudentAfter:", isStudentAfter)
  console.log("~ testGladius ~ isParentAfter:", isParentAfter)
  console.log("~ testGladius ~ isSportClubAfter:", isSportClubAfter)

  console.log("   ")
  console.log("   ")

  console.log("  📝  | Checking and Creating Courses")

  const totalCoursesBefore = await getTotalCourses(
    addressBook.getContractId(network, 'gladius_subscriptions_id'),
    gladius_admin
    );
  console.log(" ~ testGladius ~ totalCoursesBefore:", totalCoursesBefore)

  const createCourseParams: xdr.ScVal[] = [
    new Address(sport_club.publicKey()).toScVal(), // sport_club
    nativeToScVal(100, { type: 'i128' }), // price
    nativeToScVal(10, { type: 'i128' }), // incentive
    nativeToScVal('My Course', { type: 'string' }), // title
  ];
  const courseReponse = await invokeContract('gladius_subscriptions_id', addressBook, 'create_course', createCourseParams, sport_club);
  const courseIndex = scValToNative(courseReponse.returnValue);
  console.log("~ testGladius ~ courseIndex:", courseIndex)
  
  const totalCoursesAfter = await getTotalCourses(
    addressBook.getContractId(network, 'gladius_subscriptions_id'),
    gladius_admin
    );
  console.log(" ~ testGladius ~ totalCoursesAfter:", totalCoursesAfter)


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


  console.log("   ")
  console.log("   ")

  console.log("  🏆  | Gladius Coins Distribution")
  console.log("      | Sport Club will distribute 1500 units of GLC to student")

  const distributeParams: xdr.ScVal[] = [
    nativeToScVal(courseIndex, { type: 'u32' }), // course_index
    new Address(student.publicKey()).toScVal(), // student
    nativeToScVal(1500, { type: 'i128' }), // amount
  ];
  await invokeContract('gladius_subscriptions_id', addressBook, 'distribute_gladius_coins', distributeParams, sport_club);
  
  await getAllBalances();




  console.log("   ")
  console.log("   ")

  console.log("  ↔️   | Gladius Coins Transaction")
  console.log("      | Student Sends 1000 GLC to Sport Club (bought NFT?)")

  const transactionParams: xdr.ScVal[] = [
    new Address(student.publicKey()).toScVal(), // from
    new Address(sport_club.publicKey()).toScVal(), // to
    nativeToScVal(1000, { type: 'i128' }), // amount
  ];
  await invokeContract('gladius_emitter_id', addressBook, 'transfer', transactionParams, student);
  
  await getAllBalances();

  console.log("   ")
  console.log("   ")

  console.log(" 💱   | Unwrap and Burn")
  console.log("      | SportClub unwraps 1 EURC and burn 1000 GLC ")

  const unwrapParams: xdr.ScVal[] = [
    new Address(sport_club.publicKey()).toScVal(), // from
    nativeToScVal(1, { type: 'i128' }), // unwrap_amount
  ];
  await invokeContract('gladius_emitter_id', addressBook, 'unwrap_and_burn', unwrapParams, sport_club);

  await getAllBalances();



// // TODO: Test with auth
// test.contract.distribute_gladius_coins(
//   &index, // index
//   &test.student_0, // student: Address,
//   &distribute_amount_gladius_coins, // amount

// );

  // // Example of transfering a token with the token's transfer function from the admins account to the pegged token admin account
  // // ONLY EXAMPLE SHOULD BE REMOVED AFTER
  // console.log('-------------------------------------------------------');
  // console.log('Example of executing a method of an smart contract');
  // console.log('Making a transfer of EURC Token from gladius admin to pegged token admin');
  // console.log('-------------------------------------------------------');
  // const balanceTokenAdminBefore = await getTokenBalance(
  
  //   addressBook.getContractId(network, 'token_id'),
  //   payment_token_admin.publicKey(),
  //   payment_token_admin
  // );
  // console.log('🚀 « EURC balanceTokenAdminBefore:', balanceTokenAdminBefore);

  // const transferInitParams: xdr.ScVal[] = [
  //   new Address(gladius_admin.publicKey()).toScVal(),
  //   new Address(payment_token_admin.publicKey()).toScVal(),
  //   nativeToScVal(1000000000, { type: 'i128' }),
  // ];
  // await invokeContract('token_id', addressBook, 'transfer', transferInitParams, gladius_admin);
  // // Balances after transfering
  // const balanceAdminAfter = await getTokenBalance(
  //   addressBook.getContractId(network, 'token_id'),
  //   gladius_admin.publicKey(),
  //   gladius_admin
  // );
  // console.log('🚀 « EURC balanceAdminAfter:', balanceAdminAfter);
  // const balanceTokenAdminAfter = await getTokenBalance(
  //   addressBook.getContractId(network, 'token_id'),
  //   payment_token_admin.publicKey(),
  //   payment_token_admin
  // );
  // console.log('🚀 « EURC balanceTokenAdminAfter:', balanceTokenAdminAfter);
  // // END OF EXAMPLE
}

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile();

const loadedConfig = config(network);

await testGladius(addressBook);
