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

  console.log('-------------------------------------------------------');
  console.log('Testing Gladius Contracts');
  console.log('-------------------------------------------------------');

  console.log(" 💰  Minting 2500,000 EURC to parent")
  // Minting EURC tokens to the gladius admin account
  await mintToken(
    addressBook.getContractId(network, 'token_id'),
    25000000000000,
    parent.publicKey(), // to
    payment_token_admin
  );

  const balanceParentBefore = await getTokenBalance(
    addressBook.getContractId(network, 'token_id'),
    parent.publicKey(),
    parent
  );
  const balanceSportClubBefore = await getTokenBalance(
    addressBook.getContractId(network, 'token_id'),
    sport_club.publicKey(),
    sport_club
  );
  console.log('« EURC balanceParentBefore:', balanceParentBefore);
  console.log("« EURC balanceSportClubBefore:", balanceSportClubBefore)

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
  console.log("   ")
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

  // let price = 100;
  //   let incentive = 10;
  //   let ratio: u32 = 1000;
  //   let total_amount = price + incentive;
  //   let title = String::from_str(&test.env, "Title");

  //   assert_eq!(test.contract.get_total_courses(), 0);
  //   let index = test.contract
  //   .create_course(
  //       &test.club_0, 
  //       &price,
  //       &incentive,
  //       &title
  //   );
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
