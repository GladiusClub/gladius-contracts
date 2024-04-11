import { Keypair, Address, nativeToScVal, xdr, scValToNative } from 'stellar-sdk';

import { AddressBook } from '../utils/address_book.js';
import { getTokenBalance, getIsRole, getTotalCourses, invokeContract } from '../utils/contract.js';
import { mintToken } from './mint_token.js';



export async function testGladius(addressBook: AddressBook) {


  let gladius_admin = Keypair.fromSecret('SAOEGQCRNGEDGAZWTFIHLSVB6OWNFOCQKPAKP7NCAYE3ISMVSV34RUYR'); //loadedConfig.admin;
  let payment_token_admin = Keypair.fromSecret('SBJTRWSLGANJMFKZG4PUZMDHFLJHFOZC6OFB3XS25UDMQD337NQZDLAN'); //('PAYMENT_TOKEN_ADMIN_SECRET');
  let sport_club = Keypair.fromSecret('SCOS74PSM3TD7APT2BTTNGASW4EHCPN75UFYDS6KGBDHG24NR6J2XYXQ'); //('SPORT_CLUB_SECRET');
  let parent = Keypair.fromSecret('GAEWNMBYWBYVVK5OVPXAYGUWGKCT57VZIJALQPXMBWNVNEEAVWOK432Z'); //('PARENT_SECRET');
  let student = Keypair.fromSecret('GDC3Y374UZW7KPIGO2OFFMO5Q2XULFWNWKAHWAT2VHBQA7XVK6CP752M'); //('STUDENT_SECRET');

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

      let balanceSportClub = await getTokenBalance(
        addressBook.getContractId(network, 'token_id'),
        sport_club.publicKey(),
        sport_club
      );

      let balanceGLCGladiusSubscriptions = await getTokenBalance(
        addressBook.getContractId(network, 'gladius_emitter_id'), // what token
        addressBook.getContractId(network, 'gladius_subscriptions_id'), // balance of who?
        sport_club
      );
      let balanceGladiusSubscriptions = await getTokenBalance(
        addressBook.getContractId(network, 'token_id'), // what token
        addressBook.getContractId(network, 'gladius_subscriptions_id'), // balance of who?
        sport_club
      );

      console.log("« EURC balance GladiusSubscriptions:", balanceGladiusSubscriptions)
      console.log("« GLC  balance GladiusSubscriptions:", balanceGLCGladiusSubscriptions)
      console.log("« EURC balance SportClub:", balanceSportClub)
      console.log('« EURC balance Parent:', balanceParent);
      console.log("« GLC  balance Student:", balanceGLCStudent)
   
  }

  console.log('-------------------------------------------------------');
  console.log('Testing Gladius Contracts');
  console.log('-------------------------------------------------------');

  console.log(" 💰  Minting 100 EURC to parent")
  // Minting EURC tokens to the gladius admin account
  await mintToken(
    addressBook.getContractId(network, 'token_id'),
    100,
    parent.publicKey(), // to
    payment_token_admin
  );

  await getAllBalances();

  console.log("  ")
  console.log("  ")

  console.log("  🕵️  | Checking and Setting Roles")
  

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

  console.log("   ")
  console.log("   ")

  console.log("  📝  | Checking and Subscribing to Courses")

//  const courseReponse = await invokeContract('gladius_subscriptions_id', addressBook, 'create_course', createCourseParams, sport_club);
  //const courseIndex = scValToNative(courseReponse.returnValue);
  const courseIndex = 0;

  console.log("~ testGladius ~ courseIndex:", courseIndex)
  
  const totalCourses = await getTotalCourses(
    addressBook.getContractId(network, 'gladius_subscriptions_id'),
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

const network ='testnet';
const folder = 'public';
let addressBook: AddressBook;

if (folder == 'public') {
    addressBook = AddressBook.loadFromFile(network, folder);
} else {
    addressBook = AddressBook.loadFromFile(network);
}


await testGladius(addressBook);
